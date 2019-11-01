mod impls;
mod cli;

fn main() {
    let mut drinks = impls::drinks::Drinks::new();

    loop {
        let drinks_ref = &mut drinks;

        cli::cli(drinks_ref);
    }
}
