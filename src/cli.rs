use std::io::{stdin,stdout,Write};
use crate::drink;
use crate::drinks;

pub fn cli(drinks: &mut drinks::Drinks) {
    let action = menu();

    match action {
        Some(action_name) => match action_name.as_str() {
            "list_drinks" => println!("{:#?}", drinks),
            "add_drink" => menu_add_drink(drinks),
            "increment_drink" => menu_increment_drink(drinks),
            _ => println!("??")
        },
        None => println!("??")
    }
}

fn menu_add_drink(drinks: &mut drinks::Drinks) {
    println!("");
    println!("## Add drink");
    print!("Drink name: ");

    let user_input = get_user_input();

    let drink = drinks.find(user_input.clone());

    match drink {
        Some(_x) => println!("Drink already exists!"),
        None => drinks.add(
            drink::Drink {
                name: user_input,
                count: 1,
            }
        )
    }
}

fn menu_increment_drink(drinks: &mut drinks::Drinks) {
    println!("");
    println!("## Increment drink");

    for (i, drink) in drinks.drinks.iter().enumerate() {
        println!("{}: {}", i+1, drink.name)
    }

    println!("Select a drink: ");

    let user_input = get_user_input();

    let mut user_input_index: usize = user_input.parse().unwrap();
    user_input_index = user_input_index - 1;

    let drink = drinks.find_by_index(user_input_index);

    drink.increment();
}

fn menu() -> Option<String> {
    println!("");
    println!("## Menu");
    println!("1: List drinks");
    println!("2: Add drink");
    println!("3: Increment drink");

    print!("Your option: ");

    let user_input = get_user_input();

    match user_input.as_str() {
        "1" => Some("list_drinks".to_string()),
        "2" => Some("add_drink".to_string()),
        "3" => Some("increment_drink".to_string()),
        _ => None
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();

    let _ = stdout().flush();

    stdin().read_line(&mut user_input)
        .expect("Did not enter a correct string");

    user_input = clean_user_input(user_input);

    return user_input;
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
