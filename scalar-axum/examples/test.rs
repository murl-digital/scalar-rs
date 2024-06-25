use axum::async_trait;
use scalar::{doc_enum, nanoid, validations::{ValidationError, Validator}, Document, Item, Utc, DB};
use scalar_axum::{generate_routes, ScalarState};
use serde::{Deserialize, Serialize};
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
            TestEnum::Struct { eeee } if eeee.is_empty() => Err(ValidationError::Validation("eeee must have something in it".into())),
            _ => Ok(())
        }
    }
}

#[derive(Clone)]
struct Fsdb;

#[async_trait]
impl DB for Fsdb {
    async fn create<D: Document + Serialize + Send>(&self, doc: D) -> Result<Item<D>, ()> {
        let now = Utc::now();
        let item = Item {
            id: nanoid!(),
            created_at: now,
            modified_at: now,
            published_at: None,
            inner: doc,
        };
        tokio::fs::write(
            format!("./db/{}.json", item.id),
            serde_json::to_string_pretty(&item).unwrap(),
        )
        .await
        .unwrap();

        Ok(item)
    }

    async fn update<D: Document + Serialize + Send>(&self, item: Item<D>) -> Result<Item<D>, ()> {
        tokio::fs::write(
            format!("./db/{}.json", item.id),
            serde_json::to_string_pretty(&item).unwrap(),
        )
        .await
        .unwrap();

        Ok(item)
    }

    async fn delete<D: Document + Send>(&self, doc: Item<D>) -> Result<Item<D>, ()> {
        tokio::fs::remove_file(format!("./db/{}.json", doc.id))
            .await
            .unwrap();

        Ok(doc)
    }
}

#[derive(Clone)]
struct State {
    db: Fsdb,
}

impl ScalarState<Fsdb> for State {
    fn get_db(&self) -> &Fsdb {
        &self.db
    }
}

#[tokio::main]
async fn main() {
    let state = State { db: Fsdb };
    let app = generate_routes!(State, Fsdb, Test, Test2)
        .with_state(state)
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&scalar::Utc::now()).unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
