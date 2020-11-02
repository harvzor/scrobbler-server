#[derive(Clone, Debug, Queryable)]
pub struct Drink {
    pub id: i32,
    pub name: String,
    pub colour: String,
    pub deleted: bool,
}

impl Drink {
}

use crate::schema::drinks;

#[derive(Insertable)]
#[table_name="drinks"]
pub struct NewDrink<'a> {
    pub name: &'a str,
    pub colour: &'a str,
}

#[derive(Clone, Debug, Queryable)]
pub struct DrinkWithCount {
    pub id: i32,
    pub name: String,
    /// Calculated property which comes from the `core::models::drink_drank::DrankDrank`s of this `Drink`.
    pub count: i32,
    pub colour: String,
    pub deleted: bool,
}
