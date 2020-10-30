#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod models {
    pub mod drink;
    pub mod drink_drank;
}

pub mod schema;
pub mod drinks_repository;
pub mod db;
