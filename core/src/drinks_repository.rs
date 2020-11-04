use diesel::prelude::*;

use crate::models::drink::Drink;
use crate::models::drink::DrinkWithCount;
use crate::db::Db;

pub struct DrinksRepository {
    pub db: Db,
}

impl DrinksRepository {
    pub fn new() -> DrinksRepository {
        DrinksRepository {
            db: Db::new(),
        }
    }
    pub fn get_drinks(&self, drink_deleted: bool) -> Vec<DrinkWithCount> {
        // Not particularly efficient.
        // Would be better to get the drinks with count in a single call but I can't get that to work.
        // Seems Diesel doesn't really support joins too well.
        // And I can't seem to get raw SQL really working either: https://github.com/Harvzor/drinks-drunk/commit/1081cec5a37cc84db242eaf669aea18a4a1b05b1

        use crate::schema::drinks::dsl::*;
        use crate::drink_dranks_repository::DrinkDranksRepository;

        let drinks_result = drinks
            .filter(deleted.eq(drink_deleted))
            .load::<Drink>(&self.db.connection)
            .expect("Error loading drinks");

        let drink_ids = drinks_result
            .iter()
            .map(|x| x.id)
            .collect();

        let drink_drank_result = DrinkDranksRepository::new().get_drink_dranks_for_drinks(drink_ids);

        drinks_result
            .iter()
            .map(|x| {
                let count = drink_drank_result
                    .iter()
                    .filter(|y| y.drink_id == x.id)
                    .count();

                DrinkWithCount::from_drink_with_count(x, count as i32)
            })
            .collect()
    }
    pub fn create_drink<'a>(&self, name: &'a str, colour: &'a str) -> Drink {
        use crate::schema::drinks;
        use crate::models::drink::NewDrink;

        let new_drink = NewDrink {
            name: name,
            colour: colour,
        };

        diesel::insert_into(drinks::table)
            .values(&new_drink)
            .get_result(&self.db.connection)
            .expect("Error saving new drink")
    }
    pub fn delete_drink(&self, drink_id: i32, hard_delete: bool) -> Option<Drink> {
        use crate::schema::drinks::dsl::*;

        match hard_delete {
            true => {
                diesel::delete(drinks.find(id))
                    .execute(&self.db.connection)
                    .expect("Error deleted posts");

                return None;
            },
            false => {
                let drink = diesel::update(drinks.find(id))
                    .set(deleted.eq(true))
                    .get_result::<Drink>(&self.db.connection)
                    .expect(&format!("Unable to find drink {}", drink_id));

                return Some(drink);
            },
        }
    }
    pub fn find_by_id(&self, drink_id: i32) -> Option<Drink> {
        use crate::schema::drinks::dsl::*;

        let drink = drinks
            .find(drink_id)
            .first::<Drink>(&self.db.connection);

        match drink {
            Ok(d) => {
                return Some(d);
            }
            Err(_) => return None,
        }
    }
    pub fn find_by_name(&self, drink_name: &String) -> Option<Drink> {
        use crate::schema::drinks::dsl::*;

        let drink = drinks
            .filter(name.eq(drink_name))
            .first::<Drink>(&self.db.connection);

        match drink {
            Ok(d) => {
                return Some(d);
            }
            Err(_) => return None,
        }
    }
}
