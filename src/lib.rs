#[macro_use]
extern crate diesel;
extern crate dotenv;
//extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod routes;
pub mod controllers;
pub mod models;
pub mod schema;
pub mod utils;