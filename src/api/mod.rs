mod health;
mod user;

use std::ptr::replace;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::repository::user::Repository;
use crate::api::health::health;
use crate::api::user::create_user;
use crate::PostgresRepository;


pub async fn serve(url: &str, repo: PostgresRepository) -> std::io::Result<()> {
    let repo = Data::new(repo);
    HttpServer::new(move|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/health")
                    .route("", web::get().to(health))
            ).service(
            web::scope("/user")
                .route("", web::post().to(create_user::serve)).app_data(repo.clone()))
    }).bind((url, 8080))?
        .run().await
}