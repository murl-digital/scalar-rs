use chrono::{DateTime, TimeZone};
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

/// Convert an input type into a `scalar_cms::EditorField`
pub trait ToEditorField {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized;
}

impl ToEditorField for () {
    fn to_editor_field(
        _default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
            field_type: crate::EditorType::Null {
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for bool {
    fn to_editor_field(
        default: Option<impl Into<bool>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for i32 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for f32 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for String {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for MultiLine {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into).map(|v| v.0),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl ToEditorField for Markdown {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into).map(|v| v.0),
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl<Z: TimeZone> ToEditorField for DateTime<Z> {
    fn to_editor_field(
        default: Option<impl Into<DateTime<Z>>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                component_key: component_key.map(Into::into),
            },
        }
    }
}

impl<T> ToEditorField for Option<T>
where
    T: ToEditorField + Serialize,
{
    fn to_editor_field(
        default: Option<impl Into<Option<T>>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let test = default.and_then(Into::into);

        let mut field =
            T::to_editor_field(test, name, title, placeholder, validator, component_key);
        field.required = false;

        field
    }
}

impl<T> ToEditorField for Vec<T>
where
    T: ToEditorField + Serialize,
{
    fn to_editor_field(
        default: Option<impl Into<Vec<T>>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let dummy_field = T::to_editor_field(
            None::<T>,
            name,
            title,
            placeholder,
            validator,
            component_key,
        );
        let field_type = dummy_field.field_type;

        EditorField {
            name,
            title,
            placeholder,
            required: true,
            validator,
            field_type: EditorType::Array {
                default: default
                    .map(|v| serde_json::to_value(v.into()).expect("this should never fail")),
                component_key: component_key.map(Into::into),
                of: Box::new(field_type),
            },
        }
    }
}

#[cfg(feature = "url")]
impl ToEditorField for url::Url {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
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
                default: default.map(Into::into).map(Into::into),
                component_key: component_key.map(Into::into).or_else(|| Some("url".into())),
            },
        }
    }
}

#[cfg(feature = "rgb")]
impl ToEditorField for rgb::RGB8 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let default = default.map(Into::into);

        super::EditorField {
            name,
            title,
            placeholder,
            validator,
            required: true,
            field_type: crate::EditorType::Struct {
                component_key: component_key.map(Into::into).or(Some("color-input".into())),
                default: default.map(crate::convert),
                fields: vec![
                    i32::to_editor_field(default.map(|c| c.r as i32), "r", "", None, None, None),
                    i32::to_editor_field(default.map(|c| c.g as i32), "g", "", None, None, None),
                    i32::to_editor_field(default.map(|c| c.b as i32), "b", "", None, None, None),
                ],
            },
        }
    }
}

#[cfg(feature = "rgb")]
impl ToEditorField for rgb::RGBA8 {
    fn to_editor_field(
        default: Option<impl Into<Self>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>,
        component_key: Option<&'static str>,
    ) -> EditorField
    where
        Self: std::marker::Sized,
    {
        let default = default.map(Into::into);

        super::EditorField {
            name,
            title,
            placeholder,
            validator,
            required: true,
            field_type: crate::EditorType::Struct {
                component_key: component_key.map(Into::into).or(Some("color-input".into())),
                default: default.map(crate::convert),
                fields: vec![
                    i32::to_editor_field(default.map(|c| c.r as i32), "r", "", None, None, None),
                    i32::to_editor_field(default.map(|c| c.g as i32), "g", "", None, None, None),
                    i32::to_editor_field(default.map(|c| c.b as i32), "b", "", None, None, None),
                    i32::to_editor_field(default.map(|c| c.a as i32), "a", "", None, None, None),
                ],
            },
        }
    }
}
