use std::collections::HashMap;

pub use scalar_derive::{doc_enum, Document, Enum};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub use chrono::{DateTime, Utc};
pub use nanoid::nanoid;

pub use db::DB;

pub mod db;
pub mod editor_field;
pub mod editor_type;
pub mod validations;

pub use editor_field::EditorField;
pub use editor_type::EditorType;
use validations::{DataModel, ValidatorFunction};



#[derive(Serialize, Deserialize)]
pub struct MultiLine(String);
#[derive(Serialize, Deserialize)]
pub struct Markdown(String);

#[derive(Serialize, TS)]
#[ts(export)]
pub struct Schema {
    identifier: &'static str,
    title: &'static str,
    fields: Vec<EditorField>,
}

pub trait Document {
    fn identifier() -> &'static str;
    fn title() -> &'static str;

    fn fields() -> Vec<EditorField>;
    fn validators(model: DataModel) -> HashMap<String, ValidatorFunction>;
    fn schema() -> Schema {
        Schema {
            identifier: Self::identifier(),
            title: Self::title(),
            fields: Self::fields(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item<D: Document> {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub inner: D,
}