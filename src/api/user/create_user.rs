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


#[derive(Debug, Deserialize,Serialize)]
pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize )]
pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}
#[tracing::instrument]
pub async fn serve(repo: Data< PostgresRepository>, req: web::Json<Request>) -> impl Responder {
    let req = create_user::Request {
        first_name: req.0.first_name,
        last_name: req.0.last_name,
        birthday_date: req.0.birthday_date,
        city: req.0.city,
    };
    tracing::info!("create user");
    tracing::span!(tracing::Level::INFO, "juste before the db call");

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

#[cfg(test)]
impl Request {
    pub fn good() -> Self {
        Self{
            first_name: "hugo".to_string(),
            last_name: "muf".to_string(),
            birthday_date: "1994-10-03".to_string(),
            city: "nice".to_string()
        }
    }

    pub fn bad() -> Self {
        Self{
            first_name: "".to_string(),
            last_name: "".to_string(),
            birthday_date: "1994-10-03".to_string(),
            city: "nice".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web, http::header::ContentType};
    use actix_web::body::to_bytes;
    use actix_web::error::ErrorBadRequest;
    use actix_web::http::{header, StatusCode};
    use actix_web::web::Data;
    use chrono::NaiveDate;
    use crate::api::user::create_user::{Request, Response, serve};
    use crate::PostgresRepository;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive( Deserialize, Serialize)]
    pub struct Fail {
        pub id: String,
    }


    #[actix_web::test]
    async fn test_create_user_route_ok() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let mut app  =
            test::init_service(App::new()
                .route("/", web::post().to(serve))
                .app_data(repo)).await;
        let res = test::TestRequest::post()
            .uri("/").set_json(Request::good())
            .send_request(&mut app).await;
        assert!(res.status().is_success());
        let result:Response = test::read_body_json(res).await;
        let excepted = Request::good();

        assert_eq!(result.first_name, excepted.first_name);
        assert_eq!(result.last_name, excepted.last_name);
        assert_eq!(result.birthday_date,  NaiveDate::parse_from_str(&excepted.birthday_date, "%Y-%m-%d").unwrap());
        assert_eq!(result.city, excepted.city);
    }

    #[actix_web::test]
    async fn test_create_user_route_fail_bad_request() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let mut app  =
            test::init_service(App::new()
                .route("/", web::post().to(serve))
                .app_data(repo)).await;
        let res = test::TestRequest::post()
            .uri("/").set_json(Request::bad())
            .send_request(&mut app).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_user_route_vwith_bad_req_fail_bad_request() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let mut app  =
            test::init_service(App::new()
                .route("/", web::post().to(serve))
                .app_data(repo)).await;
        let res = test::TestRequest::post()
            .uri("/").set_json(Fail{id: "aaa".parse().unwrap() })
            .send_request(&mut app).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}