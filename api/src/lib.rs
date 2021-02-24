#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_cors;

#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

use rocket::http::Method;
use rocket_cors::{
    // AllowedHeaders,
    AllowedOrigins,
    // Error,
    Cors,
    CorsOptions,
};

mod controllers {
    pub mod health_controller;
    pub mod trackables_controller;
    pub mod scrobbles_controller;
}

mod dtos {
    pub mod health_dto;
    pub mod trackable_dto;
    pub mod scrobble_dto;
}

pub struct Api {
}

#[get("/")]
fn index() -> String {
    return format!("Scrobbler!");
}

fn make_cors() -> Cors {
    // let allowed_origins = AllowedOrigins::some_exact(&[
    //     "http://localhost:8080",
    // ]);

    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&[
        //     "Authorization",
        //     "Accept",
        //     "Access-Control-Allow-Origin",
        //     "Access-Control-Request-Headers",
        // ]),
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
        let trackables_repository = Arc::new(Mutex::new(core::trackables_repository::TrackablesRepository::new()));
        let scrobbles_repository = Arc::new(Mutex::new(core::scrobbles_repository::ScrobblesRepository::new()));

        rocket::ignite()
            .manage(trackables_repository)
            .manage(scrobbles_repository)
            .mount("/", routes![index])
            .mount("/health", controllers::health_controller::get_routes())
            .mount("/trackables", controllers::trackables_controller::get_routes())
            .mount("/scrobbles", controllers::scrobbles_controller::get_routes())
            .attach(make_cors())
            .launch();
    }
}
