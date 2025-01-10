use std::str::FromStr;

use color::{parse_color, DynamicColor};
use scalar::{
    doc_enum,
    validations::{NonZeroI32, Valid, ValidationError, Validator},
    Document, EditorField,
};
use serde::{Deserialize, Serialize};

fn test_fnn(field: &str) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Document, Serialize, Deserialize)]
#[document(identifier = "mcdonalds sprite")]
#[allow(dead_code)]
struct Hello {
    #[field(title = "dragon enjoyer")]
    #[validate(with = "test_fnn")]
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

// impl Document for Hello {
//     fn identifier() -> &'static str {
//         "mcdonalds sprite"
//     }
//     fn title() -> &'static str {
//         "Hello"
//     }
//     fn fields() -> Vec<::scalar::EditorField> {
//         use ::scalar::editor_field::ToEditorField;
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
//     fn validate(&self) -> Result<(), Vec<::scalar::validations::ValidationError>> {
//         use ::scalar::validations::Validator;
//         let results = [<NonZeroI32>::validate(&self.wowie)];
//         let errors: Vec<::scalar::validations::ValidationError> =
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

impl Validator for Test {
    fn validate(
        &self,
        field_name: impl AsRef<str>,
    ) -> Result<(), scalar::validations::ValidationError> {
        match self {
            Self::Struct { eeee } if eeee.is_empty() => Err(ValidationError {
                field: String::from_str(field_name.as_ref()).unwrap(),
                reason: "eeee can't be empty".into(),
            }),
            _ => Ok(()),
        }
    }
}

fn main() {
    let color: DynamicColor = parse_color("#B00B69").unwrap();
    println!("{}", serde_json::to_string_pretty(&color).unwrap());
    println!("ident: {}", Hello::identifier());

    println!(
        "schema: {}",
        serde_json::to_string_pretty(&Hello::fields()).unwrap()
    )
}
