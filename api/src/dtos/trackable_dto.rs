use core::models::trackable::Trackable;
use core::models::trackable::TrackableWithCount;

#[derive(Serialize)]
pub struct TrackableDto {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub colour: String,
    pub deleted: bool,
}

impl TrackableDto {
    pub fn from_trackable(trackable: &Trackable) -> TrackableDto {
        TrackableDto {
            id: trackable.id,
            name: trackable.name.clone(),
            count: 0,
            colour: trackable.colour.clone(),
            deleted: trackable.deleted,
        }
    }
    pub fn from_trackable_with_count(trackable_with_count: &TrackableWithCount) -> TrackableDto {
        TrackableDto {
            id: trackable_with_count.id,
            name: trackable_with_count.name.clone(),
            count: trackable_with_count.count,
            colour: trackable_with_count.colour.clone(),
            deleted: trackable_with_count.deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrackablePostDto {
    pub name: String,
    pub colour: String,
}
