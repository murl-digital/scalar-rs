use scalar::{
    doc_enum,
    validations::{DataModel, NonZeroI32, Validator},
    Document, EditorField,
};
use serde::{Deserialize, Serialize};

#[derive(Document, Serialize, Deserialize)]
#[document(identifier = "mcdonalds sprite")]
#[allow(dead_code)]
struct Hello {
    #[field(title = "dragon enjoyer")]
    pub oh_my_goodness: String,

    #[field(validate)]
    pub wowie: NonZeroI32,

    #[field(title = "this should still work")]
    pub dang: Test,

    #[field(default = 3)]
    pub hello: Option<i32>,

    pub oh_yes: Vec<i32>,

    pub ghost: Ghost,

    pub nickelback: LookAtThisStruct,
}

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
    fn validate(&self) -> Result<(), scalar::validations::ValidationError> {
        match self {
            Self::Struct { eeee } if eeee.is_empty() => Err(
                scalar::validations::ValidationError::Validation("eeee can't be empty".into()),
            ),
            _ => Ok(()),
        }
    }
}

fn main() {
    println!("ident: {}", Hello::identifier());

    println!(
        "validators: {:?}",
        Hello::validators(DataModel::Json)
            .keys()
            .collect::<Vec<&String>>()
    );

    println!(
        "schema: {}",
        serde_json::to_string_pretty(&Hello::fields()).unwrap()
    )
}
