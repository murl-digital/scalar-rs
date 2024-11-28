use serde::{de::DeserializeOwned, Serialize};

use crate::{Document, Item};

pub struct DatabaseError;
pub enum DocumentModifyError {
    NoSuchDocument,
    DatabaseError,
}

#[trait_variant::make(Send)]
pub trait DB {
    async fn draft<D: Document + Send>(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, DatabaseError>;
    async fn delete_draft<D: Document + Send>(
        &self,
        id: &str,
    ) -> Result<Item<serde_json::Value>, DatabaseError>;

    async fn put<D: Document + Serialize + DeserializeOwned + Send + 'static>(
        &self,
        item: Item<D>,
    ) -> Result<Item<D>, DatabaseError>;
    async fn delete<D: Document + Send>(&self, id: &str) -> Result<Item<D>, DatabaseError>;
    async fn get_all<D: Document + DeserializeOwned + Send>(
        &self,
    ) -> Result<Vec<Item<serde_json::Value>>, DatabaseError>;
    async fn get_by_id<D: Document + DeserializeOwned + Send>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, DatabaseError>;
}
