use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error
};
use crate::TERA;

pub async fn reload_templates(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if let Ok(mut tera) = TERA.write() {
        if let Err(e) = tera.full_reload() {
            eprintln!("Failed to reload Tera templates: {}", e);
        }
    }

    next.call(req).await
    
}