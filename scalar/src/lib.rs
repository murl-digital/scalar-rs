pub use scalar_derive::Document;
use serde::{Deserialize, Serialize};

pub mod editor_field;

#[derive(Serialize)]
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

#[derive(Serialize)]
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

pub trait Document {
    fn identifier() -> &'static str;
    fn title() -> &'static str;

    fn schema() -> Vec<EditorField>;
}
