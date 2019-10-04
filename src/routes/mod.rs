pub mod image;

use actix_web::{HttpRequest, HttpResponse, http::StatusCode};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/index.html"))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, http};

    #[test]
    fn test_index() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
            .to_http_request();

        let resp = test::block_on(index(req)).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}