use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    editor_field::ToEditorField,
    validations::{Validate, ValidationError},
};

macro_rules! deref {
    ($ty:ty > $target:ty) => {
        impl Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    (generic $ty:ident > $target:ty) => {
        impl<T: ToEditorField> Deref for $ty<T> {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T: ToEditorField> DerefMut for $ty<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiLine(pub String);

deref!(MultiLine > str);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Markdown(pub String);

deref!(Markdown > str);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Slug(pub String);

deref!(Slug > str);

impl Validate for Slug {
    // TODO: sanity slugs are guaranteed to be unique, how to handle this?
    fn validate(&self) -> Result<(), crate::validations::ValidationError> {
        self.0
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            .then_some(())
            .ok_or_else(|| {
                ValidationError::Single(
                    "slugs can only contain alphanumeic characters, -, and _.".into(),
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Toggle<T: ToEditorField>(pub Option<T>);

deref!(generic Toggle > Option<T>);
