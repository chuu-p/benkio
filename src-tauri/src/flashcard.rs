use fsrs::{Card, FSRSItem};
use serde::{Deserialize, Serialize};

// fsrs: FSRS::new(Some(&fsrs::DEFAULT_PARAMETERS)).unwrap(),

#[derive(Serialize, Deserialize)]
#[serde(remote = "Card")]
pub struct CardDef {
    pub difficulty: f32,
    pub stability: f32,
    pub last_date: f32,
    pub due: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Flashcard {
    pub side_a: String,
    pub side_b: String,

    #[serde(with = "CardDef")]
    pub card: Card,
    pub fsrs_item: FSRSItem,
}

impl Default for Flashcard {
    fn default() -> Self {
        Self {
            side_a: String::new(),
            side_b: String::new(),

            card: Card {
                difficulty: 0f32,
                stability: 0f32,
                last_date: 0f32,
                due: 0f32,
            },
            fsrs_item: FSRSItem {
                reviews: Vec::new(),
            },
        }
    }
}
