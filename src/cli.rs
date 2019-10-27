use dialoguer::{theme::ColorfulTheme, Select, Input};
use crate::drink;
use crate::drinks;

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
            Action::ListDrinks => println!("{:#?}", drinks),
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

fn menu_delete_drink(drinks: &mut drinks::Drinks) {
    let options: Vec<&String> = drinks.drinks
        .iter()
        .map(|x| &x.name)
        .rev()
        .collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Delete drink")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    drinks.remove_by_index(user_selection);
}
