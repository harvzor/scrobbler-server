use core::drinks;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::sync::{Arc, Mutex};

enum Action {
    ListDrinks,
    AddDrink,
    IncrementDrink,
    DeleteDrink,
}

pub fn run(drinks: Arc<Mutex<drinks::Drinks>>) {
    let user_action = menu_get_action();

    let my_drinks = &mut drinks.lock().unwrap();

    match user_action {
        Some(action) => match action {
            Action::ListDrinks => menu_list_drinks(my_drinks),
            Action::AddDrink => menu_add_drink(my_drinks),
            Action::IncrementDrink => menu_increment_drink(my_drinks),
            Action::DeleteDrink => menu_delete_drink(my_drinks),
        },
        None => println!("??")
    }
}

fn menu_get_action() -> Option<Action> {
    let options = &[
        "List drinks",
        "Add drink",
        "Increment drink",
        "Delete drink",
    ];

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match user_selection {
        0 => Some(Action::ListDrinks),
        1 => Some(Action::AddDrink),
        2 => Some(Action::IncrementDrink),
        3 => Some(Action::DeleteDrink),
        _ => None
    }
}

fn menu_list_drinks(drinks: &mut drinks::Drinks) {
    println!("{:#?}", drinks.list(false));
}

fn menu_add_drink(drinks: &mut drinks::Drinks) {
    let drink_name: String = Input::new()
        .with_prompt("Drink name")
        .interact()
        .unwrap();

    let drink = drinks.find_by_name(drink_name.clone());

    match drink {
        Some(_x) => println!("Drink already exists!"),
        None => {
            let drink_colour: String = Input::new()
                .with_prompt("Drink colour")
                .interact()
                .unwrap();

            drinks.add(drink_name, drink_colour);
        },
    }
}

fn menu_increment_drink(drinks: &mut drinks::Drinks) {
    let mut drink_items = drinks.list_mut(false);

    if drink_items.len() == 0 {
        println!("No drinks to increment!");

        return;
    }

    let options: Vec<&String> = drink_items
        .iter()
        .map(|x| &x.name)
        .collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select drink")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    drink_items[user_selection].increment();
}

fn menu_delete_drink(drinks: &mut drinks::Drinks) {
    let drink_items = drinks.list_mut(false);

    if drink_items.len() == 0 {
        println!("No drinks to delete!");

        return;
    }

    let options: Vec<&String> = drink_items
        .iter()
        .map(|x| &x.name)
        .collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Delete drink")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    let id = drink_items[user_selection].id;

    drinks.delete_by_id(id, false);
}
