#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod impls;
mod cli;

#[get("/")]
fn index() -> &'static str {
    return "Drinks Drunk!";
}

#[get("/drinks")]
fn drinks() -> String {
    let mut drinks = impls::drinks::Drinks::new();

    return format!("{:#?}", drinks.list(false));
}

#[get("/drinks/<id>")]
fn drink(id: usize) -> String {
    let mut drinks = impls::drinks::Drinks::new();

    return format!("{:#?}", drinks.find_by_id(id));
}

fn main() {
    let mut drinks = impls::drinks::Drinks::new();

    rocket::ignite()
        .mount("/", routes![index, drinks, drink])
        .mount("/drinks", routes![drinks, drink])
        .launch();

    // loop {
    //     cli::cli(&mut drinks);
    // }
}
