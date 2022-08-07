use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use chrono::NaiveDate;

use crate::PostgresRepository;
use serde::Deserialize;
use serde::Serialize;
use crate::domain::list_users;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}

pub async fn serve(repo: Data<PostgresRepository>) -> impl Responder {

    match list_users::execute(repo).await {
        Ok(users) => {
            let res:Vec<Response> = users.into_iter().map(|x| {
                Response {
                    id: x.id,
                    first_name: x.first_name,
                    last_name: x.last_name,
                    birthday_date: x.birthday_date,
                    city: x.city,
                }
            }
            ).collect();
            HttpResponse::Created()
                .content_type("application/json")
                .body(serde_json::to_string(&res).unwrap())
        } ,
        Err(list_users::Error::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(list_users::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(list_users::Error::Unknown) => HttpResponse::Conflict().finish(),
    }
}