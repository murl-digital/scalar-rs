use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::Document;

pub struct Valid<T: Document>(T);

impl<T: Document> Valid<T> {
    pub fn new(doc: T) -> Result<Self, Vec<ValidationError>> {
        doc.validate()?;
        Ok(Self(doc))
    }
}

#[derive(Error, Debug, Serialize)]
#[error("Validation error on {field}: {reason}")]
pub struct ValidationError {
    pub field: String,
    pub reason: String,
}

#[diagnostic::on_unimplemented(
    note = "all document fields are validated by default",
    note = "if validation isn't necesarry, use #[validate(skip)]"
)]
pub trait Validator {
    fn validate(&self, field_name: impl AsRef<str>) -> Result<(), ValidationError>;
}

impl<T: Validator> Validator for Option<T> {
    fn validate(&self, field_name: impl AsRef<str>) -> Result<(), ValidationError> {
        match self.as_ref() {
            Some(inner) => inner.validate(field_name),
            None => Ok(()),
        }
    }
}

macro_rules! validator {
    ($ty:ty, $inner:ty, $expr:block, $v:ident) => {
        impl crate::editor_field::ToEditorField<$inner> for $ty {
            fn to_editor_field(
                default: Option<impl Into<$inner>>,
                name: &'static str,
                title: &'static str,
                placeholder: Option<&'static str>,
                validator: Option<&'static str>,
            ) -> crate::EditorField
            where
                Self: std::marker::Sized,
            {
                <$inner>::to_editor_field(default, name, title, placeholder, validator)
            }
        }

        impl From<$ty> for $inner {
            fn from(val: $ty) -> Self {
                val.0
            }
        }

        impl Validator for $ty {
            fn validate(&self, field_name: impl AsRef<str>) -> Result<(), ValidationError> {
                let $v = self;
                $expr
            }
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct NonZeroI32(pub i32);

validator! {NonZeroI32, i32, {
    match v.0 {
        0 => Ok(()),
        _ => Err(todo!()),
    }
}, v}
