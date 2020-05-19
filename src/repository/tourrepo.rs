use actix_web::web;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, error::Error, oid::ObjectId};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

use crate::model::tour::{Tour, InsertableTour};

pub fn all(
  connection: web::Data<Pool<MongodbConnectionManager>>
) -> Result<Vec<Tour>, Error> {
  
  let cursor = connection
    .get()
    .expect("can not get pool")
    .collection("tours")
    .find(None, None).unwrap();
    
  cursor
    .map(|result| match result {
      Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
        Ok(result_model) => Ok(result_model),
        Err(_) => Err(Error::DefaultError(String::from(""))),
      },
        Err(err) => Err(err),
      })
    .collect::<Result<Vec<Tour>, Error>>()
}

pub fn insert(
    tour: Tour,
    connection: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<mongodb::coll::results::InsertOneResult, mongodb::error::Error> {
  
  let insertable = InsertableTour::from_tour(tour.clone());
  println!("{}", insertable.name);
  match bson::to_bson(&insertable) {
      Ok(model_bson) => match model_bson {
          bson::Bson::Document(model_doc) => {
              connection
                  .get()
                  .expect("can not get pool")
                  .collection("tours")
                  .insert_one(model_doc, None)
              
          }
          _ => Err(Error::DefaultError(String::from(
              "Failed to create Document",
          ))),
      },
      Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
  }
}
  
pub fn get(
    product_id: ObjectId,
    connection: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Option<Tour>, Error> {
    //println!("{}", product_id);

    match connection
      .get()
      .expect("can not get pool")
      .collection("tours")
      .find_one(Some(doc! { "_id":  product_id}), None)
      {
        Ok(db_result) => match db_result {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result_model) => Ok(Some(result_model)),
                Err(_) => Err(Error::DefaultError(String::from(
                    "Failed to create reverse BSON",
                ))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
      }
}