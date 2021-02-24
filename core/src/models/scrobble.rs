/// When a trackable has been scrobbled.
#[derive(Clone, Debug, Queryable)]
pub struct Scrobble {
    pub id: i32,
    pub trackable_id: i32,
    /// When this was scrobbled.
    pub timestamp: chrono::NaiveDateTime,
}

impl Scrobble {
}

use crate::schema::scrobbles;

#[derive(Insertable)]
#[table_name="scrobbles"]
pub struct NewScrobble {
    pub trackable_id: i32,
    /// When this was scrobbled.
    pub timestamp: chrono::NaiveDateTime
}
