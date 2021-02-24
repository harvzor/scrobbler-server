#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod models {
    pub mod trackable;
    pub mod scrobble;
}

pub mod schema;
pub mod trackables_repository;
pub mod scrobbles_repository;
pub mod db;
