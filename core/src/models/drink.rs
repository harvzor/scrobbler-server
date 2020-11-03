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

use diesel::sql_types::Integer;
use diesel::sql_types::BigInt;
use diesel::sql_types::Text;
use diesel::sql_types::Bool;

#[derive(Clone, Debug, QueryableByName)]
pub struct DrinkWithCount {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    /// Calculated property which comes from the `core::models::drink_drank::DrankDrank`s of this `Drink`.
    #[sql_type = "BigInt"]
    pub count: i64,
    #[sql_type = "Text"]
    pub colour: String,
    #[sql_type = "Bool"]
    pub deleted: bool,
}
