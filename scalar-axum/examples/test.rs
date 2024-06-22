use std::{any::Any, collections::HashMap, sync::Arc};

use axum::{async_trait, routing::post, Json, Router};
use scalar::{nanoid, Document, Item, Utc, DB};
use scalar_axum::{create, generate_routes, ScalarState};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test {
    pub hi: String,
    pub number: i32
}

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test2 {
    pub hello: String
}

#[derive(Clone)]
struct FSDB;

#[async_trait]
impl DB for FSDB {
    async fn create<D: Document + Serialize + Send>(&self, doc: D) -> Result<Item<D>, ()> {
        let now = Utc::now();
        let item = Item {
            id: nanoid!(),
            created_at: now,
            modified_at: now,
            published_at: None,
            inner: doc
        };
        tokio::fs::write(format!("./db/{}.json", item.id), serde_json::to_string_pretty(&item).unwrap()).await.unwrap();

        Ok(item)
    }

    async fn update<D: Document + Serialize + Send>(&self, item: Item<D>) -> Result<Item<D>, ()> {
        tokio::fs::write(format!("./db/{}.json", item.id), serde_json::to_string_pretty(&item).unwrap()).await.unwrap();

        Ok(item)
    }

    async fn delete<D: Document + Send>(&self, doc: Item<D>) -> Result<Item<D>, ()> {
        tokio::fs::remove_file(format!("./db/{}.json", doc.id)).await.unwrap();

        Ok(doc)
    }
}

#[derive(Clone)]
struct State {
    db: FSDB
}

impl ScalarState<FSDB> for State {
    fn get_db(&self) -> &FSDB {
        &self.db
    }
}

#[axum_macros::debug_handler]
async fn test_route(state: axum::extract::State<State>, doc: Json<Test>) -> Json<Item<Test>> {
    let db = state.get_db();

    let item = db.create(doc.0).await.unwrap();

    Json(item)
}

#[tokio::main]
async fn main() {
    let state = State { db: FSDB };
    let app = generate_routes!(State, FSDB, Test, Test2).with_state(state).layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}