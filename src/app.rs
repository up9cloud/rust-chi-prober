use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};

use ::info::Info;

#[derive(Serialize, Deserialize)]
struct ResponseWithData<T> {
    code: u16,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct ResponseWithErr {
    code: u16
}

fn version () -> HttpResponse {
    HttpResponse::Ok().json(ResponseWithData{
        code: 200,
        data: ::VERSION
    })
}
fn ping (_info: web::Data<Mutex<Info>>) -> HttpResponse {
    let info = _info.lock().unwrap();
    HttpResponse::Ok().json(ResponseWithData{
        code: 200,
        data: info.get_partial()
    })
}
fn server_info(_info: web::Data<Mutex<Info>>) -> HttpResponse {
    let mut info = _info.lock().unwrap();
    info.refresh();
    HttpResponse::Ok().json(ResponseWithData{
        code: 200,
        data: info.get_all()
    })
}
fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ResponseWithErr{
        code: 404
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .data(web::JsonConfig::default().limit(1024))
        .service(
            web::scope("/fn")
                .route("/version", web::get().to(version))
                .route("/ping", web::get().to(ping))
                .route("/info", web::get().to(server_info))
                .default_service(web::to(not_found))
        )
        .service(
            actix_files::Files::new("/", "./static/").index_file("index.html")
        );
}
