mod health;

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use crate::repository::user::Repository;
use crate::api::health::health;




pub async fn serve(url: &str, repo: Arc<dyn Repository>) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
            web::scope("/health")
                .route("", web::get().to(health))
        )
    }).bind((url, 8080))?
        .run().await
}