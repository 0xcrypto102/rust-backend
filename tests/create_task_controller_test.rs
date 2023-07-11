#[cfg(test)]
mod tests {
    use backend::bounded_context::infrastructure::http::{configure_routes, create_task_controller::CreateTaskResponse};
    use actix_web::{test, App};
    use actix_web::http::{header::ContentType, StatusCode};
    use serde_json::{json, from_str};

    const DEF_TITLE: &str = "Test Task";
    const DEF_DESCRIPTION: &str = "Test description";

    #[actix_web::test]
    async fn test_create_task_controller() {
        let app_entry = App::new().configure(configure_routes::configure_routes);
        let app = test::init_service(app_entry).await;

        let payload = json!({
            "title": DEF_TITLE,
            "description": DEF_DESCRIPTION
        });

        let req = test::TestRequest::post()
            .uri("/api/tasks")
            .append_header(ContentType::json())
            .set_payload(payload.to_string())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body();

        let bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_str = std::str::from_utf8(&bytes).unwrap();

        let response: CreateTaskResponse = from_str(body_str).unwrap();

        assert_eq!(response.title, DEF_TITLE);
        assert_eq!(response.description, DEF_DESCRIPTION);
        assert_eq!(response.status, "Fake status");
        assert!(!response.id.is_empty());
    }    
}
