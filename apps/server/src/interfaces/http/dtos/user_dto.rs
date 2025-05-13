use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.created_at,
        }
    }
}
