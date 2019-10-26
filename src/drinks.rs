#[derive(Debug)]
pub struct Drink {
    pub name: String,
    pub count: u32
}

impl Drink {
    pub fn increment(&mut self) {
        self.count = self.count + 1;
    }
}

#[derive(Debug)]
pub struct Drinks {
    pub drinks: Vec<Drink>
}

impl Drinks {
    pub fn new() -> Drinks {
        return Drinks {
            drinks: vec![]
        }
    }
    pub fn add(&mut self, drink: Drink) {
        self.drinks.push(drink);
    }
    pub fn find(&mut self, name: String) -> Option<&mut Drink> {
        for drink in self.drinks.iter_mut() {
            if name == drink.name {
                return Some(drink);
            }
        }

        return None;
    }
}
