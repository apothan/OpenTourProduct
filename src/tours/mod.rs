#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tour {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTour {
    pub name: Option<String>,
    pub category: Option<String>,
}

impl InsertableTour {
    fn from_tour(tours: Tour) -> InsertableTour {
        InsertableTour {
            name: tours.name,
            category: tours.category,
        }
    }
}