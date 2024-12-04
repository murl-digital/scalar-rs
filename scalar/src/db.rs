use std::error::Error;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::{Document, Item};

#[derive(Error, Debug)]
pub enum AuthenticationError<DE: Error> {
    #[error("Invalid token provided")]
    BadToken,
    #[error("Invalid credentials provided")]
    BadCredentials,
    #[error("Database error: {0}")]
    DatabaseError(#[from] DE),
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("email", &self.email)
            .field("password", &"<REDACTED>")
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    name: String,
    gravatar_hash: String,
    admin: bool,
}

#[trait_variant::make(Send + Sized)]
pub trait DatabaseFactory {
    type Error: Error;
    type Connection: DatabaseConnection + Sync + Clone;

    async fn init(&self) -> Result<Self::Connection, Self::Error>;
    async fn init_system(&self) -> Result<Self::Connection, Self::Error>;
}

#[trait_variant::make(Send + Sized)]
pub trait DatabaseConnection {
    type Error: Error;

    async fn authenticate(&self, jwt: &str) -> Result<(), AuthenticationError<Self::Error>>;
    async fn signin(
        &self,
        credentials: Credentials,
    ) -> Result<String, AuthenticationError<Self::Error>>;
    async fn me(&self) -> Result<User, Self::Error>;

    async fn draft<D: Document + Send>(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, Self::Error>;
    async fn delete_draft<D: Document + Send>(
        &self,
        id: &str,
    ) -> Result<Item<serde_json::Value>, Self::Error>;

    async fn put<D: Document + Serialize + DeserializeOwned + Send + Debug + 'static>(
        &self,
        item: Item<D>,
    ) -> Result<Item<D>, Self::Error>;
    async fn delete<D: Document + Send + Debug>(&self, id: &str) -> Result<Item<D>, Self::Error>;
    async fn get_all<D: Document + DeserializeOwned + Send>(
        &self,
    ) -> Result<Vec<Item<serde_json::Value>>, Self::Error>;
    async fn get_by_id<D: Document + DeserializeOwned + Send>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, Self::Error>;
}
