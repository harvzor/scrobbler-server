use std::io::{stdin,stdout,Write};

#[derive(Debug)]
struct Drink<'a> {
    name: &'a str,
    count: u32
}

impl Drink<'_> {
    fn increment(&mut self) {
        self.count = self.count + 1;
    }
}


fn main() {
    let drinks: &mut [&Drink] = &mut [
        &mut Drink {
            name: "Coffee",
            count: 0,
        }
    ];

    loop {
        let mut s = String::new();

        print!("Drunk a drink? : ");

        let _ = stdout().flush();

        stdin().read_line(&mut s)
            .expect("Did not enter a correct string");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        for drink in drinks.iter_mut() {
            if s == drink.name {
                *drink.increment();
            }
        }

        println!("{:#?}", drinks[0]);
    }
}
