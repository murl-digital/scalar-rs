use scalar_cms::{
    DateTime, Document, Item, Utc,
    db::{Credentials, User},
};
use sqlx::{SqlitePool, query, query_as};

use crate::DatabaseInner;
pub type Pool = SqlitePool;

pub async fn migrate(pool: &Pool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}

impl DatabaseInner for SqlitePool {
    async fn get_password_hash(&self, email: &str) -> Result<Option<String>, sqlx::Error> {
        let test = query!(
            "SELECT password_hash FROM sc__users WHERE email = $1",
            email
        )
        .fetch_optional(self)
        .await?;

        Ok(test.map(|r| r.password_hash))
    }

    async fn get_user(&self, email: &str) -> Result<scalar_cms::db::User, sqlx::Error> {
        let data = query!(
            "SELECT email, name, admin from sc__users WHERE email = $1",
            email
        )
        .fetch_one(self)
        .await?;

        Ok(User::new(
            data.email,
            data.name,
            String::default(),
            data.admin,
        ))
    }

    async fn draft<D: Document>(
        &self,
        id: &str,
        data: serde_json::Value,
    ) -> Result<Item<serde_json::Value>, sqlx::Error> {
        let mut transcation = self.begin().await?;
        let now = Utc::now();

        let published_at = query!(
            r#"INSERT INTO sc__meta(doc, id, created_at, modified_at)
            VALUES($1, $2, $3, $3)
            ON CONFLICT(id)
            DO
               UPDATE
               SET modified_at = $3
            RETURNING published_at as 'published_at: DateTime<Utc>'"#,
            D::IDENTIFIER,
            id,
            now
        )
        .fetch_one(&mut *transcation)
        .await?
        .published_at;

        query!(
            r#"INSERT INTO sc__drafts(doc, id, inner)
            VALUES($1, $2, $3)
            ON CONFLICT(id)
            DO
               UPDATE
               SET inner = $3"#,
            D::IDENTIFIER,
            id,
            data
        )
        .execute(&mut *transcation)
        .await?;

        transcation.commit().await?;

        Ok(Item {
            id: id.into(),
            created_at: now,
            modified_at: now,
            published_at,
            inner: data,
        })
    }

    async fn get_all<D: Document>(&self) -> Result<Vec<Item<serde_json::Value>>, sqlx::Error> {
        type InnerItem = Item<serde_json::Value>;
        query_as!(
            InnerItem,
            r#"SELECT
                sc__meta.id as 'id!',
                sc__meta.created_at as 'created_at!: DateTime<Utc>',
                sc__meta.modified_at as 'modified_at!: DateTime<Utc>',
                sc__meta.published_at as 'published_at: DateTime<Utc>',
                (
                    CASE WHEN sc__drafts.inner IS NULL
                        THEN sc__published.inner
                        ELSE sc__drafts.inner
                    END
                ) as 'inner!: serde_json::Value'
                FROM sc__meta
                FULL OUTER JOIN sc__drafts ON sc__meta.id = sc__drafts.id
                FULL OUTER JOIN sc__published ON sc__meta.id = sc__published.id
                WHERE sc__meta.doc = $1
            "#,
            D::IDENTIFIER
        )
        .fetch_all(self)
        .await
    }

    async fn get_by_id<D: Document>(
        &self,
        id: &str,
    ) -> Result<Option<Item<serde_json::Value>>, sqlx::Error> {
        type InnerItem = Item<serde_json::Value>;
        query_as!(
            InnerItem,
            r#"SELECT
                sc__meta.id as 'id!',
                sc__meta.created_at as 'created_at!: DateTime<Utc>',
                sc__meta.modified_at as 'modified_at!: DateTime<Utc>',
                sc__meta.published_at as 'published_at: DateTime<Utc>',
                (
                    CASE WHEN sc__drafts.inner IS NULL
                        THEN sc__published.inner
                        ELSE sc__drafts.inner
                    END
                ) as 'inner!: serde_json::Value'
                FROM sc__meta
                FULL OUTER JOIN sc__drafts ON sc__meta.id = sc__drafts.id
                FULL OUTER JOIN sc__published ON sc__meta.id = sc__published.id
                WHERE sc__meta.doc = $1 AND sc__meta.id = $2
            "#,
            D::IDENTIFIER,
            id
        )
        .fetch_optional(self)
        .await
    }
}
