use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde::Serialize;
use ts_rs::TS;

use crate::EditorField;

#[derive(Serialize, TS)]
#[serde(tag = "type")]
pub enum EditorType {
    Bool {
        default: Option<bool>,
    },
    Integer {
        default: Option<i32>,
    },
    Float {
        default: Option<f32>,
    },
    Struct {
        #[ts(type = "any | null")]
        default: Option<serde_json::Value>,
        fields: Vec<EditorField>,
    },
    Enum {
        #[ts(type = "any | null")]
        default: Option<serde_json::Value>,
        variants: Vec<EnumVariant>,
    },
    Array {
        #[ts(type = "any[] | null")]
        default: Option<serde_json::Value>,
        of: Rc<EditorType>,
    },
    SingleLine {
        default: Option<String>,
    },
    MultiLine {
        default: Option<String>,
    },
    Markdown {
        default: Option<String>,
    },
    Date {
        default: Option<DateTime<Utc>>,
    },
    DateTime {
        default: Option<DateTime<Utc>>,
    },
}

#[derive(Serialize, TS)]
pub struct EnumVariant {
    pub variant_name: &'static str,
    pub fields: Option<Vec<EditorField>>,
}
