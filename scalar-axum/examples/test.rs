use std::io::ErrorKind;

use axum::async_trait;
use scalar::{
    db::DatabaseFactory,
    doc_enum, nanoid,
    validations::{ValidationError, Validator},
    Document, Item, Utc,
};
use scalar_axum::generate_routes;
use scalar_surreal::{init, SurrealStore};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::{auth::Root, Config},
};
use tower_http::cors::CorsLayer;

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test {
    pub hi: String,
    pub number: i32,
    #[field(validate)]
    pub test: TestEnum,
}

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test2 {
    pub hello: String,
}

#[doc_enum]
#[derive(Clone)]
enum TestEnum {
    Unit,
    Struct { eeee: String },
}

impl Validator for TestEnum {
    fn validate(&self) -> Result<(), scalar::validations::ValidationError> {
        match self {
            TestEnum::Struct { eeee } if eeee.is_empty() => Err(ValidationError::Validation(
                "eeee must have something in it".into(),
            )),
            _ => Ok(()),
        }
    }
}

#[tokio::main]
async fn main() {
    let factory =
        SurrealStore::<Client, Ws, _>::new("localhost:8000", "test".into(), "test".into());
    let conn = factory.init_system().await.unwrap();
    init!(conn, Test, Test2);
    drop(conn);
    let app = generate_routes!(factory, SurrealStore<Client, Ws, &str>, Test, Test2)
        .layer(CorsLayer::very_permissive())
        .with_state(factory);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&scalar::Utc::now()).unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
