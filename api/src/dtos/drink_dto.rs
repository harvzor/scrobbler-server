use core::models::drink::Drink;

#[derive(Serialize)]
pub struct DrinkDto {
    pub id: i32,
    pub name: String,
    /// Calculated property which comes from the `core::models::drink_drank::DrankDrank`s of this `Drink`.
    pub count: u32,
    pub colour: String,
    pub deleted: bool,
}

impl DrinkDto {
    pub fn from_drink(drink: Drink) -> DrinkDto {
        DrinkDto {
            id: drink.id,
            name: drink.name,
            count: 0,
            colour: drink.colour,
            deleted: drink.deleted,
        }
    }
}
