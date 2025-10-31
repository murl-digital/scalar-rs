use std::io::Cursor;

use base64_url::escape_in_place;
use bytes::Bytes;
use image::ImageFormat;
use image_hasher::HasherConfig;
use s3::Bucket;
use scalar_cms::{editor_field::ToEditorField, validations::Validate, EditorField};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::AsyncRead;
use url::Url;

#[derive(EditorField, Serialize, Deserialize)]
#[field(editor_component = "image")]
pub struct ImageData<D: ToEditorField> {
    url: Url,
    additional_data: D,
}

impl<D: ToEditorField + Validate> Validate for ImageData<D> {
    fn validate(&self) -> Result<(), scalar_cms::validations::ValidationError> {
        self.additional_data.validate()
    }
}

#[derive(EditorField, Serialize, Deserialize)]
#[field(editor_component = "file")]
pub struct FileData<D: ToEditorField> {
    url: Url,
    additional_data: D,
}

impl<D: ToEditorField + Validate> Validate for FileData<D> {
    fn validate(&self) -> Result<(), scalar_cms::validations::ValidationError> {
        self.additional_data.validate()
    }
}

#[derive(Clone)]
pub struct WrappedBucket {
    bucket: Bucket,
    public_url: Url,
}

#[derive(Error, Debug)]
pub enum CreateBucketError {
    #[error("can't access provided bucket {0:?}. either it doesn't exist or the provided client doesn't have access to it.")]
    NoAccess(String),
    #[error("s3 client error: {0}")]
    Client(#[from] ClientError),
}

impl WrappedBucket {
    /// Creates a new `WrappedBucket`.
    ///
    /// # Panics
    ///
    /// Panics if the bucket's url cannot be parsed with `Url`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the bucket can't be reached or accessed.
    pub async fn new(bucket: Bucket) -> Result<Self, CreateBucketError> {
        if !bucket.exists().await.map_err(|e| ClientError {
            source: Box::new(e),
            bucket: bucket.name(),
        })? {
            return Err(CreateBucketError::NoAccess(bucket.name));
        }

        let public_url = Url::parse(&bucket.url()).expect("a valid bucket url");

        Ok(Self { bucket, public_url })
    }

    #[must_use]
    pub fn with_public_url(mut self, public_url: Url) -> Self {
        self.public_url = public_url;
        self
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
    source: Box<dyn std::error::Error + Send + Sync>,
    bucket: String,
}

const FILES_PREFIX: &str = "files";
const IMAGES_PREFIX: &str = "images";

impl WrappedBucket {
    /// Uploads an image to the bucket. The file name will be the image hash, instead of the input file name. This allows for deduplication.
    ///
    /// # Panics
    ///
    /// Panics if the public url is a cannot-be-a-base url (example: data:foo or mailto:example@example.com).
    ///
    /// # Errors
    ///
    /// This function will return an error if making a request to the bucket fails for any reason.
    pub async fn upload_image(&self, image: Bytes) -> Result<String, UploadImageError> {
        let (encoded_bytes, hash_string) = tokio::task::spawn_blocking(move || {
            let image =
                image::load_from_memory(&image).map_err(|_| UploadImageError::MalformedImage)?;
            let mut result = Cursor::new(Vec::new());

            image
                .write_to(&mut result, ImageFormat::Png)
                .map_err(|_| UploadImageError::MalformedImage)?;

            let hasher = HasherConfig::new().to_hasher();

            let mut hash = hasher.hash_image(&image).to_base64();
            escape_in_place(&mut hash);

            Ok::<(Vec<u8>, String), UploadImageError>((result.into_inner(), hash))
        })
        .await
        .map_err(|_| UploadImageError::MalformedImage)??;

        let key = format!("{IMAGES_PREFIX}/{hash_string}.png");

        self.bucket
            .put_object_with_content_type(&key, &encoded_bytes, "image/png")
            .await
            .map_err(|e| self.client_error(e))?;

        Ok(self
            .public_url
            .join(&key)
            .expect("url should always be valid")
            .to_string())
    }

    /// Uploads a file to the bucket. If the file is meant to be used as an image, use `WrappedBucket::upload_image` instead.
    ///
    /// # Panics
    ///
    /// Panics if the public url is a cannot-be-a-base url (example: data:foo or mailto:example@example.com).
    ///
    /// # Errors
    ///
    /// This function will return an error if making a request to the bucket fails for any reason.
    pub async fn upload_file<R: AsyncRead + Unpin>(
        &self,
        file_name: &str,
        mime_type: &str,
        file: &mut R,
    ) -> Result<String, ClientError> {
        let key = format!("{FILES_PREFIX}/{file_name}");

        self.bucket
            .put_object_stream_with_content_type(file, &key, mime_type)
            .await
            .map_err(|e| self.client_error(e))?;

        Ok(self
            .public_url
            .join(&key)
            .expect("public url must have a base")
            .to_string())
    }

    /// Lists Images in this [`WrappedBucket`].
    ///
    /// # Panics
    ///
    /// Panics if the public url is a cannot-be-a-base url (example: data:foo or mailto:example@example.com).
    ///
    /// # Errors
    ///
    /// This function will return an error if making a request to the bucket fails for any reason.
    pub async fn list_images(&self) -> Result<Vec<String>, ClientError> {
        Ok(self
            .bucket
            .list(IMAGES_PREFIX.into(), None)
            .await
            .map_err(|e| self.client_error(e))?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| {
                let mut url = self.public_url.clone();
                url.path_segments_mut()
                    .expect("public url must have a base")
                    .push(&o.key);
                url.to_string()
            })
            .collect())
    }

    /// List files in this [`WrappedBucket`]. Does not include images.
    ///
    /// # Panics
    ///
    /// Panics if the public url is a cannot-be-a-base url (example: data:foo or mailto:example@example.com).
    ///
    /// # Errors
    ///
    /// This function will return an error if making a request to the bucket fails for any reason.
    pub async fn list_files(&self) -> Result<Vec<String>, ClientError> {
        Ok(self
            .bucket
            .list(FILES_PREFIX.into(), None)
            .await
            .map_err(|e| self.client_error(e))?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| {
                let mut url = self.public_url.clone();
                url.path_segments_mut()
                    .expect("public url must have a base")
                    .push(&o.key);
                url.to_string()
            })
            .collect())
    }
}

impl WrappedBucket {
    fn client_error<E: std::error::Error + Send + Sync + 'static>(&self, error: E) -> ClientError {
        ClientError {
            source: Box::new(error),
            bucket: self.bucket.name(),
        }
    }
}
