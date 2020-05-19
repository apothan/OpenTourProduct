use actix_web::{web, App, HttpServer};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};


mod controller;
mod model;
mod repository;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            .with_db("products")
            .build(),
    );
    println!("Database connected!");
    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(controller::tourcontroller::index))
            .route("/insert", web::post().to(controller::tourcontroller::insert))
            .route("/get/{product_id}", web::get().to(controller::tourcontroller::get))
            .route("/delete/{product_id}", web::get().to(controller::tourcontroller::delete))
    })
    .bind("127.0.0.1:8010")?
    .run()
    .await
}