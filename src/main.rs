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
