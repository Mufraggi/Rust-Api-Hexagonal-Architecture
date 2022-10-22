use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use chrono::NaiveDate;
use crate::domain::update_user;
use crate::domain::update_user::{Error, Params};
use crate::PostgresRepository;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}
#[tracing::instrument]
pub async fn serve(repo: Data<PostgresRepository>, path: web::Path<Param>, req: web::Json<Request>) -> impl Responder {
    let param = Param { id: path.into_inner().id };
    let req = update_user::Request {
        first_name: req.0.first_name,
        last_name: req.0.last_name,
        birthday_date: req.0.birthday_date,
        city: req.0.city,
    };
    match update_user::execute(repo,  Params{id:param.id}, req).await {
        Ok(update_user::Response {
               id,
               first_name,
               last_name,
               birthday_date,
               city
           }) => HttpResponse::Created()
            .content_type("application/json")
            .body(serde_json::to_string(&Response { id, first_name, last_name, birthday_date, city }).unwrap()),
        Err(update_user::Error::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(update_user::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(update_user::Error::Unknown) => HttpResponse::Conflict().finish(),
        _ => HttpResponse::BadRequest().finish()
    }
}

impl Request {
    pub fn good() -> Self {
        Self {
            first_name: "hugo".to_string(),
            last_name: "muf".to_string(),
            birthday_date: "1994-10-03".to_string(),
            city: "nice".to_string(),
        }
    }

    pub fn bad() -> Self {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            birthday_date: "1994-10-03".to_string(),
            city: "nice".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, HttpResponse, test, web};
    use actix_web::http::StatusCode;
    use actix_web::web::Data;
    use chrono::NaiveDate;
    use random_string::generate;
    use uuid::Uuid;
    use crate::api::user::update_user::{Request, Response, serve};
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[actix_web::test]
    async fn test_get_user_route_ok() {
        let charset = "abcdefghijkl";
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = Data::new(PostgresRepository::new_pool(url).await.unwrap());
        let id = Uuid::new_v4().to_string();
        let db_user = DbUser {
            id: id.clone(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };

        let res_insert = repository.insert(db_user).await.unwrap();
        let mut app = test::init_service(
            App::new()
                .route("/{id}", web::put().to(serve))
                .app_data(repository)).await;
        let res = test::TestRequest::put()
            .uri(&format!("/{}", id))
            .set_json(Request::good())
            .send_request(&app).await;
        assert!(res.status().is_success());
        let result: Response = test::read_body_json(res).await;
        assert_eq!(result.first_name, Request::good().first_name);
        assert_eq!(result.last_name, Request::good().last_name);
        assert_eq!(result.birthday_date, NaiveDate::parse_from_str(Request::good().birthday_date.as_str(), "%Y-%m-%d").unwrap());
        assert_eq!(result.city,Request::good().city);
    }
}