use crate::{EditorField, Markdown, MultiLine};

pub trait ToEditorField {
    fn to_editor_field(default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized;
}

impl ToEditorField for i32 {
    fn to_editor_field(default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, title, placeholder, field_type: crate::EditorType::Integer { default: default.map(|i| i.into()) } }
    }
}

impl ToEditorField for String {
    fn to_editor_field(_default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, title, placeholder, field_type: crate::EditorType::SingleLine }
    }
}

impl ToEditorField for MultiLine {
    fn to_editor_field(_default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, title, placeholder, field_type: crate::EditorType::MultiLine }
    }
}

impl ToEditorField for Markdown {
    fn to_editor_field(_default: Option<impl Into<Self>>, name: &'static str, title: &'static str, placeholder: Option<&'static str>) -> EditorField where Self: std::marker::Sized {
        EditorField { name, title, placeholder, field_type: crate::EditorType::Markdown }
    }
}