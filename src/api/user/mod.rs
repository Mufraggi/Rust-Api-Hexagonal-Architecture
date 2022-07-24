use std::sync::Arc;
use actix_web::{App, Scope, web};
use actix_web::dev::WebService;
use actix_web::web::{Data, ServiceConfig};
use crate::repository::user::Repository;

pub mod create_user;
/*pub fn user_service()  {
    web::service(
    web::scope("/user")
        .route("", web::post().to(create_user::serve)))
}*/