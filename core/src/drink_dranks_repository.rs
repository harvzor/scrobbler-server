use diesel::prelude::*;
use diesel::dsl::*;

use crate::models::drink_drank::DrinkDrank;
use crate::db::Db;

pub struct DrinkDranksRepository {
    pub db: Db,
}

impl DrinkDranksRepository {
    pub fn new() -> DrinkDranksRepository {
        DrinkDranksRepository {
            db: Db::new(),
        }
    }
    pub fn get_drink_dranks(&self) -> Vec<DrinkDrank> {
        use crate::schema::drink_dranks::dsl::*;

        let results = drink_dranks
            .load::<DrinkDrank>(&self.db.connection)
            .expect("Error loading drink dranks");

        results
    }
    pub fn create_drink_drank(&self, drink_id: i32) -> DrinkDrank {
        use crate::schema::drink_dranks;
        use crate::models::drink_drank::NewDrinkDrank;

        let new_drink_drank = NewDrinkDrank {
            drink_id: drink_id,
            drank_timestamp: chrono::Utc::now().naive_local(),
        };

        diesel::insert_into(drink_dranks::table)
            .values(&new_drink_drank)
            .get_result(&self.db.connection)
            .expect("Error saving new drink drank")
    }
    pub fn get_drink_dranks_for_drink(&self, drink_drank_drink_id: i32) -> Vec<DrinkDrank> {
        use crate::schema::drink_dranks::dsl::*;

        drink_dranks
            .filter(drink_id.eq(drink_drank_drink_id))
            .load::<DrinkDrank>(&self.db.connection)
            .expect("Error loading drink dranks")
    }
    pub fn get_drink_dranks_for_drinks(&self, drink_drank_drink_ids: Vec<i32>) -> Vec<DrinkDrank> {
        use crate::schema::drink_dranks::dsl::*;

        drink_dranks
            .filter(drink_id.eq(any(drink_drank_drink_ids)))
            .load::<DrinkDrank>(&self.db.connection)
            .expect("Error loading drink dranks")
    }
}
