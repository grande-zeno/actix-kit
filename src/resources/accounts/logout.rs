use actix_web::{HttpResponse, Responder, post};

#[post("/logout")]
pub async fn logout() -> impl Responder {

    HttpResponse::SeeOther()
        .append_header(("HX-REDIRECT", "/login"))
        .finish()
}
