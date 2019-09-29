use std::io;
use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, http::StatusCode};
use image::{GenericImage, GenericImageView, DynamicImage};

#[derive(Debug)]
pub struct State {
    client: reqwest::Client,
}

impl State {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn load_image(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut res = self.client.get(url).send()?;

        if res.status().is_success() {
            let mut s = Vec::<u8>::new();
            res.copy_to(&mut s)?;

            Ok(s)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,
                         "Could not get resource successfully"))
            )
        }
    }
}

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

fn load_original_image(state: web::Data<Mutex<State>>, req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, String>>(req.query_string()).unwrap();

    let img = match state.lock().unwrap().load_image(&params["image_url"]) {
        Ok(res) => res,
        Err(err) => { println!("err: {}", err); panic!(); },
    };
    
    HttpResponse::Ok().content_type("image/jpeg")
        .body(img)
}

fn show_original_image(req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, Vec<String>>>(req.query_string()).unwrap();

    let mut s = String::new();

    for url in &params["image_url"] {
        let mut params = HashMap::new();
        params.insert("image_url", url);
        s += &format!("<img style=\"margin: 5px;\" src=/image/load/original?{}>", serde_qs::to_string(&params).unwrap())
    }

    HttpResponse::Ok().content_type("text/html")
        .body(&s)
}

fn load_preview_image(state: web::Data<Mutex<State>>, req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, String>>(req.query_string()).unwrap();

    let img = match state.lock().unwrap().load_image(&params["image_url"]) {
        Ok(res) => res,
         Err(err) => { println!("err: {}", err); panic!(); }
    };

    let img = image::load_from_memory_with_format(&img, image::ImageFormat::JPEG).unwrap();
    let resized = img.resize(100, 100, image::FilterType::Gaussian);

    let mut background = DynamicImage::new_rgba8(100, 100);
    let (width, height) = resized.dimensions();
    if width < 100 {
        let margin = (100 - width) / 2;
        background.copy_from(&resized, margin, 0);
    }
    if height < 100 {
        let margin = (100 - height) / 2;
        background.copy_from(&resized, 0, margin);
    }

    let mut buf: Vec<u8> = Vec::new();

    background.write_to(&mut buf, image::ImageOutputFormat::JPEG(127)).unwrap();

    HttpResponse::Ok().content_type("image/jpeg")
        .body(buf)
}

fn show_preview_image(req: HttpRequest) -> HttpResponse {
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

fn main() -> io::Result<()> {
    let state = web::Data::new(Mutex::new(State::new()));

    HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/image/load/original").route(web::get().to(load_original_image)))
            .service(web::resource("/image/load/preview").route(web::get().to(load_preview_image)))
            .service(web::resource("/image/show/original").route(web::get().to(show_original_image)))
            .service(web::resource("/image/show/preview").route(web::get().to(show_preview_image)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, http, dev::Service};
    use std::{fs::File, io::Write};

    #[test]
    fn check_state_load_image() {
        let state = State::new();

        let res = state.load_image("https://sr.gallerix.ru/_UNK/1018810316/3526.jpg").unwrap();
        let mut file = File::create("test_assets/res.jpg").unwrap();
        file.write(&res).unwrap();
        let expected_image = std::fs::File::open("test_assets/img.jpg").unwrap();

        let result_metadata = file.metadata().unwrap();
        let expected_metadata = expected_image.metadata().unwrap();

        assert_eq!(result_metadata.len(), expected_metadata.len());
    }

    #[test]
    fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
            .to_http_request();

        let resp = test::block_on(index(req)).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_load_original_image() {
        let state = web::Data::new(Mutex::new(State::new()));
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/load/original", web::get().to(load_original_image)));
        let req = test::TestRequest::get().uri("/image/load/original?image_url=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_show_original_image() {
        let state = web::Data::new(Mutex::new(State::new()));
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/show/original", web::get().to(show_original_image)));
        let req = test::TestRequest::get()
            .uri("/image/show/original?image_url[0]=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg&image_url[1]=https://sr.gallerix.ru/_EX/856702129/885028076.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_show_preview_image() {
        let state = web::Data::new(Mutex::new(State::new()));
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/show/preview", web::get().to(show_preview_image)));
        let req = test::TestRequest::get()
            .uri("/image/show/preview?image_url[0]=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg&image_url[1]=https://sr.gallerix.ru/_EX/856702129/885028076.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_load_preview_image() {
        let state = web::Data::new(Mutex::new(State::new()));

        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/load/preview", web::get().to(load_preview_image)));
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
