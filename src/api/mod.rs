mod health;
mod user;

use crate::api::health::health;
use crate::api::user::{create_user, user_service};
use crate::repository::user::Repository;
use crate::{PostgresRepository};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use std::ptr::replace;
use std::sync::Arc;

pub async fn serve(
    url: &str,
    repo: PostgresRepository,
) -> std::io::Result<()> {
    let repo = Data::new(repo);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
           // .wrap(RequestTracing::new())
            .service(web::scope("/health").route("", web::get().to(health)))
            .service(user_service(&repo))
    })
    .bind((url, 8080))?
    .run()
    .await
}
