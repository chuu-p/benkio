use chrono::{Duration, Utc};
use diesel::{RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::models::{Flashcard, NewCard};

const PASS_FACTOR: f32 = 2.5;
const FAIL_FACTOR: f32 = 0.75;
const INITIAL_INTERVAL: i32 = 1;
const MAX_NEW_CARDS_PER_DAY: usize = 20;

impl Flashcard {
    fn update(&mut self, passed: bool) {
        self.interval = {
            let factor = if passed { PASS_FACTOR } else { FAIL_FACTOR };
            (self.interval as f32 * factor).ceil() as i32
        };
        self.next_review = Utc::now().naive_utc() + Duration::days(self.interval as i64);
    }
}
