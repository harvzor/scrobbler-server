use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::drinks_repository::DrinksRepository;

use crate::dtos::drink_dto::DrinkDto;

#[get("/")]
fn drinks_get(drinks: State<Arc<Mutex<DrinksRepository>>>) -> Json<Vec<DrinkDto>> {
    let my_drinks = &mut *drinks.lock().unwrap();

    return Json(
        my_drinks
            .get_drinks(false)
            .iter()
            .map(|d| {
                DrinkDto::from_drink_with_count(d.clone())
            })
            .collect()
    );
}

// Should use body params.
#[post("/<name>")]
fn drink_post(name: String, drinks: State<Arc<Mutex<DrinksRepository>>>) -> Json<DrinkDto> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let drink = my_drinks.create_drink(&name, "red");

    return Json(DrinkDto::from_drink(drink));
}

#[get("/<id>")]
fn drink_get(id: i32, drinks: State<Arc<Mutex<DrinksRepository>>>) -> Option<Json<DrinkDto>> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let drink = my_drinks.find_by_id(id);

    match drink {
        Some(d) => Some(Json(DrinkDto::from_drink(d))),
        None => None,
    }
}

// // Should use body params.
// #[patch("/<id>")]
// fn drink_patch(id: i32, drinks: State<Arc<Mutex<DrinksRepository>>>) -> Option<Json<Drink>> {
//     let my_drinks = &mut *drinks.lock().unwrap();

//     let drink = my_drinks.find_by_id(id);

//     match drink {
//         Some(d) => {
//             d.increment();

//             return Some(Json(d.clone()));
//         },
//         None => None,
//     }
// }

// Implement soft/hard delete?
#[delete("/<id>")]
fn drink_delete(id: i32, drinks: State<Arc<Mutex<DrinksRepository>>>) {
    let my_drinks = &mut *drinks.lock().unwrap();

    my_drinks.delete_drink(id, false);
}

pub fn get_routes() -> Vec<Route> {
    return routes![drinks_get, drink_post, drink_get, drink_delete];
}
