use crate::domain::models::user_model::User;
use crate::interfaces::repositories::user_repository::UserRepository;
use uuid::Uuid;

pub struct UserService<T: UserRepository> {
    repository: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, anyhow::Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<User, anyhow::Error> {
        self.repository.create(user).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, anyhow::Error> {
        self.repository.find_all().await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool, anyhow::Error> {
        self.repository.delete(id).await
    }
}
