#[derive(Debug)]
pub struct Drink<'a> {
    pub name: &'a str,
    pub count: u32
}

impl Drink<'_> {
    pub fn increment(&mut self) {
        self.count = self.count + 1;
    }
}
