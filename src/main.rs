use crate::config::config::get_config;
use crate::repository::user::PostgresRepository;
use crate::telemetry::telemetry_client::TelemetryClient;
//use crate::telemetry::telemetry_client::TelemetryClient;

mod repository;
mod domain;
mod api;
mod config;
mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();
    //let url = "postgres://postgres:somePassword@postgres:5432/postgres";
    //let url = "postgres://postgres:somePassword@localhost:5432/postgres";
    let repository = PostgresRepository::new_pool(&config.url_postgres).await.unwrap();
    //api::serve("localhost", repository).await
    let telemetry_client = TelemetryClient::init();
    api::serve(&config.url_domain, repository).await

}
