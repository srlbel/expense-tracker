use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::user_model::User;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<User>, anyhow::Error>;
    async fn create(&self, user: User) -> Result<User, anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, anyhow::Error>;
}
