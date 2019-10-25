use std::io::{stdin,stdout,Write};
mod drink;

fn main() {
    let mut drinks: Vec<drink::Drink> = vec![];

    loop {
        let mut user_input = String::new();

        print!("Drunk a drink? : ");

        let _ = stdout().flush();

        stdin().read_line(&mut user_input)
            .expect("Did not enter a correct string");

        if let Some('\n') = user_input.chars().next_back() {
            user_input.pop();
        }

        if let Some('\r') = user_input.chars().next_back() {
            user_input.pop();
        }

        let mut found_drink = false;

        for drink in drinks.iter_mut() {
            if user_input == drink.name {
                drink.increment();

                found_drink = true;

                break;
            }
        }

        if !found_drink {
            drinks.push(
                drink::Drink {
                    name: user_input,
                    count: 0,
                }
            );
        }

        println!("{:#?}", drinks);
    }
}
