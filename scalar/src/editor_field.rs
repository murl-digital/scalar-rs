use std::rc::Rc;

use serde::Serialize;
use ts_rs::TS;

use crate::{EditorType, Markdown, MultiLine};

#[derive(Serialize, TS)]
#[ts(export)]
pub struct EditorField {
    pub name: &'static str,
    pub title: &'static str,
    pub placeholder: Option<&'static str>,
    pub validator: Option<&'static str>,
    pub required: bool,
    pub field_type: EditorType,
}

/// Convert an input type into a `scalar::EditorField`
/// It's done this way with generics to make dealing with this in derive macros easier, and options. Oh god, options.
pub trait ToEditorField<T> {
    fn to_editor_field(
        default: Option<impl Into<T>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized;
}

impl ToEditorField<i32> for i32 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: crate::EditorType::Integer {
                default: default.map(|i| i.into()),
            },
        }
    }
}

impl ToEditorField<f32> for f32 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: crate::EditorType::Float {
                default: default.map(|i| i.into()),
            },
        }
    }
}

impl ToEditorField<String> for String {
    fn to_editor_field(
        _default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: crate::EditorType::SingleLine,
        }
    }
}

impl ToEditorField<MultiLine> for MultiLine {
    fn to_editor_field(
        _default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: crate::EditorType::MultiLine,
        }
    }
}

impl ToEditorField<Markdown> for Markdown {
    fn to_editor_field(
        _default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: crate::EditorType::Markdown,
        }
    }
}

impl<T> ToEditorField<T> for Option<T>
where
    T: ToEditorField<T>,
{
    fn to_editor_field(
        default: Option<impl Into<T>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let mut field = T::to_editor_field(default, name, title, placeholder, validator);
        field.required = false;

        field
    }
}

impl<T> ToEditorField<T> for Vec<T>
where
    T: ToEditorField<T>,
{
    fn to_editor_field(
        default: Option<impl Into<T>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let dummy_field = T::to_editor_field(default, name, title, placeholder, validator);
        let field_type = dummy_field.field_type;

        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: EditorType::Array {
                of: Rc::new(field_type),
            },
        }
    }
}
