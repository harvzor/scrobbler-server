#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::{Arc, Mutex};

mod impls;
mod cli;
mod api;

fn main() {
    let drinks = Arc::new(Mutex::new(impls::drinks::Drinks::new()));

    api::Api::new(drinks.clone())
        .run();

    loop {
        cli::cli(drinks.clone());
    }
}
