use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::request::Form;
use rocket::State;
use rocket::Route;

use core::scrobbles_repository::ScrobblesRepository;

use crate::dtos::scrobble_dto::*;

#[get("/?<scrobble_get_dto..>")]
fn get(scrobble_get_dto: Form<ScrobbleGetDto>, scrobbles: State<Arc<Mutex<ScrobblesRepository>>>) -> Json<Vec<ScrobbleDto>> {
    let scrobbles_repo = &mut *scrobbles.lock().unwrap();

    let skip = match scrobble_get_dto.skip {
        Some(skip) => skip,
        None => 0
    };

    let take= match scrobble_get_dto.take {
        Some(take) => take,
        None => 10
    };

    let from = match &scrobble_get_dto.from {
        Some(from) => Some(**from),
        None => None
    };

    let to= match &scrobble_get_dto.to {
        Some(to) => Some(**to),
        None => None
    };

    return Json(
        scrobbles_repo
            .get(skip, take, from, to)
            .iter()
            .map(|d| ScrobbleDto::from_scrobble(d))
            .collect()
    );
}

#[post("/", format = "application/json", data = "<scrobble_post_dto>")]
fn create(scrobble_post_dto: Json<ScrobblePostDto>, scrobbles: State<Arc<Mutex<ScrobblesRepository>>>) -> Json<ScrobbleDto> {
    let scrobbles_repo = &*scrobbles.lock().unwrap();

    let scrobble = scrobbles_repo.create(scrobble_post_dto.trackable_id);

    return Json(ScrobbleDto::from_scrobble(&scrobble));
}

pub fn get_routes() -> Vec<Route> {
    return routes![get, create];
}
