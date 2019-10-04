use crate::State;
use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use crate::utils::ResizeImage;

pub fn load(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, String>>(req.query_string()).unwrap();

    let image = match state.load_image(&params["image_url"]) {
        Ok(res) => res,
        Err(err) => { println!("err: {}", err); panic!(); }
    };
    let image = image::load_from_memory_with_format(&image, image::ImageFormat::JPEG).unwrap();
    let image = image.resize_to_exact(100, 100);

    let mut buf: Vec<u8> = Vec::new();

    image.write_to(&mut buf, image::ImageOutputFormat::JPEG(127)).unwrap();

    HttpResponse::Ok().content_type("image/jpeg")
        .body(buf)
}


pub fn show(req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, Vec<String>>>(req.query_string()).unwrap();

    let mut s = String::new();

    for url in &params["image_url"] {
        let mut params = HashMap::new();
        params.insert("image_url", url);
        s += &format!("<img style=\"margin: 5px;\" src=/image/load/preview?{}>", serde_qs::to_string(&params).unwrap())
    }

    HttpResponse::Ok().content_type("text/html")
        .body(&s)
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, dev::Service};
    use std::{io::Write};
    use actix_web::{web, App};

    use image::{GenericImageView};

    #[test]
    fn test_show() {
        let state = web::Data::new(State::new());
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/show/preview", web::get().to(show)));
        let req = test::TestRequest::get()
            .uri("/image/show/preview?image_url[0]=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg&image_url[1]=https://sr.gallerix.ru/_EX/856702129/885028076.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_load() {
        let state = web::Data::new(State::new());

        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/load/preview", web::get().to(load)));
        let req = test::TestRequest::get().uri("/image/load/preview?image_url=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg").to_request();
        let mut resp = test::block_on(app.call(req)).unwrap();

        let v: Vec<u8> = match resp.take_body().as_ref().unwrap() {
            actix_web::body::Body::Bytes(bytes) => { bytes.to_vec() },
            _ => { Vec::new() }
        };

        let res_image = image::load_from_memory_with_format(&v, image::ImageFormat::JPEG).unwrap();

        let mut preview = std::fs::File::create("test_assets/preview.jpg").unwrap();
        preview.write(&v).unwrap();

        let expected = std::fs::File::open("test_assets/preview_expected.jpg").unwrap();

        let result_metadata = preview.metadata().unwrap();
        let expected_metadata = expected.metadata().unwrap();

        assert!(resp.status().is_success());
        assert_eq!(result_metadata.len(), expected_metadata.len());
        assert_eq!(res_image.dimensions(), (100, 100));
    }
}