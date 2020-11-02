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
        use crate::schema::*;
        // use crate::schema::drinks::dsl::*;
        // use crate::schema::drink_dranks::dsl::*;
        
        // I want something like:
        // SELECT d.id, d.name, dd.count, d.colour, d.deleted
        // FROM public.drinks d
        // LEFT JOIN
        // (
        //     SELECT dd.drink_id, COUNT(*) as count
        //     FROM public.drink_dranks dd
        //     GROUP BY dd.drink_id
        // ) dd ON d.id = dd.drink_id;

        let data = drinks::table.inner_join(drink_dranks::table)
            .filter(drinks::deleted.eq(drink_deleted))
            .select((drinks::id, drinks::name, drink_dranks::id, drinks::colour, drinks::deleted))
            .load::<DrinkWithCount>(&self.db.connection)
            .expect("Error");

        // let results = drinks
        //     .filter(deleted.eq(drink_deleted))
        //     .load::<Drink>(&self.db.connection)
        //     .expect("Error loading drinks");

        data
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
