use scalar::Document;

#[derive(Document)]
#[document(identifier = "mcdonalds sprite")]
struct Hello {
    #[field(name = "dragon enjoyer")]
    pub oh_my_goodness: String,

    pub wowie: i32
}

fn main() {
    println!("ident: {}", Hello::identifier())
}
