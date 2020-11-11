use rocket_contrib::json::{Json};
use rocket::Route;

use crate::dtos::health_dto::*;

#[get("/")]
fn health_get() -> Json<HealthDto> {
    Json(
        HealthDto {
            status: "Healthy".to_string(),
        }
    )
}

pub fn get_routes() -> Vec<Route> {
    return routes![health_get];
}
