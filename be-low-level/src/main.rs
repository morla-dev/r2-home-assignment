#[cfg(test)]
mod tests;
mod auth;
mod error;
mod luck;
mod middleware;
mod model;
mod token;

use actix_web::{middleware::Logger, web, App, HttpServer};
use token::TokenStore;
use luck::LuckStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let token_store = web::Data::new(TokenStore::new());
    let luck_store = web::Data::new(LuckStore::new());

    log::info!("Starting be-low-level on port 4000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(token_store.clone())
            .app_data(luck_store.clone())
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _| {
                        let response = error::AppError::BadRequest(err.to_string()).into();
                        actix_web::error::InternalError::from_response(err, response).into()
                    })
            )
            .route("/api/login", web::post().to(auth::login))
            .route("/api/logout", web::post().to(auth::logout))
            .route("/api/try_luck", web::post().to(auth::try_luck))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
