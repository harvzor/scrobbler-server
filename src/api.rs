use crate::impls::drinks;

pub struct Api<'a> {
    drinks: &'a drinks::Drinks
}

#[get("/")]
fn index() -> String {
    return format!("Drinks Drunk!");
}

#[get("/")]
fn drinks_get() -> String {
    let mut drinks = drinks::Drinks::new();

    return format!("{:#?}", drinks.list(false));
}

#[get("/<id>")]
fn drink_get(id: usize) -> String {
    let mut drinks = drinks::Drinks::new();

    return format!("{:#?}", drinks.find_by_id(id));
}

#[post("/<name>")]
fn drink_post(name: String) -> String {
    let mut drinks = drinks::Drinks::new();

    drinks.add(name);

    return format!("{:#?}", drinks.find_by_id(1));
}

impl<'a> Api<'a> {
    pub fn new(drinks: &mut drinks::Drinks) -> Api {
        return Api {
            drinks: drinks
        };
    }
    pub fn run(&self) {
        rocket::ignite()
            .mount("/", routes![index])
            .mount("/drinks", routes![drinks_get, drink_get, drink_post])
            .launch();
    }
}
