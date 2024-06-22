pub use scalar_derive::Document;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub use chrono::{DateTime, Utc};
pub use nanoid::nanoid;

pub use db::DB;

pub mod editor_field;
pub mod db;



#[derive(Serialize, TS)]
#[serde(tag = "type")]
pub enum EditorType {
    Bool { default: Option<bool> },
    Integer { default: Option<i32> },
    Float { default: Option<f32> },
    SingleLine,
    MultiLine,
    Markdown,
    Date,
    DateTime,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct EditorField {
    name: &'static str,
    title: &'static str,
    placeholder: Option<&'static str>,
    field_type: EditorType,
}

#[derive(Serialize, Deserialize)]
pub struct MultiLine(String);
#[derive(Serialize, Deserialize)]
pub struct Markdown(String);

#[derive(Serialize, TS)]
#[ts(export)]
pub struct Schema {
    identifier: &'static str,
    title: &'static str,
    fields: Vec<EditorField>
}

pub trait Document {
    fn identifier() -> &'static str;
    fn title() -> &'static str;

    fn fields() -> Vec<EditorField>;
    fn schema() -> Schema {
        Schema { identifier: Self::identifier(), title: Self::title(), fields: Self::fields() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item<D: Document> {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub inner: D
}
