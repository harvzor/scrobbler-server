use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::drink_dranks_repository::DrinkDranksRepository;

use crate::dtos::drink_drank_dto::*;

#[get("/")]
fn drinks_get(drink_dranks: State<Arc<Mutex<DrinkDranksRepository>>>) -> Json<Vec<DrinkDrankDto>> {
    let drink_dranks_repo = &mut *drink_dranks.lock().unwrap();

    return Json(
        drink_dranks_repo
            .get_drink_dranks()
            .iter()
            .map(|d| DrinkDrankDto::from_drink_drank(d))
            .collect()
    );
}

pub fn get_routes() -> Vec<Route> {
    return routes![drinks_get];
}
