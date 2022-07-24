use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use chrono::NaiveDate;
use crate::repository::user::Repository;
use serde::Deserialize;
use serde::Serialize;
use crate::domain::create_user;
use crate::domain::create_user::Error;
use crate::PostgresRepository;


#[derive(Debug, Deserialize)]
pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}

#[derive(Debug, Serialize )]
pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}

pub async fn serve(repo: Data< PostgresRepository>, req: web::Json<Request>) -> impl Responder {
    let req = create_user::Request {
        first_name: req.0.first_name,
        last_name: req.0.last_name,
        birthday_date: req.0.birthday_date,
        city: req.0.city,
    };
    match create_user::execute(repo, req).await {
        Ok(create_user::Response {
               id,
               first_name,
               last_name,
               birthday_date,
               city
           }) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&Response { id, first_name, last_name, birthday_date, city }).unwrap()),
        Err(create_user::Error::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(create_user::Error::Conflict) => HttpResponse::Conflict().finish(),
        Err(create_user::Error::Unknown) => HttpResponse::Conflict().finish(),
    }
}