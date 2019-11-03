use crate::impls::drinks;
use std::sync::{Arc, Mutex};
use rocket::State;

pub struct Api {
    drinks: Arc<Mutex<drinks::Drinks>>
}

#[get("/")]
fn index() -> String {
    return format!("Drinks Drunk!");
}

#[get("/")]
fn drinks_get(drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let mut my_drinks = &mut *drinks.lock().unwrap();

    return format!("{:#?}", my_drinks.list(false));
}

#[get("/<id>")]
fn drink_get(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let mut my_drinks = &mut *drinks.lock().unwrap();

    return format!("{:#?}", my_drinks.find_by_id(id));
}

#[post("/<name>")]
fn drink_post(name: String, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let mut my_drinks = &mut *drinks.lock().unwrap();

    my_drinks.add(name);

    return format!("{:#?}", my_drinks.find_by_id(1));
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
            .mount("/drinks", routes![drinks_get, drink_get, drink_post])
            .manage(self.drinks.clone())
            .launch();
    }
}
