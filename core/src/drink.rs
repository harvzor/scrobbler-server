#[derive(Clone, Serialize, Debug, Queryable)]
pub struct Drink {
    pub id: i32,
    pub name: String,
    // count: u32,
    pub colour: String,
    pub deleted: bool,
}

impl Drink {
    pub fn increment(&mut self) {
        // self.count = self.count + 1;
    }
}
