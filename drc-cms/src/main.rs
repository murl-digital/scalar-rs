use std::{borrow::Cow, collections::BTreeMap, ops::Deref};

use async_trait::async_trait;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use scalar::{DateTime, Document, Markdown, MultiLine, Utc};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::json;
use surrealdb::{
    engine::{
        local::{Db, RocksDb},
        remote::ws::{Client, Ws},
    },
    opt::auth::{Record, Root},
    sql::Thing,
    Connection, Surreal,
};
use tower_http::cors::CorsLayer;

#[derive(Document, Serialize, Deserialize)]
struct Hi {
    ayo: bool,
    ayo2: bool,
    essay: String,
}

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
}

async fn api_key_middleware(
    State(key): State<Hmac<sha2::Sha256>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    if let Some(auth) = headers.get(AUTHORIZATION) {
        println!("headers");
        if let Ok(auth_str) = auth.to_str() {
            println!("string");
            let trimmed_string = auth_str.trim();
            if trimmed_string.starts_with("Bearer ") {
                if let Some((_, token)) = trimmed_string.split_at_checked(7) {
                    let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
                    if let Ok(claims) = claims {
                        let response = next.run(request).await;
                        return response;
                    }
                }
            }
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
    } else {
        println!("{:?}", headers);
        return StatusCode::BAD_REQUEST.into_response();
    }

    return StatusCode::UNAUTHORIZED.into_response();
}

#[tokio::main]
async fn main() {
    // let key: Hmac<sha2::Sha256> = Hmac::new_from_slice(b"VERYBADUSEENVVARIABLEINSTEAD").unwrap();
    // let store = SurrealStore(Surreal::new::<RocksDb>("hi.rocksdb").await.unwrap());
    // store.use_ns("test").await.unwrap();
    // store.use_db("test").await.unwrap();
    // store
    //     .signin(Root {
    //         username: "root",
    //         password: "root",
    //     })
    //     .await
    //     .unwrap();

    // doc_init!(store, Hi, AllTypes);

    // let mut claims = BTreeMap::new();
    // claims.insert("gay_sex", "hell yeah");
    // println!("{}", claims.sign_with_key(&key).unwrap());

    // let router = Router::new()
    //     .nest(
    //         "/cpanel",
    //         generate_routes!(SurrealStore<Db>, SurrealStore<Db>, Hi, AllTypes)
    //             .layer(middleware::from_fn_with_state(key, api_key_middleware)),
    //     )
    //     .with_state(store)
    //     .layer(CorsLayer::very_permissive());

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // println!("{}", Hi::identifier());

    // axum::serve(listener, router).await.unwrap();
    //
    let test = Surreal::new::<Ws>("localhost:8000").await.unwrap();

    println!(
        "{:?}",
        test.signin(Record {
            namespace: "test",
            database: "test",
            access: "user",
            params: json!({
                "email": "wrong",
                "password": "wrong"
            })
        })
        .await
    )
}
