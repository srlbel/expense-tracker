use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::user_model::User;
use crate::interfaces::repositories::user_repository::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, anyhow::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User, anyhow::Error> {
        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, email, created_at, updated_at
            "#,
            user.id,
            user.username,
            user.email,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_user)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
