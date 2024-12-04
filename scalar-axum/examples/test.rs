use std::io::ErrorKind;

use axum::async_trait;
use scalar::{
    db::DatabaseFactory,
    doc_enum, nanoid,
    validations::{ValidationError, Validator},
    DateTime, Document, Item, Markdown, MultiLine, Utc,
};
use scalar_axum::generate_routes;
use scalar_surreal::{init, SurrealStore};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{
    engine::{
        local::{Db, SurrealKv},
        remote::ws::{Client, Ws},
    },
    opt::{auth::Root, Config},
};
use tower_http::cors::CorsLayer;

#[derive(Document, Serialize, Deserialize)]
struct AllTypes {
    bool: bool,
    integer: i32,
    float: f32,
    single_line: String,
    multi_line: MultiLine,
    markdown: Markdown,
    array: Vec<String>,
    date_time: DateTime<Utc>,
    enum_select: TestEnum,
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
    let factory = SurrealStore::<Db, SurrealKv, _>::new(
        (
            "hi.db",
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
    let app =
        generate_routes!(factory, SurrealStore<Db, SurrealKv, (&str, Config)>, AllTypes, Test2)
            .layer(CorsLayer::very_permissive())
            .with_state(factory);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&scalar::Utc::now()).unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
