mod drink;
mod drinks;
mod cli;

fn main() {
    let mut drinks = drinks::Drinks::new();

    loop {
        let drinks_ref = &mut drinks;

        cli::cli(drinks_ref);
    }
}
