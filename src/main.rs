use std::sync::Arc;
use crate::repository::user::PostgresRepository;

mod repository;
mod domain;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "postgres://postgres:somePassword@localhost:5432/postgres";
    let repository = PostgresRepository::new_pool(url).await.unwrap();
    api::serve("localhost", repository).await
}
