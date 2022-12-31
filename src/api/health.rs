use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, PartialEq, Debug, Deserialize)]
struct Response {
    status: String,
}


//#[tracing::instrument]
pub async fn health() -> impl Responder {
    tracing::info!("just before the ok");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Response {
        status: "OK".parse().unwrap()
    }).unwrap())
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web, http::header::ContentType};
    use actix_web::body::to_bytes;
    use actix_web::http::{header, StatusCode};
    use crate::api::health::{health, Response};

    #[actix_web::test]
    async fn test_health_route_ok() {
        let mut app  =
            test::init_service(App::new().route("/", web::get().to(health))).await;
        let res = test::TestRequest::get()
            .uri("/")
            .send_request(&mut app).await;
        assert!(res.status().is_success());
        let result:Response = test::read_body_json(res).await;
        assert_eq!(result,(Response{ status: "OK".parse().unwrap()}));
    }
}