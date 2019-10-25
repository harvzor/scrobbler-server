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
