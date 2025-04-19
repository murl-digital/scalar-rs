use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use scalar_img::WrappedBucket;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct UploadImageError(#[from] scalar_img::UploadImageError);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ClientError(#[from] scalar_img::ClientError);

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl IntoResponse for UploadImageError {
    fn into_response(self) -> Response {
        use scalar_img::UploadImageError as Inner;
        match self.0 {
            Inner::MalformedImage => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
            Inner::Client(error) => ClientError(error).into_response(),
        }
    }
}

pub async fn upload_image(
    State(client): State<WrappedBucket>,
    bytes: Bytes,
) -> Result<String, UploadImageError> {
    client.upload_image(bytes).await.map_err(Into::into)
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
    #[error("no filetype found")]
    NoFileType,
}

impl IntoResponse for UploadFileError {
    fn into_response(self) -> Response {
        match self {
            Self::Multipart(multipart) => multipart.into_response(),
            Self::NoField | Self::NoFileName | Self::NoFileType => {
                StatusCode::UNPROCESSABLE_ENTITY.into_response()
            }
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
    let provided_mime_type = field
        .content_type()
        .ok_or(UploadFileError::NoFileType)?
        .to_owned();

    let stream = field
        .map(|result| result.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)));

    let mut reader = StreamReader::new(stream);

    Ok(client
        .upload_file(&file_name, &provided_mime_type, &mut reader)
        .await
        .map_err(ClientError)?)
}

pub async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, ClientError> {
    Ok(Json(client.list_images().await?))
}
