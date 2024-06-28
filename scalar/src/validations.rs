use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

pub enum DataModel {
    Json,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Couldn't deserialize: {0}")]
    Deserialization(#[from] Box<dyn std::error::Error>),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type ValidatorFunction = Arc<dyn Fn(String) -> Result<(), ValidationError> + Sync + Send>;

pub trait Validator {
    fn validate(&self) -> Result<(), ValidationError>;
}

pub fn create_validator_function<V: Validator + DeserializeOwned>(
    model: DataModel,
    validator: impl Fn(&V) -> Result<(), ValidationError> + 'static + Sync + Send,
) -> ValidatorFunction {
    Arc::new(move |input| {
        let value = match model {
            DataModel::Json => serde_json::from_str(&input)
                .map_err(|e| ValidationError::Deserialization(e.into()))?,
        };

        validator(&value)
    })
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
            fn validate(&self) -> Result<(), ValidationError> {
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
        _ => Err(ValidationError::Validation("must be non zero".into())),
    }
}, v}
