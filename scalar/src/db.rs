use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Document, Item};

#[async_trait]
pub trait DB {
    async fn create<D: Document + Serialize + Send>(&self, item: D) -> Result<Item<D>, ()>;
    async fn update<D: Document + Serialize + Send>(&self, item: Item<D>) -> Result<Item<D>, ()>;
    async fn delete<D: Document + Send>(&self, item: Item<D>) -> Result<Item<D>, ()>;
    async fn get_all<D: Document + DeserializeOwned + Send>(&self) -> Result<Vec<Item<D>>, ()>;
    async fn get_by_id<D: Document + DeserializeOwned + Send>(&self, id: &str) -> Result<Option<Item<D>>, ()>;
}
