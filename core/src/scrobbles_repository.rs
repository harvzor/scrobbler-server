use diesel::prelude::*;
use diesel::dsl::*;

use crate::models::scrobble::Scrobble;
use crate::db::Db;

pub struct ScrobblesRepository {
    pub db: Db,
}

impl ScrobblesRepository {
    pub fn new() -> ScrobblesRepository {
        ScrobblesRepository {
            db: Db::new(),
        }
    }
    pub fn get(&self, skip: i64, take: i64, from: Option<chrono::NaiveDateTime>, to: Option<chrono::NaiveDateTime>) -> Vec<Scrobble> {
        use crate::schema::scrobbles::dsl::*;

        let mut query = scrobbles
            .into_boxed();

        match from {
            Some(x) => {
                query = query
                    .filter(timestamp.gt(x));
            },
            None => {}
        }

        match to {
            Some(x) => {
                query = query
                    .filter(timestamp.lt(x));
            },
            None => {}
        }

        let results = query
            .order(timestamp.desc())
            .offset(skip)
            .limit(take)
            .load::<Scrobble>(&self.db.connection)
            .expect("Error loading scrobbles");

        results
    }
    pub fn create(&self, trackable_id: i32) -> Scrobble {
        use crate::schema::scrobbles;
        use crate::models::scrobble::NewScrobble;

        let new_scrobble = NewScrobble {
            trackable_id: trackable_id,
            timestamp: chrono::Utc::now().naive_local(),
        };

        diesel::insert_into(scrobbles::table)
            .values(&new_scrobble)
            .get_result(&self.db.connection)
            .expect("Error saving new scrobble")
    }
    pub fn get_for_trackable(&self, scrobble_trackable_id: i32) -> Vec<Scrobble> {
        use crate::schema::scrobbles::dsl::*;

        scrobbles
            .filter(trackable_id.eq(scrobble_trackable_id))
            .load::<Scrobble>(&self.db.connection)
            .expect("Error loading scrobbles")
    }
    pub fn get_for_trackables(&self, trackable_ids: Vec<i32>) -> Vec<Scrobble> {
        use crate::schema::scrobbles::dsl::*;

        scrobbles
            .filter(trackable_id.eq(any(trackable_ids)))
            .load::<Scrobble>(&self.db.connection)
            .expect("Error loading scrobbles")
    }
}
