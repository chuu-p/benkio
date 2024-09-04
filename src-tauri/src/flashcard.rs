// use std::sync::atomic::{AtomicUsize, Ordering};

// use fsrs::{Card, FSRSItem};
// use serde::{Deserialize, Serialize};

// // fsrs: FSRS::new(Some(&fsrs::DEFAULT_PARAMETERS)).unwrap(),
// static COUNTER: AtomicUsize = AtomicUsize::new(1);
// fn get_id() -> usize {
//     COUNTER.fetch_add(1, Ordering::Relaxed)
// }

// #[derive(Serialize, Deserialize)]
// #[serde(remote = "Card")]
// pub struct CardDef {
//     pub difficulty: f32,
//     pub stability: f32,
//     pub last_date: f32,
//     pub due: f32,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Flashcard {
//     pub id: usize,
//     pub side_a: String,
//     pub side_b: String,

//     #[serde(with = "CardDef")]
//     pub card: Card,
//     pub fsrs_item: FSRSItem,
// }

// impl Default for Flashcard {
//     fn default() -> Self {
//         Self {
//             id: get_id(),
//             side_a: String::new(),
//             side_b: String::new(),

//             card: Card {
//                 difficulty: 0f32,
//                 stability: 0f32,
//                 last_date: 0f32,
//                 due: 0f32,
//             },
//             fsrs_item: FSRSItem {
//                 reviews: Vec::new(),
//             },
//         }
//     }
// }

// Import necessary libraries
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// Define a struct to represent a flashcard
#[derive(Debug, Serialize, Deserialize)]
struct Flashcard {
    // id: u64,
    content: String,
    // content_front: String,
    // content_back: String,
    ease_factor: f64,
    interval: i64,
    next_review: chrono::DateTime<Utc>,
}

// Define constants for initial settings
const INITIAL_EASE_FACTOR: f64 = 2.5;
const INITIAL_INTERVAL: i64 = 1;
const FORGETTING_INDEX: f64 = 0.2; // Target forgetting probability

impl Flashcard {
    // Create a new flashcard with initial values
    fn new(content: String) -> Self {
        Flashcard {
            content,
            ease_factor: INITIAL_EASE_FACTOR,
            interval: INITIAL_INTERVAL,
            next_review: Utc::now() + Duration::days(INITIAL_INTERVAL),
        }
    }

    // Update the flashcard based on recall performance
    fn update(&mut self, recall_quality: i32) {
        // Adjust the ease factor based on recall quality
        let new_ease_factor = match recall_quality {
            5 => self.ease_factor + 0.1,
            4 => self.ease_factor,
            3 => self.ease_factor - 0.15,
            2 => self.ease_factor - 0.3,
            1 => self.ease_factor - 0.5,
            _ => self.ease_factor - 1.0,
        };

        // Ensure the ease factor does not drop below a minimum threshold
        self.ease_factor = new_ease_factor.max(1.3);

        // Calculate the new interval based on the ease factor and previous interval
        if recall_quality >= 3 {
            self.interval = (self.interval as f64 * self.ease_factor).ceil() as i64;
        } else {
            self.interval = 1;
        }

        // Update the next review date
        self.next_review = Utc::now() + Duration::days(self.interval);
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    // use crate::flashcard::Flashcard;

    #[test]
    fn smoke_test_flashcard_update() -> Result<(), String> {
        // Create a new flashcard
        let mut flashcard = Flashcard::new("What is the capital of France?".to_string());

        // Simulate a review with different recall qualities
        println!("Initial Flashcard: {:?}", flashcard);

        // User recalls correctly with ease
        flashcard.update(5);
        println!("After Easy Recall: {:?}", flashcard);

        // User recalls with some difficulty
        flashcard.update(3); // Good
        println!("After Difficult Recall: {:?}", flashcard);

        // User fails to recall
        flashcard.update(1); // FAIL
        println!("After Failed Recall: {:?}", flashcard);

        Ok(())
    }
}
