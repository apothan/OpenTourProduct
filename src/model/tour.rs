use serde::{Serialize,Deserialize};
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tour {
  #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTour {
  pub name: String,
}

impl InsertableTour {
  pub fn from_tour(tour: Tour) -> InsertableTour {
      InsertableTour {
          name: tour.name,
      }
  }
}

