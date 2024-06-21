pub use scalar_derive::Document;

pub mod editor_field;

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

pub struct EditorField {
    name: &'static str,
    placeholder: Option<&'static str>,
    field_type: EditorType,
}

pub struct MultiLine(String);
pub struct Markdown(String);

pub trait Document {
    fn identifier() -> &'static str;
    fn title() -> &'static str;

    fn editor() -> Vec<EditorField>;
}
