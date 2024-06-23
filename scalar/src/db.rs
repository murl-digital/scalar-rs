use async_trait::async_trait;
use serde::Serialize;

use crate::{Document, Item};

#[async_trait]
pub trait DB {
    async fn create<D: Document + Serialize + Send>(&self, item: D) -> Result<Item<D>, ()>;
    async fn update<D: Document + Serialize + Send>(&self, item: Item<D>) -> Result<Item<D>, ()>;
    async fn delete<D: Document + Send>(&self, item: Item<D>) -> Result<Item<D>, ()>;
}