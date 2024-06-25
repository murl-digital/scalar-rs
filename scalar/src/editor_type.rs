use std::rc::Rc;

use serde::Serialize;
use ts_rs::TS;

use crate::EditorField;

#[derive(Serialize, TS)]
#[serde(tag = "type")]
pub enum EditorType {
    Bool { default: Option<bool> },
    Integer { default: Option<i32> },
    Float { default: Option<f32> },
    Enum { variants: Vec<EnumVariant> },
    Array { of: Rc<EditorType> },
    SingleLine,
    MultiLine,
    Markdown,
    Date,
    DateTime,
}

#[derive(Serialize, TS)]
pub struct EnumVariant {
    pub variant_name: &'static str,
    pub fields: Option<Vec<EditorField>>,
}
