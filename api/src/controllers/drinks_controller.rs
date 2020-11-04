use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::drinks_repository::DrinksRepository;

use crate::dtos::drink_dto::*;

#[get("/")]
fn drinks_get(drinks: State<Arc<Mutex<DrinksRepository>>>) -> Json<Vec<DrinkDto>> {
    let drinks_repo = &mut *drinks.lock().unwrap();

    return Json(
        drinks_repo
            .get_drinks(false)
            .iter()
            .map(|d| DrinkDto::from_drink_with_count(d))
            .collect()
    );
}

#[post("/", format = "application/json", data = "<drink>")]
fn drink_post(drink: Json<DrinkDtoPost>, drinks: State<Arc<Mutex<DrinksRepository>>>) -> Json<DrinkDto> {
    let drinks_repo = &*drinks.lock().unwrap();

    let drink = drinks_repo.create_drink(&drink.name as &str, &drink.colour as &str);

    return Json(DrinkDto::from_drink(&drink));
}

#[get("/<id>")]
fn drink_get(id: i32, drinks: State<Arc<Mutex<DrinksRepository>>>) -> Option<Json<DrinkDto>> {
    let drinks_repo = &mut *drinks.lock().unwrap();

    let drink = drinks_repo.find_by_id(id);

    match drink {
        Some(d) => Some(Json(DrinkDto::from_drink(&d))),
        None => None,
    }
}

#[delete("/<id>")]
fn drink_delete(id: i32, drinks: State<Arc<Mutex<DrinksRepository>>>) {
    let drinks_repo = &mut *drinks.lock().unwrap();

    drinks_repo.delete_drink(id, false);
}

pub fn get_routes() -> Vec<Route> {
    return routes![drinks_get, drink_post, drink_get, drink_delete];
}
