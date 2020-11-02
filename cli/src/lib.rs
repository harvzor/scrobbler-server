use core::drinks_repository::DrinksRepository;
use core::drink_dranks_repository::DrinkDranksRepository;
use dialoguer::{theme::ColorfulTheme, Select, Input};

enum Action {
    ListDrinks,
    AddDrink,
    IncrementDrink,
    DeleteDrink,
}

pub struct Cli {
    drinks_repository: DrinksRepository,
    drink_dranks_repository: DrinkDranksRepository,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            drinks_repository: DrinksRepository::new(),
            drink_dranks_repository: DrinkDranksRepository::new(),
        }
    }
    pub fn run(&self) {
        let user_action = self.menu_get_action();

        match user_action {
            Some(action) => match action {
                Action::ListDrinks => self.menu_list_drinks(),
                Action::AddDrink => self.menu_add_drink(),
                Action::IncrementDrink => self.menu_increment_drink(),
                Action::DeleteDrink => self.menu_delete_drink(),
            },
            None => println!("??")
        }
    }
    fn menu_get_action(&self) -> Option<Action> {
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
    fn menu_list_drinks(&self) {
        println!("{:#?}", self.drinks_repository.get_drinks(false));
    }
    fn menu_add_drink(&self) {
        let drink_name: String = Input::new()
            .with_prompt("Drink name")
            .interact()
            .unwrap();

        let drink = self.drinks_repository.find_by_name(&drink_name);

        match drink {
            Some(_x) => println!("Drink already exists!"),
            None => {
                let drink_colour: String = Input::new()
                    .with_prompt("Drink colour")
                    .interact()
                    .unwrap();

                self.drinks_repository.create_drink(&drink_name, &drink_colour);
            },
        }
    }
    fn menu_increment_drink(&self) {
        let drinks = self.drinks_repository.get_drinks(false);

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

        self.drink_dranks_repository.create_drink_drank(user_selection);
    }
    fn menu_delete_drink(&self) {
        let drink_items = self.drinks_repository.get_drinks(false);

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

        self.drinks_repository.delete_drink(id, false);
    }
}
