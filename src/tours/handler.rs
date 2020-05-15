use crate::tours;
use crate::mongo_connection::Conn;
use tours::Tour;
use mongodb::{ doc, error::Error, oid::ObjectId};
use rocket_contrib::json::Json;
use rocket::{http::Status};

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Tour>>, Status> {
    match tours::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<Tour>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match tours::repository::get(res, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        }
        Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
    }
}

#[post("/", format = "application/json", data = "<tours>")]
pub fn post(tours: Json<Tour>, connection: Conn) -> Result<Json<ObjectId>, Status> {
    match tours::repository::insert(tours.into_inner(), &connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[put("/<id>", format = "application/json", data = "<tours>")]
pub fn put(id: String, tours: Json<Tour>, connection: Conn) -> Result<Json<Tour>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match tours::repository::update(res, tours.into_inner(), &connection) {
            Ok(res) => Ok(Json(res)),
            Err(err) => Err(error_status(err)),
        }
        Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
    }
}

#[delete("/<id>")]
pub fn delete(id: String, connection: Conn) -> Result<Json<String>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match tours::repository::delete(res, &connection) {
            Ok(_) => Ok(Json(id)),
            Err(err) => Err(error_status(err)),
        }
        Err(_) => Err(error_status(Error::DefaultError(String::from("Couldn't parse ObjectId"))))
    }
}

#[delete("/")]
pub fn delete_all(connection: Conn) -> Result<Json<bool>, Status> {
    match tours::repository::delete_all(&connection) {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(error_status(err)),
    }
}