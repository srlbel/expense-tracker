pub mod dtos;
pub mod handlers;

use actix_files::{Files, NamedFile};
use actix_web::{HttpRequest, HttpResponse, Result, web};

async fn not_found(req: HttpRequest) -> Result<HttpResponse> {
    let file = NamedFile::open("./apps/client/dist/404.html")?;
    Ok(file
        .use_last_modified(true)
        .set_status_code(actix_web::http::StatusCode::NOT_FOUND) // TODO: Search a form to use a non deprecated function
        .into_response(&req))
}

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

    cfg.service(
        Files::new("/", "./apps/client/dist")
            .index_file("index.html")
            .use_last_modified(true)
            .default_handler(web::to(not_found)),
    );
}
