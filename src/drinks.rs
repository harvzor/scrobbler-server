use crate::drink;

#[derive(Debug)]
pub struct Drinks {
    pub drinks: Vec<drink::Drink>
}

impl Drinks {
    pub fn new() -> Drinks {
        return Drinks {
            drinks: vec![]
        }
    }
    pub fn list(&self, show_deleted: bool) -> Vec<&drink::Drink> {
        return self.drinks
            .iter()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn list_mut(&mut self, show_deleted: bool) -> Vec<&mut drink::Drink> {
        return self.drinks
            .iter_mut()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn add(&mut self, name: String) {
        self.drinks.push(
            drink::Drink {
                id: self.drinks.len() + 1,
                name: name,
                count: 1,
                deleted: false,
            }
        );
    }
    pub fn find_by_name(&mut self, name: String) -> Option<&mut drink::Drink> {
        for drink in self.drinks.iter_mut() {
            if name == drink.name {
                return Some(drink);
            }
        }

        return None;
    }
    // pub fn find_by_index(&mut self, index: usize, include_deleted: bool) -> Option<&mut drink::Drink> {
    //     let mut drinks = self.list_mut(include_deleted);

    //     for (i, drink) in drinks.iter_mut().enumerate() {
    //         if i == index {
    //             return Some(drink);
    //         }
    //     }

    //     return None;
    // }
    pub fn delete_by_id(&mut self, id: usize, hard_delete: bool) {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => match hard_delete {
                true => {self.drinks.remove(i);},
                false => {self.drinks[i].deleted = true;},
            }
            None => panic!("Drink does not exist"),
        }
    }
}
