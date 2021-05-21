#[cfg(test)]
mod tests {
    use crate::lib::init_app_data;
    use crate::routes::read;
    use actix_web::{test, App};
    use std::time::Duration;

    #[actix_rt::test]
    async fn test_unknown_key() {
        let db = init_app_data(Duration::from_secs(5));
        let mut app = test::init_service(App::new().app_data(db).service(read)).await;
        let req = test::TestRequest::get().uri("/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
    }
    // @TODO more tests
}
