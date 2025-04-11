use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use axum_macros::debug_handler;
use s3::{creds::Credentials, Bucket};
use scalar_img::{UploadImageError, WrappedBucket};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut bucket = Bucket::new(
        "dev",
        s3::Region::Custom {
            region: "".into(),
            endpoint: "http://192.168.0.121:9000".into(),
        },
        Credentials {
            access_key: Some("4NPWPU3t08ulF8jOXQsm".into()),
            secret_key: Some("E0qqX4jUeBP5wA4dUMU9ctOYRfLbdhuhqiRYLD5S".into()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .unwrap();
    bucket.set_path_style();

    let wrapped_bucket = WrappedBucket::new(*bucket, None::<String>).await.unwrap();

    let router = Router::new()
        .route("/upload", put(upload))
        .route("/list", get(list))
        .with_state(wrapped_bucket);

    axum::serve(TcpListener::bind("0.0.0.0:5000").await.unwrap(), router)
        .await
        .unwrap();
}

#[debug_handler]
async fn upload(
    State(client): State<WrappedBucket>,
    bytes: Bytes,
) -> Result<String, UploadImageError> {
    client.upload_image(bytes.as_ref().into()).await
}

#[debug_handler]
async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(client.list().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?))
}
