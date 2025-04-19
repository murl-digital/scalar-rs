use std::{borrow::Cow, fmt::Debug, ops::Deref};

use scalar::{
    db::{AuthenticationError, Credentials, DatabaseFactory, User},
    validations::Valid,
    DateTime, Document, Item, Utc,
};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use surrealdb::{
    error::{Api, Db},
    opt::{
        auth::{Record, Root},
        IntoEndpoint,
    },
    sql::Thing,
    Connection, Error, Surreal,
};

fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(t.id.to_raw())
}

#[derive(Clone, Debug)]
pub struct SurrealConnection<C: Connection + Debug> {
    namespace: String,
    db: String,
    inner: Surreal<C>,
}

impl<C: Connection + Debug> Deref for SurrealConnection<C> {
    type Target = Surreal<C>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurrealItem<D> {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub inner: D,
}

impl<D> From<SurrealItem<D>> for Item<D> {
    fn from(item: SurrealItem<D>) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            modified_at: item.modified_at,
            published_at: item.published_at,
            inner: item.inner,
        }
    }
}

impl<D: Debug> From<Item<D>> for SurrealItem<D> {
    fn from(value: Item<D>) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            modified_at: value.modified_at,
            published_at: value.published_at,
            inner: value.inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurrealStore<C: Connection> {
    namespace: String,
    db: String,
    inner_instance: Surreal<C>,
}

impl<C: Connection> SurrealStore<C> {
    pub async fn new<S, P: IntoEndpoint<S, Client = C>>(
        address: P,
        namespace: String,
        db: String,
    ) -> Result<Self, surrealdb::Error> {
        Ok(Self {
            namespace,
            db,
            inner_instance: Surreal::new(address).await?,
        })
    }
}

impl<C: Connection + Clone + Debug> DatabaseFactory for SurrealStore<C> {
    type Error = surrealdb::Error;

    type Connection = SurrealConnection<C>;

    #[tracing::instrument(level = "debug", err)]
    async fn init(&self) -> Result<Self::Connection, Self::Error> {
        let inner = self.inner_instance.clone();

        inner.use_ns(&self.namespace).await?;
        inner.use_db(&self.db).await?;

        Ok(SurrealConnection {
            namespace: self.namespace.clone(),
            db: self.namespace.clone(),
            inner,
        })
    }

    #[tracing::instrument(level = "debug", err)]
    async fn init_system(&self) -> Result<Self::Connection, Self::Error> {
        let inner = self.inner_instance.clone();

        inner.use_ns(&self.namespace).await?;
        inner.use_db(&self.db).await?;

        inner
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        Ok(SurrealConnection {
            namespace: self.namespace.clone(),
            db: self.namespace.clone(),
            inner,
        })
    }
}

impl<C: Connection + Debug> scalar::DatabaseConnection for SurrealConnection<C> {
    type Error = surrealdb::Error;

    #[tracing::instrument(level = "debug", err)]
    async fn authenticate(&self, jwt: &str) -> Result<(), AuthenticationError<Self::Error>> {
        self.inner.authenticate(jwt).await.map_err(|e| {
            println!("{e:?}");
            match e {
                Error::Api(Api::Query(_)) => AuthenticationError::BadToken,
                Error::Db(Db::InvalidAuth | Db::ExpiredToken | Db::ExpiredSession) => {
                    AuthenticationError::BadToken
                }
                _ => e.into(),
            }
        })?;

        Ok(())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn signin(
        &self,
        credentials: Credentials,
    ) -> Result<String, AuthenticationError<Self::Error>> {
        let result = self
            .inner
            .signin(Record {
                namespace: &self.namespace,
                database: &self.db,
                access: "sc__editor",
                params: credentials,
            })
            .await
            .map_err(|e| match e {
                Error::Api(Api::Query(_)) => AuthenticationError::BadCredentials,
                Error::Db(Db::InvalidAuth) => AuthenticationError::BadCredentials,
                _ => e.into(),
            })?;

        Ok(result.into_insecure_token())
    }

    async fn me(&self) -> Result<User, Self::Error> {
        let user: Option<User> = self
            .query("SELECT *, crypto::sha256(email) as gravatar_hash OMIT id, password FROM $auth")
            .await?
            .take(0)?;

        Ok(user.expect("user should be authenticated when this is called"))
    }

    #[tracing::instrument(level = "debug", err)]
    async fn draft<D: Document + Send>(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, Self::Error> {
        #[derive(Serialize)]
        struct Bindings<'a> {
            doc: Cow<'a, str>,
            id: Cow<'a, str>,
            inner: serde_json::Value,
        }

        let mut result = self
            .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
            .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
            .query("UPSERT $draft_id SET inner = $inner")
            .query("UPSERT $meta_id SET draft = $draft_id, modified_at = time::now()")
            .query(
                "SELECT
                id,
                created_at,
                modified_at,
                IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
                published.published_at AS published_at
            FROM $meta_id
            FETCH draft, published",
            )
            .bind(Bindings {
                doc: D::identifier().into(),
                id: id.to_owned().into(),
                inner: data,
            })
            .await?;

        let thingy: Option<SurrealItem<serde_json::Value>> =
            result.take(4).expect("this should always succeed");

        Ok(thingy
            .expect("this option should always return something")
            .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn delete_draft<D: Document + Send + DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<Item<serde_json::Value>, Self::Error> {
        #[derive(Serialize)]
        struct Bindings<'a> {
            doc: Cow<'a, str>,
            id: Cow<'a, str>,
        }

        //TODO: VERY BAD!!!!
        let pre_delete = self.get_by_id::<D>(id).await?.unwrap();

        let _ = self
            .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
            .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
            .query("DELETE $draft_id")
            .query("DELETE $meta_id WHERE published IS NONE")
            .bind(Bindings {
                doc: D::identifier().into(),
                id: id.to_owned().into(),
            })
            .await?;

        Ok(pre_delete)
    }

    async fn publish<D: Document + Send + Serialize + DeserializeOwned + 'static>(
        &self,
        id: &str,
        publish_at: Option<DateTime<Utc>>,
        data: Valid<D>,
    ) -> Result<Item<D>, Self::Error> {
        #[derive(Serialize)]
        struct Bindings<'a, D> {
            doc: Cow<'a, str>,
            id: Cow<'a, str>,
            publish_at: Option<DateTime<Utc>>,
            inner: D,
        }

        let mut result = self
            .query("LET $published_id = type::thing($doc, $id)")
            .query("LET $draft_id = type::thing(string::concat($doc, '_draft'), $id)")
            .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
            .query("UPSERT $published_id SET inner = $inner, published_at = $published_at")
            .query("UPSERT $meta_id SET published = $published_id, modified_at = time::now(), draft = NONE")
            .query("DELETE $draft_id")
            .query(
                "SELECT
                id,
                created_at,
                modified_at,
                IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
                published.published_at AS published_at
            FROM $meta_id
            FETCH draft, published",
            ).bind(Bindings::<D> {
                doc: D::identifier().into(),
                id: id.to_owned().into(),
                publish_at,
                inner: data.inner()
            }).await?;

        let thingy: Option<SurrealItem<D>> = result.take(6).expect("this should always succeed");

        Ok(thingy
            .expect("this option should always return something")
            .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn put<D: Document + Serialize + DeserializeOwned + Send + Debug + 'static>(
        &self,
        item: Item<D>,
    ) -> Result<Item<D>, Self::Error> {
        let updated_thingy: Option<SurrealItem<D>> = self
            .upsert((D::identifier(), item.id.to_owned()))
            .content(SurrealItem::<D>::from(item))
            .await?;

        Ok(updated_thingy
            .expect("surreal should return data regardless")
            .into())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn delete<D: Document + Send + Debug>(&self, id: &str) -> Result<Item<D>, Self::Error> {
        todo!()
    }

    #[tracing::instrument(level = "debug", err)]
    async fn get_all<D: Document + DeserializeOwned + Send>(
        &self,
    ) -> Result<Vec<Item<serde_json::Value>>, Self::Error> {
        let result = self
            .query(
                "SELECT
                id,
                created_at,
                modified_at,
                IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
                published.published_at AS published_at
            FROM type::table(string::concat($doc, '_meta'))
            FETCH draft, published",
            )
            .bind(("doc", D::identifier()))
            .await?
            .take::<Vec<SurrealItem<serde_json::Value>>>(0)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    #[tracing::instrument(level = "debug", err)]
    async fn get_by_id<D: Document + DeserializeOwned + Send>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, Self::Error> {
        #[derive(Serialize)]
        struct Bindings<'a> {
            doc: Cow<'a, str>,
            id: Cow<'a, str>,
        }

        Ok(self
            .query("LET $meta_id = type::thing(string::concat($doc, '_meta'), $id)")
            .query(
                "SELECT
                id,
                created_at,
                modified_at,
                IF draft IS NOT NONE THEN draft.inner ELSE published.inner END AS inner,
                published.published_at AS published_at
            FROM $meta_id
            FETCH draft, published",
            )
            .bind(Bindings {
                doc: D::identifier().into(),
                id: id.to_owned().into(),
            })
            .await?
            .take::<Option<SurrealItem<serde_json::Value>>>(1)?
            .map(Into::into))
    }
}

impl<C: Connection + Debug> SurrealConnection<C> {
    pub async fn init_doc<D: Document>(&self) {
        let published_table = D::identifier();
        let draft_table = format!("{published_table}_draft");
        let meta_table = format!("{published_table}_meta");
        self
            // published documents
            .query(format!("DEFINE TABLE OVERWRITE {published_table} SCHEMAFULL PERMISSIONS FOR select WHERE true FOR create, update, delete WHERE $auth.id IS NOT NONE"))
            .query(format!("DEFINE FIELD IF NOT EXISTS published_at ON {published_table} TYPE option<datetime>"))
            .query(format!("DEFINE FIELD IF NOT EXISTS inner ON {published_table} FLEXIBLE TYPE object"))
            // drafts
            .query(format!("DEFINE TABLE OVERWRITE {draft_table} SCHEMAFULL PERMISSIONS FOR select, create, update, delete WHERE $auth.id IS NOT NONE"))
            .query(format!("DEFINE FIELD IF NOT EXISTS inner ON {draft_table} FLEXIBLE TYPE object"))
            // meta table
            .query(format!("DEFINE TABLE OVERWRITE {meta_table} SCHEMAFULL PERMISSIONS FOR select, create, update, delete WHERE $auth.id IS NOT NONE"))
            .query(format!("DEFINE FIELD IF NOT EXISTS created_at ON {meta_table} TYPE datetime DEFAULT time::now()"))
            .query(format!("DEFINE FIELD IF NOT EXISTS modified_at ON {meta_table} TYPE datetime"))
            .query(format!("DEFINE FIELD IF NOT EXISTS draft ON {meta_table} TYPE option<record<{draft_table}>>"))
            .query(format!("DEFINE FIELD IF NOT EXISTS published ON {meta_table} TYPE option<record<{published_table}>>"))
            .query(format!("DEFINE FUNCTION fn::{published_table}_public() {{ RETURN SELECT * FROM {published_table} WHERE published_at < time::now() }}"))
            .await
            .unwrap_or_else(|e| panic!("setting up tables for {published_table} failed: {e}"));
    }

    pub async fn init_auth(&self) {
        self
            .query("DEFINE TABLE OVERWRITE sc__editor SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id OR $auth.admin = true FOR create WHERE $auth.admin = true")
            .query("DEFINE FIELD IF NOT EXISTS name ON sc__editor TYPE string")
            .query("DEFINE FIELD IF NOT EXISTS email ON sc__editor TYPE string ASSERT string::is::email($value)")
            .query("DEFINE FIELD IF NOT EXISTS password ON sc__editor TYPE string")
            .query("DEFINE FIELD IF NOT EXISTS admin ON sc__editor TYPE bool")
            .query("DEFINE INDEX email ON sc__editor FIELDS email UNIQUE")
            .query("
            DEFINE ACCESS OVERWRITE sc__editor ON DATABASE TYPE RECORD
            SIGNIN (
                SELECT * FROM sc__editor WHERE email = $email AND crypto::argon2::compare(password, $password)
            )
        ").await.expect("auth setup failed");
    }
}

// TODO: unit tests

#[macro_export]
macro_rules! doc_init {
    ($db:ident, $doc:ty) => {
        $db.init_doc::<$doc>().await;
    };
    ($db:ident, $doc:ty, $($docs:ty),+) => {
        ::scalar_surreal::doc_init!($db, $doc);
        ::scalar_surreal::doc_init!($db, $($docs),+);
    }
}

#[macro_export]
macro_rules! init {
    ($db:ident, $($docs:ty),+) => {
        $db.init_auth().await;
        ::scalar_surreal::doc_init!($db, $($docs),+);
    };
}
