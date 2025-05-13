use actix_web::{HttpResponse, Responder, delete, get, post, web};
use chrono::Utc;
use uuid::Uuid;

use crate::application::services::user_service::UserService;
use crate::domain::models::user_model::User;
use crate::infrastructure::repositories::user_repository::PostgresUserRepository;
use crate::interfaces::http::dtos::user_dto::{CreateUserRequest, UserResponse};

#[get("/{id}")]
pub async fn get_user(path: web::Path<Uuid>, db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let id = path.into_inner();
    let repository = PostgresUserRepository::new(db_pool.get_ref().clone());
    let service = UserService::new(repository);

    match service.get_user(id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponse::from(user)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn create_user(
    user_req: web::Json<CreateUserRequest>,
    db_pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let repository = PostgresUserRepository::new(db_pool.get_ref().clone());
    let service = UserService::new(repository);

    let user = User {
        id: Uuid::new_v4(),
        username: user_req.username.clone(),
        email: user_req.email.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match service.create_user(user).await {
        Ok(created_user) => HttpResponse::Created().json(UserResponse::from(created_user)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("")]
pub async fn list_users(db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repository = PostgresUserRepository::new(db_pool.get_ref().clone());
    let service = UserService::new(repository);

    match service.list_users().await {
        Ok(users) => {
            let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
pub async fn delete_user(
    path: web::Path<Uuid>,
    db_pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let repository = PostgresUserRepository::new(db_pool.get_ref().clone());
    let service = UserService::new(repository);

    match service.delete_user(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
