pub mod login;
pub mod user_store;
pub mod logout;

use actix_web::{web::ServiceConfig, HttpResponse};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(login::login_page)
        .service(auth_callback)
        .service(logout::logout);
}

#[actix_web::get("/auth/google/callback")]
async fn auth_callback() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header(("LOCATION", "/dashboard"))
        .finish()
}