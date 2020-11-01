use core::drinks_repository::DrinksRepository;
use core::drink_dranks_repository::DrinkDranksRepository;
use dialoguer::{theme::ColorfulTheme, Select, Input};

enum Action {
    ListDrinks,
    AddDrink,
    IncrementDrink,
    DeleteDrink,
}

pub fn run() {
    let drink_dranks_repository = DrinkDranksRepository::new();
    let mut drinks = DrinksRepository::new();
    let user_action = menu_get_action();

    match user_action {
        Some(action) => match action {
            Action::ListDrinks => menu_list_drinks(&mut drinks),
            Action::AddDrink => menu_add_drink(&mut drinks),
            Action::IncrementDrink => menu_increment_drink(&mut drinks, drink_dranks_repository),
            Action::DeleteDrink => menu_delete_drink(&mut drinks),
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

fn menu_list_drinks(drinks: &mut DrinksRepository) {
    println!("{:#?}", drinks.get_drinks(false));
}

fn menu_add_drink(drinks: &mut DrinksRepository) {
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

            drinks.create_drink(&drink_name, &drink_colour);
        },
    }
}

fn menu_increment_drink(drinks_repository: &DrinksRepository, drink_dranks_repository: DrinkDranksRepository) {
    let drinks = drinks_repository.get_drinks(false);

    if drinks.len() == 0 {
        println!("No drinks to increment!");

        return;
    }

    let options: Vec<&String> = drinks
        .iter()
        .map(|x| &x.name)
        .collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select drink")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap() as i32;

    drink_dranks_repository.create_drink_drank(user_selection);
}

fn menu_delete_drink(drinks: &mut DrinksRepository) {
    let drink_items = drinks.get_drinks(false);

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

    drinks.delete_drink(id, false);
}
