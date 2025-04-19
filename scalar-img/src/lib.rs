use std::io::{BufWriter, Cursor};

use base64_url::escape_in_place;
use bytes::Bytes;
use image::ImageFormat;
use image_hasher::HasherConfig;
use s3::Bucket;
use scalar::{editor_field::ToEditorField, EditorField};
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

#[derive(EditorField, Serialize, Deserialize)]
#[field(editor_component = "file")]
pub struct FileData<D: ToEditorField> {
    url: Url,
    additional_data: D,
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
    pub async fn new(bucket: Bucket) -> Result<Self, CreateBucketError> {
        if !bucket.exists().await.map_err(|e| ClientError {
            source: Box::new(e),
            bucket: bucket.name(),
        })? {
            return Err(CreateBucketError::NoAccess(bucket.name));
        }

        let public_url = Url::parse(&format!("{}/", &bucket.url()))
            .expect("bucket url should DEFINITELY be valid...");

        println!("{public_url}");

        Ok(Self { bucket, public_url })
    }

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
    pub async fn upload_image(&self, image: Bytes) -> Result<String, UploadImageError> {
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
            .expect("url should always be valid")
            .to_string())
    }

    pub async fn list_images(&self) -> Result<Vec<String>, ClientError> {
        Ok(self
            .bucket
            .list(IMAGES_PREFIX.into(), None)
            .await
            .map_err(|e| self.client_error(e))?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| {
                self.public_url
                    .join(&o.key)
                    .expect("should always be a valid url")
                    .to_string()
            })
            .collect())
    }

    pub async fn list_files(&self) -> Result<Vec<String>, ClientError> {
        Ok(self
            .bucket
            .list(FILES_PREFIX.into(), None)
            .await
            .map_err(|e| self.client_error(e))?
            .iter()
            .flat_map(|r| &r.contents)
            .map(|o| {
                self.public_url
                    .join(&o.key)
                    .expect("should always be a valid url")
                    .to_string()
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
