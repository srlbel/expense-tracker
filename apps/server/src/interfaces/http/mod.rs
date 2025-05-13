pub mod dtos;
pub mod handlers;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/users")
                .service(handlers::user_handler::get_user)
                .service(handlers::user_handler::create_user)
                .service(handlers::user_handler::list_users)
                .service(handlers::user_handler::delete_user),
        ),
    );
}
