use scalar::{
    doc_enum,
    validations::{DataModel, NonZero, Validator},
    Document,
};

#[derive(Document)]
#[document(identifier = "mcdonalds sprite")]
#[allow(dead_code)]
struct Hello {
    #[field(title = "dragon enjoyer")]
    pub oh_my_goodness: String,

    #[field(validate)]
    pub wowie: NonZero,

    #[field(title = "this should still work")]
    pub dang: Test,

    #[field(default = 3)]
    pub hello: Option<i32>,

    pub oh_yes: Vec<i32>,
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
