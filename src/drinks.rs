use crate::drink;

#[derive(Debug)]
pub struct Drinks {
    drinks: Vec<drink::Drink>
}

impl Drinks {
    pub fn new() -> Drinks {
        return Drinks {
            drinks: vec![]
        }
    }
    pub fn add(&mut self, drink: drink::Drink) {
        self.drinks.push(drink);
    }
    pub fn find(&mut self, name: String) -> Option<&mut drink::Drink> {
        for drink in self.drinks.iter_mut() {
            if name == drink.name {
                return Some(drink);
            }
        }

        return None;
    }
}
