use serde::Deserialize;

pub mod controller;
mod repository;

#[derive(Deserialize)]
pub struct Tour {
  name: String,
}
