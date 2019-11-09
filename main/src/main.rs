#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::{Arc, Mutex};
use std::thread;

use core;
use cli::cli::cli;
use api::api::Api;

fn main() {
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
