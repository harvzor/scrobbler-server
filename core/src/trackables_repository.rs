use diesel::prelude::*;

use crate::models::trackable::Trackable;
use crate::models::trackable::TrackableWithCount;
use crate::db::Db;

pub struct TrackablesRepository {
    pub db: Db,
}

impl TrackablesRepository {
    pub fn new() -> TrackablesRepository {
        TrackablesRepository {
            db: Db::new(),
        }
    }
    pub fn get(&self, trackable_deleted: bool) -> Vec<TrackableWithCount> {
        // Not particularly efficient.
        // Would be better to get the trackables with count in a single call but I can't get that to work.
        // Seems Diesel doesn't really support joins too well.
        // And I can't seem to get raw SQL really working either: https://github.com/Harvzor/scrobbler-server/commit/1081cec5a37cc84db242eaf669aea18a4a1b05b1

        use crate::schema::trackables::dsl::*;
        use crate::scrobbles_repository::ScrobblesRepository;

        let trackables_result = trackables
            .filter(deleted.eq(trackable_deleted))
            .load::<Trackable>(&self.db.connection)
            .expect("Error loading trackables");

        let trackable_ids = trackables_result
            .iter()
            .map(|x| x.id)
            .collect();

        let scrobble_result = ScrobblesRepository::new().get_for_trackables(trackable_ids);

        trackables_result
            .iter()
            .map(|x| {
                let count = scrobble_result
                    .iter()
                    .filter(|y| y.trackable_id == x.id)
                    .count();

                TrackableWithCount::from_trackable_with_count(x, count as i32)
            })
            .collect()
    }
    pub fn create<'a>(&self, name: &'a str, colour: &'a str) -> Trackable {
        use crate::schema::trackables;
        use crate::models::trackable::NewTrackable;

        let new_trackable = NewTrackable {
            name: name,
            colour: colour,
        };

        diesel::insert_into(trackables::table)
            .values(&new_trackable)
            .get_result(&self.db.connection)
            .expect("Error saving new trackable")
    }
    pub fn delete(&self, trackable_id: i32, hard_delete: bool) -> Option<Trackable> {
        use crate::schema::trackables::dsl::*;

        match hard_delete {
            true => {
                diesel::delete(trackables.find(id))
                    .execute(&self.db.connection)
                    .expect("Error deleting trackable");

                return None;
            },
            false => {
                let trackable = diesel::update(trackables.find(id))
                    .set(deleted.eq(true))
                    .get_result::<Trackable>(&self.db.connection)
                    .expect(&format!("Unable to find trackable {}", trackable_id));

                return Some(trackable);
            },
        }
    }
    pub fn find_by_id(&self, trackable_id: i32) -> Option<Trackable> {
        use crate::schema::trackables::dsl::*;

        let trackable = trackables
            .find(trackable_id)
            .first::<Trackable>(&self.db.connection);

        match trackable {
            Ok(d) => {
                return Some(d);
            }
            Err(_) => return None,
        }
    }
    pub fn find_by_name(&self, trackable_name: &String) -> Option<Trackable> {
        use crate::schema::trackables::dsl::*;

        let trackable = trackables
            .filter(name.eq(trackable_name))
            .first::<Trackable>(&self.db.connection);

        match trackable {
            Ok(d) => {
                return Some(d);
            }
            Err(_) => return None,
        }
    }
}
