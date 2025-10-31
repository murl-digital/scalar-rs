use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use s3::{creds::Credentials, Bucket};
use scalar_img::WrappedBucket;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut bucket = Bucket::new(
        "dev",
        s3::Region::Custom {
            region: String::new(),
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

    let wrapped_bucket = WrappedBucket::new(*bucket).await.unwrap();

    let router = Router::new()
        .route("/upload", put(upload))
        .route("/list", get(list))
        .with_state(wrapped_bucket);

    axum::serve(TcpListener::bind("0.0.0.0:5000").await.unwrap(), router)
        .await
        .unwrap();
}

async fn upload(State(client): State<WrappedBucket>, bytes: Bytes) -> Result<String, StatusCode> {
    client
        .upload_image(bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(client.list_images().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?))
}
