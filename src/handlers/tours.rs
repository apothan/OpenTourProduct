use crate::model::tour::TourForm;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
//use std::io::Error;

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::tour::Tours;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// NOTE: test API without db conneciton;
// fn tours_mock() -> Result<Tour, Error> {
//     let _tours_json = r#"{"id":"89251ab3-1cdc-4629-9086-ce022cf6669e","name":"Rome Tour"}"#;

//     let tour: Tour = serde_json::from_str(_tours_json)?;

//     Ok(user)
// }

pub async fn get(
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(Tours::list(&mysql_pool)))
}

pub async fn insert(
    tour: web::Json<TourForm>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {

    //println!("model: {:?}", &item);
    //HttpResponse::Ok().json(item.0)

    //let serialized = serde_json::to_string(&tour_form).unwrap();

        // Prints serialized = {"x":1,"y":2}
    //println!("serialized = {:#?}", tour_form);
    
    let mysql_pool = mysql_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(Tours::insert(tour.0, &mysql_pool)))
    //Ok(())
    //Ok(HttpResponse::Ok().json(&item.0))
     // <- send response
}

// NOTE: testing connection without DB
// pub async fn get(
//     _req: HttpRequest,
// ) -> Result<HttpResponse, HttpResponse> {
//     let mysql_pool = mysql_pool_handler(pool)?;
//     Ok(HttpResponse::Ok().json(Tours::list(&mysql_pool)))
// }