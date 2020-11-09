use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::drink_dranks_repository::DrinkDranksRepository;

use crate::dtos::drink_drank_dto::*;

#[get("/")]
fn drink_dranks_get(drink_dranks: State<Arc<Mutex<DrinkDranksRepository>>>) -> Json<Vec<DrinkDrankDto>> {
    let drink_dranks_repo = &mut *drink_dranks.lock().unwrap();

    return Json(
        drink_dranks_repo
            .get_drink_dranks()
            .iter()
            .map(|d| DrinkDrankDto::from_drink_drank(d))
            .collect()
    );
}

#[post("/", format = "application/json", data = "<drink_drank_post_dto>")]
fn drink_dranks_post(drink_drank_post_dto: Json<DrinkDrankPostDto>, drink_dranks: State<Arc<Mutex<DrinkDranksRepository>>>) -> Json<DrinkDrankDto> {
    let drink_dranks_repo = &*drink_dranks.lock().unwrap();

    let drink_drank = drink_dranks_repo.create_drink_drank(drink_drank_post_dto.drink_id);

    return Json(DrinkDrankDto::from_drink_drank(&drink_drank));
}

pub fn get_routes() -> Vec<Route> {
    return routes![drink_dranks_get, drink_dranks_post];
}
