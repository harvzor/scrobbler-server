#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_cors;

#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    // Error,
    Cors,
    CorsOptions,
};

mod controllers {
    pub mod drinks_controller;
    pub mod drink_dranks_controller;
}

mod dtos {
    pub mod drink_dto;
    pub mod drink_drank_dto;
}

pub struct Api {
}

#[get("/")]
fn index() -> String {
    return format!("Drinks Drunk!");
}

fn make_cors() -> Cors {
    // let allowed_origins = AllowedOrigins::some_exact(&[
    //     "http://localhost:8080",
    // ]);

    let allowed_origins = AllowedOrigins::all();

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
    pub fn new() -> Api {
        return Api {
        };
    }
    pub fn run(&self) {
        let drinks_repository = Arc::new(Mutex::new(core::drinks_repository::DrinksRepository::new()));
        let drink_dranks_repository = Arc::new(Mutex::new(core::drink_dranks_repository::DrinkDranksRepository::new()));

        rocket::ignite()
            .manage(drinks_repository)
            .manage(drink_dranks_repository)
            .mount("/", routes![index])
            .mount("/drinks", controllers::drinks_controller::get_routes())
            .mount("/drink-dranks", controllers::drink_dranks_controller::get_routes())
            .attach(make_cors())
            .launch();
    }
}
