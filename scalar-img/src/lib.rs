use std::{
    io::{BufWriter, Cursor},
    sync::Arc,
};

use base64_url::escape_in_place;
use image::ImageFormat;
use image_hasher::HasherConfig;
use s3::Bucket;
use scalar::{editor_field::ToEditorField, EditorField};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[cfg(feature = "axum")]
pub mod axum;

#[derive(EditorField, Serialize, Deserialize)]
#[field(editor_component = "image")]
pub struct ImageData<D: ToEditorField> {
    url: Url,
    additional_data: D,
}

#[derive(EditorField, Serialize, Deserialize)]
#[field(editor_component = "file")]
pub struct FileData<D: ToEditorField> {
    url: Url,
    additional_data: D,
}

#[derive(Clone)]
pub struct WrappedBucket {
    bucket: Bucket,
    prefix: String,
}

#[derive(Error, Debug)]
pub enum CreateBucketError {
    #[error("can't access provided bucket {0:?}. either it doesn't exist or the provided client doesn't have access to it.")]
    NoAccess(String),
    #[error("s3 client error: {0}")]
    Client(#[from] ClientError),
}

impl WrappedBucket {
    pub async fn new(
        bucket: Bucket,
        prefix: Option<impl Into<String>>,
    ) -> Result<Self, CreateBucketError> {
        if !bucket.exists().await.map_err(|e| ClientError {
            source: Box::new(e),
            bucket: bucket.name(),
        })? {
            return Err(CreateBucketError::NoAccess(bucket.name));
        }

        Ok(Self {
            bucket,
            prefix: prefix
                .map(Into::into)
                .map(|s| format!("{s}/"))
                .unwrap_or_default(),
        })
    }
}

#[derive(Error, Debug)]
pub enum UploadImageError {
    #[error("provided image was malformed")]
    MalformedImage,
    #[error("s3 client error: {0}")]
    Client(#[from] ClientError),
}

#[derive(Error, Debug)]
#[error("couldn't list objects in bucket {bucket}, source: {source}")]
pub struct ClientError {
    source: Box<dyn std::error::Error + Send>,
    bucket: String,
}

impl WrappedBucket {
    pub async fn upload_image(&self, image: Arc<[u8]>) -> Result<String, UploadImageError> {
        let (encoded_bytes, hash_string) = tokio::task::spawn_blocking(move || {
            let image =
                image::load_from_memory(&image).map_err(|_| UploadImageError::MalformedImage)?;
            let mut result = BufWriter::new(Cursor::new(Vec::new()));

            image
                .write_to(&mut result, ImageFormat::Png)
                .expect("encoding should never fail at this point");

            let hasher = HasherConfig::new().to_hasher();

            let mut hash = hasher.hash_image(&image).to_base64();
            escape_in_place(&mut hash);

            Ok::<(Vec<u8>, String), UploadImageError>((
                result.into_inner().unwrap().into_inner(),
                hash,
            ))
        })
        .await
        .expect("something has gone very wrong")?;

        let prefix = self.prefix.as_str();
        let key = format!("{prefix}image/{hash_string}.png");

        self.bucket
            .put_object(&key, &encoded_bytes)
            .await
            .map_err(|e| self.client_error(e))?;

        Ok(format!("{}/{}", self.bucket.url(), key))
    }

    pub async fn upload_file(
        &self,
        file_name: String,
        file: Arc<[u8]>,
    ) -> Result<String, ClientError> {
        let prefix = self.prefix.as_str();
        let key = format!("{prefix}files/{file_name}");

        self.bucket
            .put_object(&key, &file)
            .await
            .map_err(|e| self.client_error(e))?;

        Ok(format!("{}/{}", self.bucket.url(), key))
    }

    pub async fn list(&self) -> Result<Vec<String>, ClientError> {
        Ok(self
            .bucket
            .list(self.prefix.clone(), None)
            .await
            .map_err(|e| self.client_error(e))?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| format!("{}/{}", self.bucket.url(), o.key))
            .collect())
    }

    fn client_error<E: std::error::Error + Send + 'static>(&self, error: E) -> ClientError {
        ClientError {
            source: Box::new(error),
            bucket: self.bucket.name(),
        }
    }
}
