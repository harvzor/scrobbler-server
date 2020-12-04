use core::models::drink_drank::DrinkDrank;
use rocket::request::FromFormValue;
use rocket::http::RawStr;

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

#[derive(Serialize, Deserialize)]
pub struct DrinkDrankPostDto {
    pub drink_id: i32,
}

#[derive(FromForm)]
pub struct DrinkDrankGetDto {
    pub skip: Option<i64>,
    pub take: Option<i64>,
    pub from: Option<NaiveDateTimeForm>,
    pub to: Option<NaiveDateTimeForm>,
}

use std::ops::Deref;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::NaiveDateTime;

// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
// https://github.com/SergioBenitez/Rocket/issues/602#issuecomment-380497269
pub struct NaiveDateForm(NaiveDate);
pub struct NaiveTimeForm(NaiveTime);
pub struct NaiveDateTimeForm(NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(NaiveDateForm(date));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M:%S%.3f") {
            // if time.nanosecond() >= 1_000_000_000 {
            //     return Err(form_value);
            // }
            return Ok(NaiveTimeForm(time));
        }
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M") {
            return Ok(NaiveTimeForm(time));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveDateTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTimeForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if decoded.len() < "0000-00-00T00:00".len() {
            return Err(form_value)
        }
        let date = NaiveDateForm::from_form_value(RawStr::from_str(&decoded[.."0000-00-00".len()]))
            .map_err(|_| form_value)?;
        let time = NaiveTimeForm::from_form_value(RawStr::from_str(&decoded["0000-00-00T".len()..]))
            .map_err(|_| form_value)?;
        Ok(NaiveDateTimeForm(NaiveDateTime::new(*date, *time)))
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

impl Deref for NaiveTimeForm {
    type Target = NaiveTime;
    fn deref(&self) -> &NaiveTime {
        &self.0
    }
}

impl Deref for NaiveDateTimeForm {
    type Target = NaiveDateTime;
    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}
