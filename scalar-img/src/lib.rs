use std::{
    io::{BufWriter, Cursor},
    sync::Arc,
};

use image::ImageFormat;
use image_hasher::HasherConfig;
use sc_minio::{
    client::{Bucket, BucketArgs},
    Minio,
};
use thiserror::Error;

#[derive(Clone)]
pub struct WrappedBucket {
    bucket: Bucket,
    prefix: String,
}

#[derive(Error, Debug)]
pub enum CreateBucketError {
    #[error("can't access provided bucket {0:?}. either it doesn't exist or the provided client doesn't have access to it.")]
    NoAccess(BucketArgs),
    #[error("minio client error: {0}")]
    ClientError(#[from] sc_minio::error::Error),
}

impl WrappedBucket {
    pub async fn new<'e>(
        bucket: Bucket,
        prefix: Option<impl Into<String>>,
    ) -> Result<Self, CreateBucketError> {
        if !bucket.exists().await? {
            return Err(CreateBucketError::NoAccess(bucket.bucket_args()));
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

pub async fn upload(bucket: &WrappedBucket, image: Arc<[u8]>) -> String {
    let (encoded_bytes, hash_string) = tokio::task::spawn_blocking(move || {
        let image = image::load_from_memory(&image).unwrap();
        let mut result = BufWriter::new(Cursor::new(Vec::new()));

        image.write_to(&mut result, ImageFormat::Png).unwrap();

        let hasher = HasherConfig::new().to_hasher();

        (
            result.into_inner().unwrap().into_inner(),
            hasher.hash_image(&image).to_base64(),
        )
    })
    .await
    .unwrap();

    let prefix = bucket.prefix.as_str();
    let key = format!("{prefix}{hash_string}.png");

    bucket
        .bucket
        .put_object(key.clone(), encoded_bytes.into())
        .await
        .unwrap();

    bucket.bucket.object_url(key)
}
