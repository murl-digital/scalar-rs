use std::fmt::Debug;
use std::{error::Error, sync::Arc};

use chrono::{DateTime, Utc};
use openidconnect::{AdditionalClaims, GenderClaim, IdTokenClaims};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::{validations::Valid, Document, Item};

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

impl Credentials {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("email", &self.email)
            .field("password", &"<REDACTED>")
            .finish()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    email: Arc<str>,
    name: Arc<str>,
    profile_picture_url: Arc<str>,
    admin: bool,
}

impl User {
    pub fn new(
        email: impl Into<String>,
        name: impl Into<String>,
        profile_picture_url: impl Into<String>,
        admin: bool,
    ) -> Self {
        Self {
            email: email.into().into(),
            name: name.into().into(),
            profile_picture_url: profile_picture_url.into().into(),
            admin,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn profile_picture_url(&self) -> &str {
        &self.profile_picture_url
    }

    pub fn admin(&self) -> bool {
        self.admin
    }
}

#[trait_variant::make(Send + Sized)]
pub trait DatabaseFactory {
    type Error: Error;
    type Connection: DatabaseConnection + Sync;

    async fn init(&self) -> Result<Self::Connection, Self::Error>;
    async fn init_system(&self) -> Result<Self::Connection, Self::Error>;
}

#[derive(Debug)]
pub struct Authenticated<DB: DatabaseConnection> {
    conn: DB,
    user: User,
}

impl<DB: DatabaseConnection> Authenticated<DB> {
    pub async fn authenticate(
        conn: DB,
        token: &str,
    ) -> Result<Self, AuthenticationError<DB::Error>> {
        Ok(Self {
            user: conn.authenticate(token).await?,
            conn,
        })
    }

    pub fn me(&self) -> User {
        self.user.clone()
    }

    pub fn inner(&self) -> &DB {
        &self.conn
    }
}

#[trait_variant::make(Send + Sized)]
pub trait DatabaseConnection {
    type Error: Error;

    async fn authenticate(&self, jwt: &str) -> Result<User, AuthenticationError<Self::Error>>;
    async fn signin(
        &self,
        credentials: Credentials,
    ) -> Result<String, AuthenticationError<Self::Error>>;
    async fn signin_oidc<AC: AdditionalClaims + Send, GC: GenderClaim + Send>(
        &self,
        user_info: IdTokenClaims<AC, GC>,
    ) -> Result<String, AuthenticationError<Self::Error>>;

    async fn draft<D: Document + Send>(
        conn: &Authenticated<Self>,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, Self::Error>;
    async fn delete_draft<D: Document + Send + DeserializeOwned>(
        conn: &Authenticated<Self>,
        id: &str,
    ) -> Result<Item<serde_json::Value>, Self::Error>;

    async fn publish<D: Document + Send + Serialize + DeserializeOwned + 'static>(
        conn: &Authenticated<Self>,
        id: &str,
        publish_at: Option<DateTime<Utc>>,
        data: Valid<D>,
    ) -> Result<Item<D>, Self::Error>;

    async fn put<D: Document + Serialize + DeserializeOwned + Send + Debug + 'static>(
        conn: &Authenticated<Self>,
        item: Item<D>,
    ) -> Result<Item<D>, Self::Error>;
    async fn delete<D: Document + Send + Debug>(
        conn: &Authenticated<Self>,
        id: &str,
    ) -> Result<Item<D>, Self::Error>;
    async fn get_all<D: Document + DeserializeOwned + Send>(
        &self,
    ) -> Result<Vec<Item<serde_json::Value>>, Self::Error>;
    async fn get_by_id<D: Document + DeserializeOwned + Send>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, Self::Error>;
}
