#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod impls;
// mod cli;
mod api;

fn main() {
    let mut drinks = impls::drinks::Drinks::new();

    api::Api::new(&mut drinks)
        .run();

    // loop {
    //     cli::cli(&mut drinks);
    // }
}
