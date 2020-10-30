use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::drink::Drink;
use crate::schema::drinks::dsl::*;

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
    pub fn get_drinks(&self) -> Vec<Drink> {
        let results = drinks
            .filter(deleted.eq(false))
            .load::<Drink>(&self.connection)
            .expect("Error loading drinks");

        return results;
    }
}
