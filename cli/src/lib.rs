use core::trackables_repository::TrackablesRepository;
use core::scrobbles_repository::ScrobblesRepository;
use dialoguer::{theme::ColorfulTheme, Select, Input};

enum Action {
    ListTrackables,
    AddTrackable,
    IncrementTrackable,
    DeleteTrackable,
}

pub struct Cli {
    trackables_repository: TrackablesRepository,
    scrobbles_repository: ScrobblesRepository,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            trackables_repository: TrackablesRepository::new(),
            scrobbles_repository: ScrobblesRepository::new(),
        }
    }
    pub fn run(&self) {
        let user_action = self.menu_get_action();

        match user_action {
            Some(action) => match action {
                Action::ListTrackables => self.menu_list_trackables(),
                Action::AddTrackable => self.menu_add_trackable(),
                Action::IncrementTrackable => self.menu_increment_trackable(),
                Action::DeleteTrackable => self.menu_delete_trackable(),
            },
            None => println!("??")
        }
    }
    fn menu_get_action(&self) -> Option<Action> {
        let options = &[
            "List trackables",
            "Add trackable",
            "Increment trackable",
            "Delete trackable",
        ];

        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match user_selection {
            0 => Some(Action::ListTrackables),
            1 => Some(Action::AddTrackable),
            2 => Some(Action::IncrementTrackable),
            3 => Some(Action::DeleteTrackable),
            _ => None
        }
    }
    fn menu_list_trackables(&self) {
        println!("{:#?}", self.trackables_repository.get(false));
    }
    fn menu_add_trackable(&self) {
        let name: String = Input::new()
            .with_prompt("Trackable name")
            .interact()
            .unwrap();

        let trackable = self.trackables_repository.find_by_name(&name);

        match trackable {
            Some(_x) => println!("Trackable already exists!"),
            None => {
                let colour: String = Input::new()
                    .with_prompt("Trackable colour")
                    .interact()
                    .unwrap();

                self.trackables_repository.create(&name, &colour);
            },
        }
    }
    fn menu_increment_trackable(&self) {
        let trackables = self.trackables_repository.get(false);

        if trackables.len() == 0 {
            println!("No trackable to increment!");

            return;
        }

        let options: Vec<&String> = trackables
            .iter()
            .map(|x| &x.name)
            .collect();

        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select trackable")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        self.scrobbles_repository.create(trackables[user_selection].id);
    }
    fn menu_delete_trackable(&self) {
        let trackable_items = self.trackables_repository.get(false);

        if trackable_items.len() == 0 {
            println!("No trackables to delete!");

            return;
        }

        let options: Vec<&String> = trackable_items
            .iter()
            .map(|x| &x.name)
            .collect();

        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Delete trackable")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        let id = trackable_items[user_selection].id;

        self.trackables_repository.delete(id, false);
    }
}
