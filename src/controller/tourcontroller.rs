use actix_web::{web, HttpResponse, http, Error};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use serde::Deserialize;
//use bson::oid::ObjectId;
use mongodb::oid::ObjectId;

#[derive(Deserialize)]
pub struct Info {
    product_id: String,
}

use crate::repository::tourrepo;
use crate::model::tour::Tour;

pub async fn index(
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error>{
    
    let res = web::block(move || tourrepo::all(pool))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn insert(
    tour: web::Json<Tour>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error>{
    let res = web::block(move || tourrepo::insert(tour.into_inner(), pool))
        .await
        .map(|_result| HttpResponse::Ok().body("Success"))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    info: web::Path<Info>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

    let product_id = ObjectId::with_string(&info.product_id).unwrap();
    let res = web::block(move || tourrepo::get(product_id, pool))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn delete(
    info: web::Path<Info>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

    let product_id = ObjectId::with_string(&info.product_id).unwrap();
    let res = web::block(move || tourrepo::delete(product_id, pool))
        .await
        .map(|_result| HttpResponse::Ok().body("Success"))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}