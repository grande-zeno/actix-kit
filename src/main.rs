use actix_web::web::Data;
use actixkit::{AppState, SeaOrmUserStore};
use actix_passport::prelude::*;
use sea_orm::{ConnectOptions, Database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let redis_url = dotenvy::var("REDIS_URL").unwrap();
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let google_client_id = dotenvy::var("GOOGLE_CLIENT_ID").unwrap();
    let google_client_secret = dotenvy::var("GOOGLE_CLIENT_SECRET").unwrap();

    let mut opt = ConnectOptions::new(&database_url);
    opt.min_connections(5);
    let db = Database::connect(opt).await.unwrap();

    let state = Data::new(AppState { db: db.clone() });

    let google_provider = GoogleOAuthProvider::new(google_client_id, google_client_secret);
    let actix_passport = ActixPassportBuilder::new(SeaOrmUserStore::new(db))
        .enable_oauth(vec![Box::new(google_provider)])
        .build();

    actixkit::create_server(state, redis_url, actix_passport).await
}
