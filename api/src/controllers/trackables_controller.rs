use std::sync::{Arc, Mutex};

use rocket_contrib::json::{Json};
use rocket::State;
use rocket::Route;

use core::trackables_repository::TrackablesRepository;

use crate::dtos::trackable_dto::*;

#[get("/")]
fn query(trackables: State<Arc<Mutex<TrackablesRepository>>>) -> Json<Vec<TrackableDto>> {
    let trackables_repo = &mut *trackables.lock().unwrap();

    return Json(
        trackables_repo
            .get(false)
            .iter()
            .map(|d| TrackableDto::from_trackable_with_count(d))
            .collect()
    );
}

#[post("/", format = "application/json", data = "<trackable>")]
fn create(trackable: Json<TrackablePostDto>, trackables: State<Arc<Mutex<TrackablesRepository>>>) -> Json<TrackableDto> {
    let trackables_repo = &*trackables.lock().unwrap();

    let trackable = trackables_repo.create(&trackable.name as &str, &trackable.colour as &str);

    return Json(TrackableDto::from_trackable(&trackable));
}

#[get("/<id>")]
fn get(id: i32, trackables: State<Arc<Mutex<TrackablesRepository>>>) -> Option<Json<TrackableDto>> {
    let trackables_repo = &mut *trackables.lock().unwrap();

    let trackable = trackables_repo.find_by_id(id);

    match trackable {
        Some(d) => Some(Json(TrackableDto::from_trackable(&d))),
        None => None,
    }
}

#[delete("/<id>")]
fn delete(id: i32, trackables: State<Arc<Mutex<TrackablesRepository>>>) {
    let trackables_repo = &mut *trackables.lock().unwrap();

    trackables_repo.delete(id, false);
}

pub fn get_routes() -> Vec<Route> {
    return routes![query, create, get, delete];
}
