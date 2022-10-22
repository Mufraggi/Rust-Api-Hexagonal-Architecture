use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use crate::{domain, PostgresRepository};
use crate::domain::get_user;

#[derive(Debug, Deserialize, Serialize)]
pub struct Param {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}
#[tracing::instrument]
pub async fn serve(repo: Data<PostgresRepository>, path: web::Path<Param>) -> impl Responder {
    let req = get_user::Request { id: path.into_inner().id };
    match get_user::execute(repo, req).await {
        Ok(get_user::Response {
               id,
               first_name,
               last_name,
               birthday_date,
               city
           }) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&Response { id, first_name, last_name, birthday_date, city }).unwrap()),
        Err(get_user::Error::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(get_user::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(get_user::Error::Unknown) => HttpResponse::Conflict().finish(),
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use actix_web::{App, test, web, http::header::ContentType, Responder};
    use actix_web::body::to_bytes;
    use actix_web::dev::ServiceResponse;
    use actix_web::error::ErrorBadRequest;
    use actix_web::http::{header, StatusCode};
    use actix_web::web::Data;
    use chrono::NaiveDate;
    use random_string::generate;
    use uuid::Uuid;
    use crate::api::user::create_user::{Request, Response};
    use crate::api::user::get_user::{serve};
    use crate::domain::get_user;
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};


    #[actix_web::test]
    async fn test_get_user_route_ok() {
        let charset = "abcdefghijkl";
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let id = Uuid::new_v4().to_string();
        let db_user =  DbUser {
            id: id.clone(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let res_insert = repository.insert(db_user).await.unwrap();
        let repo = Data::new(repository);
        let mut app = test::init_service(
            App::new()
                .route("/{id}", web::get().to(serve))
            .app_data(repo)).await;
        let res = test::TestRequest::get().uri(&format!("/{}", id)).send_request(&app).await;
        assert!(res.status().is_success());
        let result:Response = test::read_body_json(res).await;
        assert_eq!(result.first_name, res_insert.first_name);
        assert_eq!(result.last_name, res_insert.last_name);
        assert_eq!(result.birthday_date,  res_insert.birthday_date);
        assert_eq!(result.city, res_insert.city);
    }
    #[actix_web::test]
    async fn test_get_user_route_fail_badurl() {
        let charset = "abcdefghijkl";
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let id = Uuid::new_v4().to_string();
        let db_user =  DbUser {
            id: id.clone(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let res_insert = repository.insert(db_user).await.unwrap();
        let repo = Data::new(repository);
        let mut app = test::init_service(
            App::new()
                .route("/{id}", web::get().to(serve))
                .app_data(repo)).await;
       let res= test::TestRequest::get().uri("/").send_request(&app).await;

        assert_eq!( res.status(),StatusCode::NOT_FOUND);
    }
}