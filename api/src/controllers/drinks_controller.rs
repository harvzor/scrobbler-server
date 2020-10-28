use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::drinks;
use core::drink;

#[get("/")]
fn drinks_get(drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Json<Vec<drink::Drink>> {
    let my_drinks = &mut *drinks.lock().unwrap();

    return Json(
        my_drinks
            .list(false)
            .iter_mut()
            .map(|d| {
                return d.clone();
            })
            .collect()
    );
}

// Should use body params.
#[post("/<name>")]
fn drink_post(name: String, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Json<drink::Drink> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let id = my_drinks.add(name, "red".to_string());

    let drink = my_drinks.find_by_id(id)
        .unwrap();

    return Json(drink.clone());
}

#[get("/<id>")]
fn drink_get(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Option<Json<drink::Drink>> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let drink = my_drinks.find_by_id(id);

    match drink {
        Some(d) => Some(Json(d.clone())),
        None => None,
    }
}

// Should use body params.
#[patch("/<id>")]
fn drink_patch(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Option<Json<drink::Drink>> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let drink = my_drinks.find_by_id_mut(id);

    match drink {
        Some(d) => {
            d.increment();

            return Some(Json(d.clone()));
        },
        None => None,
    }
}

// Implement soft/hard delete?
#[delete("/<id>")]
fn drink_delete(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) {
    let my_drinks = &mut *drinks.lock().unwrap();

    my_drinks.delete_by_id(id, false);
}

pub fn get_routes() -> Vec<Route> {
    return routes![drinks_get, drink_post, drink_get, drink_patch, drink_delete];
}
