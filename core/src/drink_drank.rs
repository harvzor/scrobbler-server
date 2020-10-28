use std::time::{SystemTime};

/// An instance of something that was drink at some time.
#[derive(Clone, Serialize, Debug)]
pub struct DrinkDrank {
    pub id: usize,
    pub drinkId: usize,
    pub timestamp: usize,
}

impl DrinkDrank {
    pub fn create(drinkId: usize) -> DrinkDrank {
        DrinkDrank {
            id: 0,
            drinkId: drinkId,
            timestamp: SystemTime::now().as_secs(),
        }
    }
}
