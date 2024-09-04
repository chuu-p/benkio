use chrono::{Duration, Utc};

use crate::models::Flashcard;

const PASS_FACTOR: f32 = 2.5;
const FAIL_FACTOR: f32 = 0.75;
const INITIAL_INTERVAL: i32 = 1;
pub const MAX_NEW_CARDS_PER_DAY: i64 = 5;

// TODO: interval should be 1 min, 10 min, 1 day, 3 days, etc.
// TODO: Add entry to reviews table
impl Flashcard {
    pub fn update(&mut self, passed: bool) {
        self.interval = {
            let factor = if passed { PASS_FACTOR } else { FAIL_FACTOR };
            (self.interval as f32 * factor).ceil() as i32
        };
        self.next_review = Utc::now().naive_utc() + Duration::days(self.interval as i64);
    }
}
