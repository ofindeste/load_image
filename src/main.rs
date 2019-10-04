use std::io;

use actix_web::{web, App, HttpServer};

mod routes;
mod state;
mod utils;

use state::State;

fn main() -> io::Result<()> {
    let state = web::Data::new(State::new());

    HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/image/original").route(web::get().to(routes::image::original::load)))
            .service(web::resource("/image/preview").route(web::get().to(routes::image::preview::load)))
            .service(web::resource("/image/original/show").route(web::get().to(routes::image::original::show)))
            .service(web::resource("/image/preview/show").route(web::get().to(routes::image::preview::show)))
    })
    .bind("127.0.0.1:8090")?
    .run()
}
