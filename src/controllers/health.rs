use actix_web::{get, HttpResponse, Responder};

use crate::models::health_response::HealthResponse;

#[get("/health")]
async fn health() -> impl Responder {
    let response = HealthResponse {
        message: "Everything is OK!".into(),
        status: 200,
    };

    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use crate::models::health_response::HealthResponse;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_response() {
        let app = test::init_service(App::new().service(health)).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp: HealthResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.message, "Everything is OK!");
        assert_eq!(resp.status, 200)
    }
}
