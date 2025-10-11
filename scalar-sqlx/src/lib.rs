use std::fmt::Debug;

use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash};
use rusty_paseto::{
    core::{Local, PasetoSymmetricKey, V4},
    prelude::{CustomClaim, PasetoBuilder},
};
use scalar_cms::{
    DatabaseConnection, DateTime, Document, Item, Utc,
    db::{AuthenticationError, Credentials, User},
    validations::Valid,
};
use sqlx::{Database, Pool, query};
use thiserror::Error;

#[cfg(feature = "sqlite")]
pub mod sqlite;

pub(crate) trait DatabaseInner {
    fn get_password_hash(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<Option<String>, sqlx::Error>> + Send;

    fn get_user(&self, email: &str) -> impl Future<Output = Result<User, sqlx::Error>> + Send;
}

pub struct Connection<DB: DatabaseInner> {
    paseto_key: PasetoSymmetricKey<V4, Local>,
    inner: DB,
}

impl<DB: DatabaseInner + Debug> Debug for Connection<DB> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("paseto_key", &"<redacted>")
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("password error: {0}")]
    Password(#[from] password_hash::Error),
}

impl<DB: DatabaseInner + Debug + Send + Sync> DatabaseConnection for Connection<DB> {
    type Error = Error;

    #[tracing::instrument(level = "debug", err)]
    async fn authenticate(&mut self, jwt: &str) -> Result<(), AuthenticationError<Self::Error>> {
        // self.inner.authenticate(jwt).await.map_err(|e| {
        //     println!("{e:?}");
        //     match e {
        //         Error::Api(Api::Query(_)) => AuthenticationError::BadToken,
        //         Error::Db(Db::InvalidAuth | Db::ExpiredToken | Db::ExpiredSession) => {
        //             AuthenticationError::BadToken
        //         }
        //         _ => e.into(),
        //     }
        // })?;

        Ok(())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn signin(
        &self,
        credentials: Credentials,
    ) -> Result<String, AuthenticationError<Self::Error>> {
        let password_hash = self
            .inner
            .get_password_hash(credentials.email())
            .await
            .map_err(Error::from)?
            .ok_or(AuthenticationError::BadCredentials)?;

        let parsed_hash = PasswordHash::new(&password_hash).unwrap();
        Argon2::default()
            .verify_password(credentials.password().as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                password_hash::Error::Password => AuthenticationError::BadCredentials,
                e => Error::from(e).into(),
            })?;

        let user = self
            .inner
            .get_user(credentials.email())
            .await
            .map_err(Error::from)?;

        let token = PasetoBuilder::<_, Local>::default()
            .set_claim(CustomClaim::try_from(("email", credentials.email())).unwrap())
            .set_claim(CustomClaim::try_from(("name", user.name())).unwrap())
            .set_claim(CustomClaim::try_from(("admin", user.admin())).unwrap())
            .build(&self.paseto_key)
            .unwrap();

        Ok(token)
    }

    async fn me(&self) -> Result<User, Self::Error> {
        todo!()
        // let user: Option<User> = self
        //     .query("SELECT *, crypto::sha256(email) as gravatar_hash OMIT id, password FROM $auth")
        //     .await?
        //     .take(0)?;

        // Ok(user.expect("user should be authenticated when this is called"))
    }

    #[tracing::instrument(level = "debug", err)]
    async fn draft<D: Document + Send>(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, Self::Error> {
        todo!()
        // #[derive(Serialize)]
        // struct Bindings<'a> {
        //     doc: Cow<'a, str>,
        //     id: Cow<'a, str>,
        //     inner: serde_json::Value,
        // }

        // let mut result = self
        //     .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
        //     .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
        //     .query("UPSERT $draft_id SET inner = $inner")
        //     .query("UPSERT $meta_id SET draft = $draft_id, modified_at = time::now()")
        //     .query(
        //         "SELECT
        //         id,
        //         created_at,
        //         modified_at,
        //         IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
        //         published.published_at AS published_at
        //     FROM $meta_id
        //     FETCH draft, published",
        //     )
        //     .bind(Bindings {
        //         doc: D::identifier().into(),
        //         id: id.to_owned().into(),
        //         inner: data,
        //     })
        //     .await?;

        // let thingy: Option<SurrealItem<serde_json::Value>> =
        //     result.take(4).expect("this should always succeed");

        // Ok(thingy
        //     .expect("this option should always return something")
        //     .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn delete_draft<D: Document + Send>(
        &self,
        id: &str,
    ) -> Result<Item<serde_json::Value>, Self::Error> {
        todo!();
        // #[derive(Serialize)]
        // struct Bindings<'a> {
        //     doc: Cow<'a, str>,
        //     id: Cow<'a, str>,
        // }

        // //TODO: VERY BAD!!!!
        // let pre_delete = self.get_by_id::<D>(id).await?.unwrap();

        // let _ = self
        //     .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
        //     .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
        //     .query("DELETE $draft_id")
        //     .query("DELETE $meta_id WHERE published IS NONE")
        //     .bind(Bindings {
        //         doc: D::identifier().into(),
        //         id: id.to_owned().into(),
        //     })
        //     .await?;

        // Ok(pre_delete)
    }

    async fn publish<D: Document + Send + 'static>(
        &self,
        id: &str,
        publish_at: Option<DateTime<Utc>>,
        data: Valid<D>,
    ) -> Result<Item<D>, Self::Error> {
        todo!()
        // #[derive(Serialize)]
        // struct Bindings<'a, D> {
        //     doc: Cow<'a, str>,
        //     id: Cow<'a, str>,
        //     publish_at: Option<DateTime<Utc>>,
        //     inner: D,
        // }

        // let mut result = self
        //     .query("LET $published_id = type::thing($doc, $id)")
        //     .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
        //     .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
        //     .query("UPSERT $published_id SET inner = $inner, published_at = $published_at")
        //     .query("UPSERT $meta_id SET published = $published_id, modified_at = time::now(), draft = NONE")
        //     .query("DELETE $draft_id")
        //     .query(
        //         "SELECT
        //         id,
        //         created_at,
        //         modified_at,
        //         IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
        //         published.published_at AS published_at
        //     FROM $meta_id
        //     FETCH draft, published",
        //     ).bind(Bindings::<D> {
        //         doc: D::identifier().into(),
        //         id: id.to_owned().into(),
        //         publish_at,
        //         inner: data.inner()
        //     }).await?;

        // let thingy: Option<SurrealItem<D>> = result.take(6).expect("this should always succeed");

        // Ok(thingy
        //     .expect("this option should always return something")
        //     .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn put<D: Document + Send + Debug + 'static>(
        &self,
        item: Item<D>,
    ) -> Result<Item<D>, Self::Error> {
        todo!()
        // let updated_thingy: Option<SurrealItem<D>> = self
        //     .upsert((D::identifier(), item.id.to_owned()))
        //     .content(SurrealItem::<D>::from(item))
        //     .await?;

        // Ok(updated_thingy
        //     .expect("surreal should return data regardless")
        //     .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn delete<D: Document + Send + Debug>(&self, id: &str) -> Result<Item<D>, Self::Error> {
        todo!()
    }

    #[tracing::instrument(level = "debug", err)]
    async fn get_all<D: Document + Send>(
        &self,
    ) -> Result<Vec<Item<serde_json::Value>>, Self::Error> {
        todo!()
        // let result = self
        //     .query(
        //         "SELECT
        //         id,
        //         created_at,
        //         modified_at,
        //         IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
        //         published.published_at AS published_at
        //     FROM type::table(string::concat($doc, '_meta'))
        //     FETCH draft, published",
        //     )
        //     .bind(("doc", D::identifier()))
        //     .await?
        //     .take::<Vec<SurrealItem<serde_json::Value>>>(0)?;

        // Ok(result.into_iter().map(Into::into).collect())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn get_by_id<D: Document + Send>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, Self::Error> {
        todo!()
        // #[derive(Serialize)]
        // struct Bindings<'a> {
        //     doc: Cow<'a, str>,
        //     id: Cow<'a, str>,
        // }

        // Ok(self
        //     .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
        //     .query(
        //         "SELECT
        //         id,
        //         created_at,
        //         modified_at,
        //         IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
        //         published.published_at AS published_at
        //     FROM $meta_id
        //     FETCH draft, published",
        //     )
        //     .bind(Bindings {
        //         doc: D::identifier().into(),
        //         id: id.to_owned().into(),
        //     })
        //     .await?
        //     .take::<Option<SurrealItem<serde_json::Value>>>(1)?
        //     .map(Into::into))
    }
}
