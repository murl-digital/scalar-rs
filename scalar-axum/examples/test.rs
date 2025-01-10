use scalar::{
    db::DatabaseFactory,
    doc_enum,
    validations::{ValidationError, Validator},
    DateTime, Document, Markdown, MultiLine, Utc,
};
use scalar_axum::generate_routes;
use scalar_surreal::{init, SurrealStore};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::{auth::Root, Config},
};
use tower_http::cors::CorsLayer;

#[derive(Document, Serialize, Deserialize)]
struct AllTypes {
    #[validate(skip)]
    bool: bool,
    #[validate(skip)]
    integer: i32,
    #[validate(skip)]
    float: f32,
    #[validate(skip)]
    single_line: String,
    #[validate(skip)]
    multi_line: MultiLine,
    #[validate(skip)]
    markdown: Markdown,
    #[validate(skip)]
    array: Vec<String>,
    #[validate(skip)]
    date_time: DateTime<Utc>,
    enum_select: TestEnum,
}

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test2 {
    #[validate(skip)]
    pub hello: String,
}

#[doc_enum]
#[derive(Clone)]
enum TestEnum {
    Unit,
    Struct { eeee: String },
}

impl Validator for TestEnum {
    fn validate(
        &self,
        field_name: impl AsRef<str>,
    ) -> Result<(), scalar::validations::ValidationError> {
        match self {
            TestEnum::Struct { eeee } if eeee.is_empty() => Err(ValidationError {
                field: field_name.as_ref().into(),
                reason: "eeee must have something in it".into(),
            }),
            _ => Ok(()),
        }
    }
}

#[tokio::main]
async fn main() {
    let factory = SurrealStore::<Client, Ws, _>::new(
        (
            "localhost:8000",
            Config::new().user(Root {
                username: "root",
                password: "root",
            }),
        ),
        "test".into(),
        "test".into(),
    );
    let conn = factory.init_system().await.unwrap();
    init!(conn, AllTypes, Test2);
    drop(conn);
    let app = generate_routes!(factory, SurrealStore<Client, Ws, (&str, Config)>, AllTypes, Test2)
        .layer(CorsLayer::very_permissive())
        .with_state(factory);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&scalar::Utc::now()).unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
