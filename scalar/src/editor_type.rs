use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde::Serialize;
use ts_rs::TS;

use crate::EditorField;

#[derive(Serialize, TS)]
#[serde(tag = "type")]
pub enum EditorType {
    Bool {
        component_key: Option<String>,
        default: Option<bool>,
    },
    Integer {
        component_key: Option<String>,
        default: Option<i32>,
    },
    Float {
        component_key: Option<String>,
        default: Option<f32>,
    },
    Struct {
        component_key: Option<String>,
        #[ts(type = "any | null")]
        default: Option<serde_json::Value>,
        fields: Vec<EditorField>,
    },
    Enum {
        component_key: Option<String>,
        #[ts(type = "any | null")]
        default: Option<serde_json::Value>,
        variants: Vec<EnumVariant>,
    },
    Array {
        component_key: Option<String>,
        #[ts(type = "any[] | null")]
        default: Option<serde_json::Value>,
        of: Rc<EditorType>,
    },
    SingleLine {
        component_key: Option<String>,
        default: Option<String>,
    },
    MultiLine {
        component_key: Option<String>,
        default: Option<String>,
    },
    Markdown {
        component_key: Option<String>,
        default: Option<String>,
    },
    Date {
        component_key: Option<String>,
        default: Option<DateTime<Utc>>,
    },
    DateTime {
        component_key: Option<String>,
        default: Option<DateTime<Utc>>,
    },
}

#[derive(Serialize, TS)]
pub struct EnumVariant {
    pub variant_name: &'static str,
    pub fields: Option<Vec<EditorField>>,
}
