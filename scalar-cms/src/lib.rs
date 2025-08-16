use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub use scalar_derive::{doc_enum, Document, EditorField, Enum};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub use chrono::{DateTime, Utc};
pub use nanoid::nanoid;

pub use db::DatabaseConnection;

pub mod db;
pub mod editor_field;
pub mod editor_type;
pub mod internals;
pub mod validations;

pub use serde_json::Value;

pub fn convert<T: Serialize>(value: T) -> Value {
    serde_json::to_value(value).expect("this should never fail")
}

pub use editor_field::EditorField;
pub use editor_type::EditorType;
use validations::Validate;

#[derive(Serialize, Deserialize)]
pub struct MultiLine(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Markdown(String);

impl Deref for MultiLine {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MultiLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Markdown {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Markdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Serialize, TS)]
pub struct Schema {
    identifier: &'static str,
    title: &'static str,
    fields: Vec<EditorField>,
}

#[derive(Serialize, TS)]
pub struct DocInfo {
    pub identifier: &'static str,
    pub title: &'static str,
}

pub trait Document: Validate {
    fn identifier() -> &'static str;
    fn title() -> &'static str;

    fn fields() -> Vec<EditorField>;
    fn schema() -> Schema {
        Schema {
            identifier: Self::identifier(),
            title: Self::title(),
            fields: Self::fields(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, concrete(D = String))]
pub struct Item<D> {
    #[serde(rename = "__sc_id")]
    pub id: String,
    #[serde(rename = "__sc_created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "__sc_modified_at")]
    pub modified_at: DateTime<Utc>,
    #[serde(rename = "__sc_published_at")]
    pub published_at: Option<DateTime<Utc>>,
    #[serde(rename = "content")]
    #[ts(type = "any")]
    pub inner: D,
}

impl<D: Document> Validate for Item<D> {
    fn validate(&self) -> Result<(), validations::ValidationError> {
        self.inner.validate()
    }
}
