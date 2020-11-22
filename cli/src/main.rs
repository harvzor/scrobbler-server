use cli::Cli;

fn main() {
    let cli = Cli::new();

    loop {
        cli.run();
    }
}
