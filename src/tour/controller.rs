 
use actix_web::{web, HttpResponse, http, Error};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

use super::repository;
use super::Tour;

pub async fn index(
    user: web::Query<Tour>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error>{
    let res = web::block(move || repository::index(&tour.name, pool))
    .await
    .map(|_result| HttpResponse::Ok().body("Success"))
    .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || repository::get(pool))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}