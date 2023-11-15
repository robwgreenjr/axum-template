#[cfg(test)]
mod users {
    use axum_test::TestServer;
    use http::StatusCode;

    use crate::app;

    #[tokio::test]
    async fn when_calling_users_endpoint_should_return_200() {
        let app = app("postgres://root:root@127.0.0.3:5432/main".to_string())
            .await
            .into_make_service();
        let server = TestServer::new(app).unwrap();
        let response = server
            .get("/users")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn when_calling_user_endpoint_should_return_200() {
        let app = app("postgres://root:root@127.0.0.3:5432/main".to_string())
            .await
            .into_make_service();
        let server = TestServer::new(app).unwrap();
        let response = server
            .get("/user")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
