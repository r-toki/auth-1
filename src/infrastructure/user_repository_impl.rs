use crate::domain::user::{User, UserRepository};
use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> anyhow::Result<()> {
        query!(
            r#"
INSERT INTO users ( id, email, hashed_password, hashed_refresh_token, created_at, updated_at )
VALUES ( $1, $2, $3, $4, $5, $6 )
ON CONFLICT ( id )
DO UPDATE
SET email = $2, hashed_password = $3, hashed_refresh_token = $4, created_at = $5, updated_at = $6
            "#,
            user.id,
            user.email,
            user.hashed_password,
            user.hashed_refresh_token,
            user.created_at,
            user.updated_at
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        query!(
            r#"
DELETE FROM users
WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn find_one(&self, id: &str) -> anyhow::Result<User> {
        let user = query_as!(
            User,
            r#"
SELECT * FROM users
WHERE id = $1
        "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    async fn find_one_by_email(&self, email: &str) -> anyhow::Result<User> {
        let user = query_as!(
            User,
            r#"
SELECT * FROM users
WHERE email = $1
            "#,
            email
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
