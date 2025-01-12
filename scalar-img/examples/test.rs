use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    routing::put,
    Router,
};
use axum_macros::debug_handler;
use sc_minio::{provider::StaticProvider, Minio};
use scalar_img::WrappedBucket;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let client = Minio::builder()
        .endpoint("192.168.0.121:9000")
        .provider(StaticProvider::new(
            "4NPWPU3t08ulF8jOXQsm",
            "E0qqX4jUeBP5wA4dUMU9ctOYRfLbdhuhqiRYLD5S",
            None,
        ))
        .secure(false)
        .build()
        .unwrap();

    let wrapped_bucket = WrappedBucket::new(client.bucket("dev"), None::<String>)
        .await
        .unwrap();

    let router = Router::new()
        .route("/upload", put(upload))
        .with_state(wrapped_bucket);

    axum::serve(TcpListener::bind("0.0.0.0:5000").await.unwrap(), router)
        .await
        .unwrap();
}

#[debug_handler]
async fn upload(
    State(client): State<WrappedBucket>,
    mut multipart: Multipart,
) -> Result<String, StatusCode> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        Ok(scalar_img::upload(&client, bytes.as_ref().into()).await)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
