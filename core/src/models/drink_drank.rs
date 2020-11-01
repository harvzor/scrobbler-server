/// An instance of something that was drink at some time.
#[derive(Clone, Serialize, Debug, Queryable)]
pub struct DrinkDrank {
    pub id: i32,
    pub drink_id: i32,
    /// When it was drank, as seconds since the unix epoch.
    pub drank_timestamp: chrono::NaiveDateTime,
}

impl DrinkDrank {
    pub fn create(drink_id: i32) -> DrinkDrank {
        DrinkDrank {
            id: 0,
            drink_id: drink_id,
            drank_timestamp: chrono::Utc::now().naive_local(),
        }
    }
}

use crate::schema::drink_dranks;

#[derive(Insertable)]
#[table_name="drink_dranks"]
pub struct NewDrinkDrank {
    pub drink_id: i32,
    pub drank_timestamp: chrono::NaiveDateTime
}
