#![feature(proc_macro_hygiene, decl_macro)]

use std::thread;

use cli::Cli;
use api::Api;

fn main() {
    thread::spawn(move || {
        Api::new().run();
    });

    let cli = Cli::new();

    loop {
        cli.run();
    }
}
