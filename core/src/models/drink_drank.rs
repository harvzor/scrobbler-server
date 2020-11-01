/// An instance of something that was drink at some time.
#[derive(Clone, Serialize, Debug, Queryable)]
pub struct DrinkDrank {
    pub id: i32,
    pub drink_id: i32,
    /// When the drink was drank.
    pub drank_timestamp: chrono::NaiveDateTime,
}

impl DrinkDrank {
}

use crate::schema::drink_dranks;

#[derive(Insertable)]
#[table_name="drink_dranks"]
pub struct NewDrinkDrank {
    pub drink_id: i32,
    pub drank_timestamp: chrono::NaiveDateTime
}
