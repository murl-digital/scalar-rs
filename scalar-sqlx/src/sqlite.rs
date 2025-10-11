use scalar_cms::{
    DateTime, Document, Item, Utc,
    db::{Credentials, User},
};
use sqlx::{SqlitePool, query, query_as};

use crate::DatabaseInner;

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
}
