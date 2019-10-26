use std::io::{stdin,stdout,Write};
mod drinks;

fn main() {
    let mut drinks = drinks::Drinks::new();

    loop {
        let mut user_input = String::new();

        print!("Drunk a drink? : ");

        let _ = stdout().flush();

        stdin().read_line(&mut user_input)
            .expect("Did not enter a correct string");

        user_input = clean_user_input(user_input);

        let mut drink = drinks.find(user_input.clone());

        // if let Some(x) = &mut drink {
        //     x.increment();
        // } else {
        //     drinks.add(
        //         drinks::Drink {
        //             name: user_input,
        //             count: 0,
        //         }
        //     );
        // }

        match drink {
            Some(x) => x.increment(),
            None => drinks.add(
                drinks::Drink {
                    name: user_input,
                    count: 0,
                }
            )
        }

        println!("{:#?}", drinks);
    }
}

fn clean_user_input(mut user_input: String) -> String {
    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input;
}
