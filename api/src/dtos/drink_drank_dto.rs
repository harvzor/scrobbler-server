use core::models::drink_drank::DrinkDrank;

#[derive(Serialize)]
pub struct DrinkDrankDto {
    pub id: i32,
    pub drink_id: i32,
    pub drank_timestamp: chrono::NaiveDateTime,
}

impl DrinkDrankDto {
    pub fn from_drink_drank(drink_drank: &DrinkDrank) -> DrinkDrankDto {
        DrinkDrankDto {
            id: drink_drank.id,
            drink_id: drink_drank.drink_id,
            drank_timestamp: drink_drank.drank_timestamp,
        }
    }
}
