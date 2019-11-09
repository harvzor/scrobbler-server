use serde;

#[derive(Serialize, Debug)]
pub struct Drink {
    pub id: usize,
    pub name: String,
    pub count: u32,
    pub deleted: bool,
}

impl Drink {
    pub fn increment(&mut self) {
        self.count = self.count + 1;
    }
}
