use scalar_cms::{
    Document,
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
    ) -> Result<(), sqlx::Error> {
        let data = query!("");

        Ok(())
    }
}
