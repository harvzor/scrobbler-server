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
        // use crate::schema::*;

        // use crate::schema::drinks::dsl::*;
        // use crate::schema::drink_dranks::dsl::*;

        // Guess I'll Diesel 2.0 for this.
        // use diesel::dsl::count;
        // use diesel::dsl::sql;
        // use diesel::sql_types::BigInt;
        // drinks::table
        //     .left_join(drink_dranks::table.on(drinks::id.eq(drink_dranks::drink_id)))
        //     .group_by(drink_dranks::drink_id)
        //     .filter(drinks::deleted.eq(drink_deleted))
        //     .select((drinks::id, drinks::name, count(drink_dranks::drink_id), drinks::colour, drinks::deleted))
        //     // .select((drinks::id, drinks::name, sql::<BigInt>("COUNT(drink_dranks.drink_id) AS count"), drinks::colour, drinks::deleted))
        //     .load::<DrinkWithCount>(&self.db.connection)
        //     .expect("Error");

        // use diesel::sql_types::Integer;
        // use diesel::sql_types::BigInt;
        // use diesel::sql_types::Text;
        // use diesel::sql_types::Bool;

        diesel::sql_query(include_str!("drinks_with_count.sql"))
            // .bind::<Integer, _>(id)
            // .bind::<Text, _>(name)
            // .bind::<BigInt, _>("count")
            // .bind::<Text , _>(colour)
            // .bind::<Bool, _>(deleted)
            .load::<DrinkWithCount>(&self.db.connection)
            .expect("Error")
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
