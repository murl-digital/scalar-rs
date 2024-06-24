use scalar::{doc_enum, Document};

#[derive(Document)]
#[document(identifier = "mcdonalds sprite")]
#[allow(dead_code)]
struct Hello {
    #[field(title = "dragon enjoyer")]
    pub oh_my_goodness: String,

    pub wowie: i32,

    #[field(title = "this should still work")]
    pub dang: Test,

    #[field(default = 3)]
    pub hello: Option<i32>,
}

#[doc_enum]
enum Test {
    Unit,
    Struct { eeee: String },
}

fn main() {
    println!("ident: {}", Hello::identifier());

    println!(
        "schema: {}",
        serde_json::to_string_pretty(&Hello::fields()).unwrap()
    )
}
