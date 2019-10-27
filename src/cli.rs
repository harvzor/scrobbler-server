use dialoguer::{theme::ColorfulTheme, Select, Input};
use crate::drink;
use crate::drinks;

pub fn cli(drinks: &mut drinks::Drinks) {
    let user_action = menu_get_action();

    match user_action {
        Some(action_name) => match action_name.as_str() {
            "list_drinks" => println!("{:#?}", drinks),
            "add_drink" => menu_add_drink(drinks),
            "increment_drink" => menu_increment_drink(drinks),
            _ => println!("??")
        },
        None => println!("??")
    }
}

fn menu_get_action() -> Option<String> {
    let options = &[
        "List drinks",
        "Add drink",
        "Increment drink",
    ];

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match user_selection {
        0 => Some("list_drinks".to_string()),
        1 => Some("add_drink".to_string()),
        2 => Some("increment_drink".to_string()),
        _ => None
    }
}

fn menu_add_drink(drinks: &mut drinks::Drinks) {
    let user_input: String = Input::new()
        .with_prompt("Drink name")
        .interact()
        .unwrap();

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
    let options: Vec<&String> = drinks.drinks
        .iter()
        .map(|x| &x.name)
        .rev()
        .collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select drink")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    let drink = drinks.find_by_index(user_selection);

    drink.increment();
}
