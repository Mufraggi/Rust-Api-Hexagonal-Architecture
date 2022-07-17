use std::sync::Arc;
use crate::repository::user::PostgresRepository;

mod repository;
mod domain;

#[tokio::main]
 async fn main() {
  /*  let url = "postgres://postgres:somePassword@localhost:5432/postgres";
     let repository = PostgresRepository::new_pool(url).await.unwrap();
    let repo =Arc::new(repository);*/
}
