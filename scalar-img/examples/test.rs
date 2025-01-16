use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
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
        .route("/list", get(list))
        .with_state(wrapped_bucket);

    axum::serve(TcpListener::bind("0.0.0.0:5000").await.unwrap(), router)
        .await
        .unwrap();
}

#[debug_handler]
async fn upload(State(client): State<WrappedBucket>, bytes: Bytes) -> Result<String, StatusCode> {
    client
        .upload(bytes.as_ref().into())
        .await
        .map_err(|e| match e {
            scalar_img::UploadError::MalformedImage => StatusCode::UNPROCESSABLE_ENTITY,
            scalar_img::UploadError::Client(error) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[debug_handler]
async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(
        client
            .list()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
