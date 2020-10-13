use core::drinks;
use core::drink;

use std::sync::{Arc, Mutex};

use rocket::State;
use rocket_contrib::json::{Json};
use rocket::http::Method;

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

// // Should use body params.
// #[post("/<name>")]
// fn drink_post(name: String, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Json<&drink::Drink> {
//     let my_drinks = &mut drinks.lock().unwrap();

//     let id = my_drinks.add(name);

//     let drink = my_drinks.find_by_id(id)
//         .unwrap();

//     return Json(&drink);
// }

#[post("/<name>")]
fn drink_post(name: String, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> Json<drink::Drink> {
    let my_drinks = &mut *drinks.lock().unwrap();

    let id = my_drinks.add(name);

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
            .mount("/drinks", routes![drinks_get, drink_post, drink_get, drink_patch, drink_delete])
            .manage(self.drinks.clone())
            .attach(make_cors())
            .launch();
    }
}
