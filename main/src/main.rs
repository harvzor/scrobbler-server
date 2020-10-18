#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::{Arc, Mutex};
use std::thread;

use core;
use cli::cli::cli;
use api::api::Api;

fn main() {
    // Drinks is defined here as both the API and the cli are modifying the same object.
    // Basically it's an in-memory db.
    let drinks = Arc::new(Mutex::new(core::drinks::Drinks::new()));

    let drinks_api = drinks.clone();

    thread::spawn(move || {
        Api::new(drinks_api)
            .run();
    });

    loop {
        cli(drinks.clone());
    }
}
