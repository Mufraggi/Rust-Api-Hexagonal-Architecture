use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use crate::domain::delete_user;
use crate::domain::delete_user::Request;
use crate::PostgresRepository;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Param {
    pub id: String,
}
#[tracing::instrument]
pub async fn serve(repo: Data<PostgresRepository>, path: web::Path<Param>) -> impl Responder {
    let req = Request { id: path.into_inner().id };
    match delete_user::execute(repo, req).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(delete_user::Error::BadRequest) => HttpResponse::BadRequest().finish(),
        Err(delete_user::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(delete_user::Error::Unknown) => HttpResponse::Conflict().finish(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use actix_web::http::StatusCode;
    use actix_web::web::Data;
    use chrono::NaiveDate;
    use random_string::generate;
    use uuid::Uuid;
    use crate::api::user::delete_user::serve;
    use crate::repository::user::{FetchAllError, FetchOneError};
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[actix_web::test]
    async fn test_delete_user_route_ok() {
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
                .app_data(repo.clone())).await;
        let res = test::TestRequest::get().uri(&format!("/{}", id)).send_request(&app).await;
        assert!(res.status().is_success());
        let res = repo.get(id.clone()).await;
        assert_eq!(res.err().unwrap(), FetchOneError::NotFound)
    }
    #[actix_web::test]
    async fn test_delete_user_route_not_found() {
        let id = Uuid::new_v4().to_string();
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let mut app = test::init_service(
            App::new()
                .route("/{id}", web::get().to(serve))
                .app_data(repo.clone())).await;
        let res = test::TestRequest::get().uri(&format!("/{}", id)).send_request(&app).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND)
    }
}