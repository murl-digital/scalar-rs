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
    Client(#[from] s3::error::S3Error),
}

impl WrappedBucket {
    pub async fn new(
        bucket: Bucket,
        prefix: Option<impl Into<String>>,
    ) -> Result<Self, CreateBucketError> {
        if !bucket.exists().await? {
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
pub enum UploadError {
    #[error("provided image was malformed")]
    MalformedImage,
    #[error("s3 client error: {0}")]
    Client(#[from] s3::error::S3Error),
}

#[derive(Error, Debug)]
#[error("couldn't list objects in bucket {bucket}, source: {source}")]
pub struct ListImagesError {
    source: Box<dyn std::error::Error>,
    bucket: String,
}

impl WrappedBucket {
    pub async fn upload(&self, image: Arc<[u8]>) -> Result<String, UploadError> {
        let (encoded_bytes, hash_string) = tokio::task::spawn_blocking(move || {
            let image = image::load_from_memory(&image).map_err(|_| UploadError::MalformedImage)?;
            let mut result = BufWriter::new(Cursor::new(Vec::new()));

            image
                .write_to(&mut result, ImageFormat::Png)
                .expect("encoding should never fail at this point");

            let hasher = HasherConfig::new().to_hasher();

            let mut hash = hasher.hash_image(&image).to_base64();
            escape_in_place(&mut hash);

            Ok::<(Vec<u8>, String), UploadError>((result.into_inner().unwrap().into_inner(), hash))
        })
        .await
        .expect("something has gone very wrong")?;

        let prefix = self.prefix.as_str();
        let key = format!("{prefix}{hash_string}.png");

        self.bucket.put_object(key.clone(), &encoded_bytes).await?;

        Ok(format!("{}/{}", self.bucket.url(), key))
    }

    pub async fn list(&self) -> Result<Vec<String>, ListImagesError> {
        Ok(self
            .bucket
            .list(self.prefix.clone(), None)
            .await
            .map_err(|e| ListImagesError {
                source: Box::new(e),
                bucket: format!("{:?}", self.bucket.name()),
            })?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| format!("{}/{}", self.bucket.url(), o.key))
            .collect())
    }
}
