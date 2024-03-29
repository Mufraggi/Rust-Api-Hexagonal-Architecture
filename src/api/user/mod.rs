use std::sync::Arc;
use actix_web::{App, Scope, web};
use actix_web::dev::WebService;
use actix_web::web::{Data, ServiceConfig};
use crate::PostgresRepository;
use crate::repository::user::Repository;

pub mod create_user;
pub mod get_user;
pub mod delete_user;
pub mod list_users;
mod update_user;

pub fn user_service(repo: &Data<PostgresRepository>) -> Scope {
    web::scope("/user")
        .route("", web::post().to(create_user::serve))
        .route("/{id}", web::get().to(get_user::serve))
        .route("", web::get().to(list_users::serve))
        .route("/{id}", web::delete().to(delete_user::serve))
        .route("/{id}", web::put().to(update_user::serve))
        .app_data(repo.clone())
}