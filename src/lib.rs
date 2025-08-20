pub mod helper;
pub mod middleware;
mod resources;

pub use resources::accounts::user_store::SeaOrmUserStore;

use actix_files::Files;
use actix_session::{SessionMiddleware, storage::RedisSessionStore, config::PersistentSession};
use actix_web::{
    App, HttpServer, 
    middleware::from_fn,
    cookie::{Key, time}, web::Data
};
use actix_passport::prelude::*;

    
use sea_orm::DatabaseConnection;

use tera::Tera;
use std::sync::{RwLock, LazyLock};

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    RwLock::new(Tera::new("templates/**/*").expect("Failed to create Tera instance"))
});

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn create_server(state: Data<AppState>, redis_url: String, actix_passport: ActixPassport) -> std::io::Result<()> {
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(&redis_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(resources::guest_page)
            .service(resources::dashboard)
            .configure(resources::accounts::routes)
            .configure(|cfg| actix_passport.configure_routes(cfg, RouteConfig::default()))
            .service(Files::new("/public", "./public").prefer_utf8(true))
            .wrap(SessionMiddleware::builder(
                    redis_store.clone(),
                    secret_key.clone(),
                ).session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::days(5)),
                ).build()
            )
            // Middleware to reload templates. Comment this out in production.
            .wrap(from_fn(middleware::reload_templates)) 
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}