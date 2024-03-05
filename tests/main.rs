use std::collections::HashSet;

use ip_blocklist::app_state::AppState;
use ip_blocklist::methods::get;

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use actix_web::{test, web, App};

    fn setup_ips() -> HashSet::<String> {
        let mut hs = HashSet::<String>::new();
        hs.insert("144.172.73.16".to_string());
        hs.insert("89.234.157.254".to_string());
        hs.insert("95.214.24.192".to_string());
        return hs;
    }

    #[actix_web::test]
    async fn test_successful_match() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { ips: setup_ips() }))
                .service(web::scope("/ips").service(get))
        ).await;

        let req = test::TestRequest::get().uri("/ips/144.172.73.16").to_request();
        let resp = test::call_and_read_body(&app, req).await;
        assert_eq!(resp, Bytes::from_static(b"true"));

        let req2 = test::TestRequest::get().uri("/ips/95.214.24.192").to_request();
        let resp2 = test::call_and_read_body(&app, req2).await;
        assert_eq!(resp2, Bytes::from_static(b"true"));
    }

    #[actix_web::test]
    async fn test_unsuccessful_match() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { ips: setup_ips() }))
                .service(web::scope("/ips").service(get)),
        )
        .await;

        let req = test::TestRequest::get().uri("/ips/144.172.73.15").to_request();
        let resp = test::call_and_read_body(&app, req).await;
        assert_eq!(resp, Bytes::from_static(b"false"));

        let req2 = test::TestRequest::get().uri("/ips/95.214.24.193").to_request();
        let resp2 = test::call_and_read_body(&app, req2).await;
        assert_eq!(resp2, Bytes::from_static(b"false"));
    }

    #[actix_web::test]
    async fn test_invalid_ip() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { ips: setup_ips() }))
                .service(web::scope("/ips").service(get)),
        )
        .await;

        let req = test::TestRequest::get().uri("/ips/144.172.73.15123").to_request();
        let resp = test::call_and_read_body(&app, req).await;
        assert_eq!(resp, Bytes::from_static(b"Not a IPv4"));

        let req2 = test::TestRequest::get().uri("/ips/gonzalo").to_request();
        let resp2 = test::call_and_read_body(&app, req2).await;
        assert_eq!(resp2, Bytes::from_static(b"Not a IPv4"));
    }
}