use actix_web::{web, HttpResponse, http, Error};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

use crate::repository::tourrepo;
use crate::model::Tour;

pub async fn index(
    tour: web::Query<Tour>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error>{
    let res = web::block(move || tourrepo::index(&tour.name, pool))
    .await
    .map(|_result| HttpResponse::Ok().body("Success"))
    .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || tourrepo::get(pool))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}