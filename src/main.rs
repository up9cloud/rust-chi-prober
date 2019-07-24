#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate actix_rt;
extern crate actix_files;
extern crate actix_web;
extern crate sysinfo;

mod app;
mod info;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::sync::Mutex;
use actix_web::{
  App,
  web,
  middleware::Logger
};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", format!("actix_web=info,{}=info", NAME));
    env_logger::init();
    let ip_address = "127.0.0.1:8088";

    let sv = actix_rt::System::new(NAME);

    let app = move || {
      debug!("Constructing the App");

      let info = info::Info::new();

      // let error_handlers = ErrorHandlers::new()
      //     .handler(
      //         http::StatusCode::INTERNAL_SERVER_ERROR,
      //         api::internal_server_error,
      //     )
      //     .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
      //     .handler(http::StatusCode::NOT_FOUND, api::not_found);

      App::new()
          .wrap(Logger::default())
          .register_data(web::Data::new(Mutex::new(info)))
          .configure(::app::config)
          // .wrap(error_handlers)
    };

    debug!("Starting server");
    actix_web::HttpServer::new(app)
        .bind(ip_address)?
        .start();

    info!("Running server on {}", ip_address);

    sv.run()
}
