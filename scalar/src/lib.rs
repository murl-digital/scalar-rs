use internals::ts::AnythingElse;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub use scalar_derive::{doc_enum, Document, EditorField, Enum};
use serde::{de, Deserialize, Serialize};
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
use validations::{DataModel, ValidationError, ValidatorFunction};

#[derive(Serialize, Deserialize)]
pub struct MultiLine(String);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Markdown(String);

// impl<'de> Deserialize<'de> for Markdown {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let result = String::deserialize(deserializer)?;

//         if result.is_empty() {
//             Err(<D::Error as de::Error>::invalid_value(
//                 de::Unexpected::Str(&result),
//                 &"a non-empty string",
//             ))
//         } else {
//             Ok(Self(result))
//         }
//     }
// }

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

pub trait Document: Serialize + for<'de> Deserialize<'de> {
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

    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, concrete(D = String))]
pub struct Item<D: Debug> {
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
