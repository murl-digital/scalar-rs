use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::{ClientError, UploadImageError, WrappedBucket};

impl IntoResponse for UploadImageError {
    fn into_response(self) -> Response {
        match self {
            UploadImageError::MalformedImage => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
            UploadImageError::Client(error) => error.into_response(),
        }
    }
}

pub async fn upload_image(
    State(client): State<WrappedBucket>,
    bytes: Bytes,
) -> Result<String, UploadImageError> {
    client.upload_image(bytes.as_ref().into()).await
}

#[derive(Error, Debug)]
pub enum UploadFileError {
    #[error("multipart processing error: {0}")]
    Multipart(#[from] MultipartError),
    #[error("file upload error: {0}")]
    Upload(#[from] ClientError),
    #[error("expected multipart field not found")]
    NoField,
    #[error("no filename found")]
    NoFileName,
}

impl IntoResponse for UploadFileError {
    fn into_response(self) -> Response {
        match self {
            Self::Multipart(multipart) => multipart.into_response(),
            Self::NoField | Self::NoFileName => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
            Self::Upload(error) => error.into_response(),
        }
    }
}

pub async fn upload_file(
    State(client): State<WrappedBucket>,
    mut multipart: Multipart,
) -> Result<String, UploadFileError> {
    let field = multipart
        .next_field()
        .await?
        .ok_or(UploadFileError::NoField)?;
    let file_name = field
        .file_name()
        .ok_or(UploadFileError::NoFileName)?
        .to_owned();
    let bytes = field.bytes().await?;

    Ok(client.upload_file(file_name, bytes.as_ref().into()).await?)
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, ClientError> {
    Ok(Json(client.list().await?))
}
