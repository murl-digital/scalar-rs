use std::rc::Rc;

use chrono::{DateTime, TimeZone};
use serde::{Deserialize, Serialize};
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

impl ToEditorField<bool> for bool {
    fn to_editor_field(
        default: Option<impl Into<bool>>,
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
            field_type: crate::EditorType::Bool {
                default: default.map(|i| i.into()),
            },
        }
    }
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
            field_type: crate::EditorType::SingleLine {
                default: default.map(|i| i.into()),
            },
        }
    }
}

impl ToEditorField<MultiLine> for MultiLine {
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
            field_type: crate::EditorType::MultiLine {
                default: default.map(|i| i.into().0),
            },
        }
    }
}

impl ToEditorField<Markdown> for Markdown {
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
            field_type: crate::EditorType::Markdown {
                default: default.map(|i| i.into().0),
            },
        }
    }
}

impl<Z: TimeZone> ToEditorField<DateTime<Z>> for DateTime<Z> {
    fn to_editor_field(
        default: Option<impl Into<DateTime<Z>>>,
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
            validator,
            required: true,
            field_type: EditorType::DateTime {
                default: default.map(Into::into).as_ref().map(DateTime::to_utc),
            },
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

// impl<T> ToEditorField<T> for Vec<T>
// where
//     T: ToEditorField<T>,
// {
//     fn to_editor_field(
//         default: Option<impl Into<T>>,
//         name: &'static str,
//         title: &'static str,
//         placeholder: Option<&'static str>,
//         validator: Option<&'static str>,
//     ) -> EditorField
//     where
//         Self: std::marker::Sized,
//     {
//         let dummy_field = T::to_editor_field(default, name, title, placeholder, validator);
//         let field_type = dummy_field.field_type;

//         EditorField {
//             name,
//             title,
//             placeholder,
//             required: true,
//             validator,
//             field_type: EditorType::Array {
//                 default: Some(
//                     serde_json::to_value(Vec::<i32>::default()).expect("this should never fail"),
//                 ),
//                 of: Rc::new(field_type),
//             },
//         }
//     }
// }

impl<T> ToEditorField<Vec<T>> for Vec<T>
where
    T: ToEditorField<T>,
{
    fn to_editor_field(
        default: Option<impl Into<Vec<T>>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let dummy_field = T::to_editor_field(None::<T>, name, title, placeholder, validator);
        let field_type = dummy_field.field_type;

        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: EditorType::Array {
                //default: default.map(|v| serde_json::to_value(v).expect("this should never fail")),
                default: None,
                of: Rc::new(field_type),
            },
        }
    }
}
