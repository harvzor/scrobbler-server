#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
extern crate rocket_cors;

use std::sync::{Arc, Mutex};

use rocket::http::Method;

use core::drinks_repository::DrinksRepository;

mod controllers {
    pub mod drinks_controller;
}

use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    // Error,
    Cors,
    CorsOptions,
};

pub struct Api {
    drinks_repository: Arc<Mutex<DrinksRepository>>
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
            drinks_repository: Arc::new(Mutex::new(core::drinks_repository::DrinksRepository::new())),
        };
    }
    pub fn run(&self) {
        rocket::ignite()
            .manage(self.drinks_repository.clone())
            .mount("/", routes![index])
            .mount("/drinks", controllers::drinks_controller::get_routes())
            .attach(make_cors())
            .launch();
    }
}
