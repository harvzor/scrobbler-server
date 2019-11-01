use dialoguer::{theme::ColorfulTheme, Select, Input};
use crate::impls::drinks;

enum Action {
    ListDrinks,
    AddDrink,
    IncrementDrink,
    DeleteDrink,
}

pub fn cli(drinks: &mut drinks::Drinks) {
    let user_action = menu_get_action();

    match user_action {
        Some(action) => match action {
            Action::ListDrinks => menu_list_drinks(drinks),
            Action::AddDrink => menu_add_drink(drinks),
            Action::IncrementDrink => menu_increment_drink(drinks),
            Action::DeleteDrink => menu_delete_drink(drinks),
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
    let user_input: String = Input::new()
        .with_prompt("Drink name")
        .interact()
        .unwrap();

    let drink = drinks.find_by_name(user_input.clone());

    match drink {
        Some(_x) => println!("Drink already exists!"),
        None => drinks.add(user_input),
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
