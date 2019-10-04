use crate::State;
use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};

pub fn load(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
    let config = serde_qs::Config::new(5, false);
    let params = config.deserialize_str::<HashMap<String, String>>(req.query_string()).unwrap();

    let img = match state.load_image(&params["image_url"]) {
        Ok(res) => res,
        Err(err) => { println!("err: {}", err); panic!(); },
    };
    
    HttpResponse::Ok().content_type("image/jpeg")
        .body(img)
}

pub fn show(req: HttpRequest) -> HttpResponse {
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

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, dev::Service};
    use actix_web::{web, App};

    #[test]
    fn test_load() {
        let state = web::Data::new(State::new());
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/load/original", web::get().to(load)));
        let req = test::TestRequest::get().uri("/image/load/original?image_url=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_show() {
        let state = web::Data::new(State::new());
        let mut app = test::init_service(App::new().register_data(state.clone()).route("/image/show/original", web::get().to(show)));
        let req = test::TestRequest::get()
            .uri("/image/show/original?image_url[0]=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg&image_url[1]=https://sr.gallerix.ru/_EX/856702129/885028076.jpg").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }
}