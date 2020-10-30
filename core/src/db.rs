use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::drink::Drink;
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

        results
    }
    pub fn create_drink<'a>(&self, new_name: &'a str, new_colour: &'a str) -> Drink {
        use crate::schema::drinks;
        use crate::models::drink::NewDrink;

        let new_drink = NewDrink {
            name: new_name,
            colour: new_colour,
        };

        diesel::insert_into(drinks::table)
            .values(&new_drink)
            .get_result(&self.connection)
            .expect("Error saving new drink")
    }
    pub fn delete_drink(&self, drink_id: i32) -> Drink {
        diesel::update(drinks.find(drink_id))
            .set(deleted.eq(true))
            .get_result::<Drink>(&self.connection)
            .expect(&format!("Unable to find drink {}", drink_id))
    }
}
