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

#[derive(Clone, Debug)]
pub struct DrinkWithCount {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub colour: String,
    pub deleted: bool,
}

impl DrinkWithCount {
    pub fn from_drink_with_count(drink: &Drink, count: i32) -> DrinkWithCount {
        DrinkWithCount {
            id: drink.id,
            name: drink.name.clone(),
            count: count,
            colour: drink.colour.clone(),
            deleted: drink.deleted,
        }
    }
}
