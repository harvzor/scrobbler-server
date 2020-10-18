use std::sync::{Arc, Mutex};

// use rocket::State;
// use rocket_contrib::json::{Json};
use rocket::http::Method;

use core::drinks;

use crate::DrinksController::DrinksController;

use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    // Error,
    Cors,
    CorsOptions,
};

pub struct Api {
    drinks: Arc<Mutex<drinks::Drinks>>
}

#[get("/")]
fn index() -> String {
    return format!("Drinks Drunk!");
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

impl Api {
    pub fn new(drinks: Arc<Mutex<drinks::Drinks>>) -> Api {
        return Api {
            drinks: drinks
        };
    }
    pub fn run(&self) {
        rocket::ignite()
            .mount("/", routes![index])
            .mount("/drinks", DrinksController::get_routes())
            .manage(self.drinks.clone())
            .attach(make_cors())
            .launch();
    }
}
