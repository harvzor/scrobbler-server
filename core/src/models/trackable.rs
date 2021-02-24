#[derive(Clone, Debug, Queryable)]
pub struct Trackable {
    pub id: i32,
    pub name: String,
    pub colour: String,
    pub deleted: bool,
}

impl Trackable {
}

use crate::schema::trackables;

#[derive(Insertable)]
#[table_name="trackables"]
pub struct NewTrackable<'a> {
    pub name: &'a str,
    pub colour: &'a str,
}

#[derive(Clone, Debug)]
pub struct TrackableWithCount {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub colour: String,
    pub deleted: bool,
}

impl TrackableWithCount {
    pub fn from_trackable_with_count(trackable: &Trackable, count: i32) -> TrackableWithCount {
        TrackableWithCount {
            id: trackable.id,
            name: trackable.name.clone(),
            count: count,
            colour: trackable.colour.clone(),
            deleted: trackable.deleted,
        }
    }
}
