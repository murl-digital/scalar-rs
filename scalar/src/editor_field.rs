use crate::{EditorField, Markdown, MultiLine};

pub trait ToEditorField {
    fn to_editor_field(default: Option<Self>, name: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized;
}

impl ToEditorField for i32 {
    fn to_editor_field(default: Option<Self>, name: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, placeholder, field_type: crate::EditorType::Integer { default } }
    }
}

impl ToEditorField for String {
    fn to_editor_field(default: Option<Self>, name: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, placeholder, field_type: crate::EditorType::SingleLine }
    }
}

impl ToEditorField for MultiLine {
    fn to_editor_field(default: Option<Self>, name: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, placeholder, field_type: crate::EditorType::MultiLine }
    }
}

impl ToEditorField for Markdown {
    fn to_editor_field(default: Option<Self>, name: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, placeholder, field_type: crate::EditorType::Markdown }
    }
}