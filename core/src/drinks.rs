use crate::drink::Drink;

#[derive(Serialize, Debug)]
pub struct Drinks {
    pub drinks: Vec<Drink>
}

impl Drinks {
    pub fn new() -> Drinks {
        return Drinks {
            drinks: vec![]
        }
    }
    pub fn list(&self, show_deleted: bool) -> Vec<&Drink> {
        return self.drinks
            .iter()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn list_mut(&mut self, show_deleted: bool) -> Vec<&mut Drink> {
        return self.drinks
            .iter_mut()
            .filter(|drink| drink.deleted == show_deleted)
            .collect();
    }
    pub fn add(&mut self, name: String) -> usize {
        let id = self.drinks.len() + 1;

        self.drinks.push(
            Drink {
                id: id,
                name: name,
                count: 1,
                deleted: false,
            }
        );

        return id;
    }
    pub fn find_by_name(&mut self, name: String) -> Option<&mut Drink> {
        for drink in self.drinks.iter_mut() {
            if name == drink.name {
                return Some(drink);
            }
        }

        return None;
    }
    // pub fn find_by_index(&mut self, index: usize, include_deleted: bool) -> Option<&mut Drink> {
    //     let mut drinks = self.list_mut(include_deleted);

    //     for (i, drink) in drinks.iter_mut().enumerate() {
    //         if i == index {
    //             return Some(drink);
    //         }
    //     }

    //     return None;
    // }
    pub fn find_by_id(&mut self, id: usize) -> Option<&Drink> {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => return Some(&self.drinks[i]),
            None => return None,
        }
    }
    pub fn find_by_id_clone(&mut self, id: usize) -> Option<Drink> {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => return Some(self.drinks[i].clone()),
            None => return None,
        }
    }
    pub fn find_by_id_mut(&mut self, id: usize) -> Option<&mut Drink> {
        let index = self.drinks
            .iter()
            .position(|x| x.id == id);

        match index {
            Some(i) => return Some(&mut self.drinks[i]),
            None => return None,
        }
    }
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
