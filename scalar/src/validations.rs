use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;

use crate::editor_field::ToEditorField;

pub enum DataModel {
    Json,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Couldn't deserialize: {0}")]
    Deserialization(#[from] Box<dyn std::error::Error>),
    #[error("Validation error: {0}")]
    Validation(String)
}

pub type ValidatorFunction = Arc<dyn Fn(String) -> Result<(), ValidationError> + Sync + Send>;

pub trait Validator {
    fn validate(&self) -> Result<(), ValidationError>;
}

pub fn create_validator_function<V: Validator + DeserializeOwned>(model: DataModel, validator: impl Fn(&V) -> Result<(), ValidationError> + 'static + Sync + Send) -> ValidatorFunction {
    Arc::new(move |input| {
        let value = match model {
            DataModel::Json => {
                serde_json::from_str(&input).map_err(|e| ValidationError::Deserialization(e.into()))?
            }
        };

        validator(&value)
    })
}

#[derive(Deserialize)]
pub struct NonZero(pub i32);

impl Validator for NonZero {
    fn validate(&self) -> Result<(), ValidationError> {
        match self.0 {
            0 => Ok(()),
            _ => Err(ValidationError::Validation("must be non zero".into()))
        }
    }
}

impl ToEditorField<i32> for NonZero {
    fn to_editor_field(
        default: Option<impl Into<i32>>,
        name: &'static str,
        title: &'static str,
        placeholder: Option<&'static str>,
        validator: Option<&'static str>
    ) -> crate::EditorField
    where
        Self: std::marker::Sized {
        i32::to_editor_field(default, name, title, placeholder, validator)
    }
}

impl From<NonZero> for i32 {
    fn from(val: NonZero) -> Self {
        val.0
    }
}