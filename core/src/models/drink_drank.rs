use std::time::{SystemTime, UNIX_EPOCH};

/// An instance of something that was drink at some time.
#[derive(Clone, Serialize, Debug)]
pub struct DrinkDrank {
    pub id: usize,
    pub drink_id: usize,
    /// When it was drank, as seconds since the unix epoch.
    pub timestamp: u64,
}

impl DrinkDrank {
    pub fn create(drink_id: usize) -> DrinkDrank {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        DrinkDrank {
            id: 0,
            drink_id: drink_id,
            timestamp: since_the_epoch.as_secs(),
        }
    }
}
