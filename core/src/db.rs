use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// For this to really be performant, it may be necessary to explicitly make use of connection pools?
// More documentation on how to use this with Rocket here: https://rocket.rs/v0.4/guide/state/#databases

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Db {
    pub connection: PgConnection,
}

impl Db {
    pub fn new() -> Db {
        Db {
            connection: establish_connection(),
        }
    }
}
