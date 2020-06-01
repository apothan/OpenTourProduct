#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate dotenv_codegen;

pub mod db;
pub mod handlers;
pub mod model;
pub mod schema;

//use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use db::connect;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // let sys = actix::System::new("user_sync");
    env_logger::init();

    // load ssl keys
    /*
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    */

    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(connect())
            .service(web::resource("/").route(web::get().to(handlers::tours::get)))
            .service(web::resource("/insert").route(web::post().to(handlers::tours::insert)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .await
    /*
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
    */
}