use scalar_cms::{
    doc_enum,
    validations::{NonZeroI32, Validate, ValidationError},
    Document, EditorField,
};
use serde::{Deserialize, Serialize};

fn test_fnn(_field: &str) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Document, Serialize, Deserialize)]
#[document(identifier = "mcdonalds sprite")]
#[allow(dead_code)]
struct Hello {
    #[field(title = "dragon enjoyer")]
    #[validate(with = test_fnn)]
    pub oh_my_goodness: String,

    pub wowie: NonZeroI32,

    #[field(title = "this should still work")]
    pub dang: Test,

    #[field(default = 3)]
    #[validate(skip)]
    pub hello: Option<i32>,

    #[validate(skip)]
    pub oh_yes: Vec<i32>,

    #[validate(skip)]
    pub ghost: Ghost,

    #[validate(skip)]
    pub nickelback: LookAtThisStruct,
}

// impl Validate for Hello {
//     fn validate(&self) -> Result<(), ::scalar_cms::validations::ValidationError> {
//         use ::scalar_cms::validations::Validate;

//         let results: [(::scalar_cms::validations::Field, Result<(), ::scalar_cms::validations::ValidationError>); 3] = [
//             ("oh_my_goodness".into(), test_fnn(&self.oh_my_goodness)),
//             ("wowie".into(), Validate::validate(&self.wowie)),
//             ("dang".into(), Validate::validate(&self.dang)),
//         ];

//         let errors: Vec<(Field, ::scalar_cms::validations::ValidationError)> = results
//             .into_iter()
//             .filter_map(|(f, r)| r.err().map(|e| (f, e)))
//             .collect();

//         if errors.is_empty() {
//             Ok(())
//         } else {
//             Err(::scalar_cms::validations::ValidationError::Composite(errors))
//         }
//     }
// }

// impl Document for Hello {
//     fn identifier() -> &'static str {
//         "mcdonalds sprite"
//     }
//     fn title() -> &'static str {
//         "Hello"
//     }
//     fn fields() -> Vec<::scalar_cms::EditorField> {
//         use ::scalar_cms::editor_field::ToEditorField;
//         vec![
//             <String>::to_editor_field(
//                 None::<String>,
//                 "oh_my_goodness",
//                 "dragon enjoyer",
//                 None,
//                 None,
//             ),
//             <NonZeroI32>::to_editor_field(
//                 None::<NonZeroI32>,
//                 "wowie",
//                 "Wowie",
//                 None,
//                 Some("NonZeroI32"),
//             ),
//             <Test>::to_editor_field(None::<Test>, "dang", "this should still work", None, None),
//             <Option<i32>>::to_editor_field(Some(3), "hello", "Hello", None, None),
//             <Vec<i32>>::to_editor_field(None::<Vec<i32>>, "oh_yes", "Oh Yes", None, None),
//             <Ghost>::to_editor_field(None::<Ghost>, "ghost", "Ghost", None, None),
//             <LookAtThisStruct>::to_editor_field(
//                 None::<LookAtThisStruct>,
//                 "nickelback",
//                 "Nickelback",
//                 None,
//                 None,
//             ),
//         ]
//     }
//     fn validate(&self) -> Result<(), Vec<::scalar_cms::validations::ValidationError>> {
//         use ::scalar_cms::validations::Validator;
//         let results = [<NonZeroI32>::validate(&self.wowie)];
//         let errors: Vec<::scalar_cms::validations::ValidationError> =
//             results.into_iter().filter_map(Result::err).collect();

//         if errors.is_empty() {
//             Ok(())
//         } else {
//             Err(errors)
//         }
//     }
// }

#[derive(EditorField, Serialize, Deserialize)]
struct Ghost(i32);

#[derive(EditorField, Serialize, Deserialize)]
struct LookAtThisStruct {
    every_time_i_do_it_makes_me_laugh: String,
    idk_how_the_rest_of_it_goes: f32,
}

#[doc_enum]
enum Test {
    Unit,
    Struct { eeee: String },
}

impl Validate for Test {
    fn validate(&self) -> Result<(), scalar_cms::validations::ValidationError> {
        match self {
            Self::Struct { eeee } if eeee.is_empty() => {
                Err(ValidationError::Single("eeee can't be empty".into()))
            }
            _ => Ok(()),
        }
    }
}

fn main() {
    println!(
        "schema: {}",
        serde_json::to_string_pretty(&Hello::fields()).unwrap()
    )
}
