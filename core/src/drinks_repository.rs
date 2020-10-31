use diesel::prelude::*;

use crate::models::drink::Drink;
use crate::db::Db;

pub struct DrinksRepository {
    pub drinks: Vec<Drink>,
    pub db: Db,
}

impl DrinksRepository {
    pub fn new() -> DrinksRepository {
        DrinksRepository {
            drinks: vec![],
            db: Db::new(),
        }
    }
    pub fn get_drinks(&self, drink_deleted: bool) -> Vec<Drink> {
        use crate::schema::drinks::dsl::*;

        let results = drinks
            .filter(deleted.eq(drink_deleted))
            .load::<Drink>(&self.db.connection)
            .expect("Error loading drinks");

        results
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

    pub fn list(&self, show_deleted: bool) -> Vec<&Drink> {
        return self.drinks
            .iter()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn list_mut(&mut self, show_deleted: bool) -> Vec<&mut Drink> {
        return self.drinks
            .iter_mut()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn add(&mut self, name: String, colour: String) -> i32 {
        let id = self.drinks.len() as i32 + 1;

        self.drinks.push(
            Drink {
                id: id,
                name: name,
                // count: 0,
                colour: colour,
                deleted: false,
            }
        );

        return id;
    }
    pub fn find_by_name(&mut self, name: String) -> Option<&mut Drink> {
        for drink in self.drinks.iter_mut() {
            if name == drink.name {
                return Some(drink);
            }
        }

        return None;
    }
    // pub fn find_by_index(&mut self, index: usize, include_deleted: bool) -> Option<&mut Drink> {
    //     let mut drinks = self.list_mut(include_deleted);

    //     for (i, drink) in drinks.iter_mut().enumerate() {
    //         if i == index {
    //             return Some(drink);
    //         }
    //     }

    //     return None;
    // }
    pub fn find_by_id(&mut self, id: i32) -> Option<&Drink> {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => return Some(&self.drinks[i]),
            None => return None,
        }
    }
    pub fn find_by_id_mut(&mut self, id: i32) -> Option<&mut Drink> {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => return Some(&mut self.drinks[i]),
            None => return None,
        }
    }
    pub fn delete_by_id(&mut self, id: i32, hard_delete: bool) {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => match hard_delete {
                true => {self.drinks.remove(i);},
                false => {self.drinks[i].deleted = true;},
            }
            None => panic!("Drink does not exist"),
        }
    }
}
