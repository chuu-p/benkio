use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;


// #[derive(Debug, Serialize, Deserialize, FromRow)]
// struct Flashcard {
//     id: Option<i64>,
//     content: String,
//     interval: i64,
//     next_review: NaiveDateTime,
// }

const PASS_FACTOR: f64 = 2.5;
const FAIL_FACTOR: f64 = 0.75;
const INITIAL_INTERVAL: i64 = 1;

impl Flashcard {
    fn new(content: String) -> Self {
        Flashcard {
            id: None,
            content,
            interval: INITIAL_INTERVAL,
            next_review: Utc::now().naive_utc() + Duration::days(INITIAL_INTERVAL),
        }
    }

    fn update(&mut self, passed: bool) {
        self.interval = {
            let factor = if passed { PASS_FACTOR } else { FAIL_FACTOR };
            (self.interval as f64 * factor).ceil() as i64
        };
        self.next_review = Utc::now().naive_utc() + Duration::days(self.interval);
    }
}

#[derive(Debug)]
struct Deck {
    cards: VecDeque<Flashcard>,
    learning_queue: VecDeque<Flashcard>,
    max_new_cards_per_day: usize,
}

impl Deck {
    fn new(max_new_cards_per_day: usize) -> Self {
        Deck {
            cards: VecDeque::new(),
            learning_queue: VecDeque::new(),
            max_new_cards_per_day,
        }
    }

    async fn load_from_db(pool: &SqlitePool) -> Self {
        // let cards: Vec<Flashcard> = sqlx::query_as!(
        //     Flashcard,
        //     "SELECT id, content, interval, next_review as next_review FROM flashcards"
        // )
        // .fetch_all(pool)
        // .await
        // .unwrap();

        // let mut deck = Deck::new(20);
        // for card in cards {
        //     deck.cards.push_back(card);
        // }
        // deck
        todo!()
    }

    async fn save_to_db(&self, pool: &SqlitePool) {
        // for card in &self.cards {
        //     if card.id.is_none() {
        //         sqlx::query!(
        //             "INSERT INTO flashcards (content, interval, next_review) VALUES (?, ?, ?)",
        //             card.content, card.interval, card.next_review
        //         )
        //         .execute(pool)
        //         .await
        //         .unwrap();
        //     } else {
        //         sqlx::query!(
        //             "UPDATE flashcards SET content = ?, interval = ?, next_review = ? WHERE id = ?",
        //             card.content, card.interval, card.next_review, card.id
        //         )
        //         .execute(pool)
        //         .await
        //         .unwrap();
        //     }
        // }
        todo!()
    }

    fn add_card(&mut self, content: String) {
        self.learning_queue.push_back(Flashcard::new(content));
    }

    fn learn_day(&mut self) {
        let mut new_cards_today = 0;

        for card in &mut self.cards {
            if card.next_review <= Utc::now().naive_utc() {
                let passed = rand::random::<bool>();
                card.update(passed);
                println!(
                    "Reviewed: {:?}, Passed: {}, Next Review: {:?}",
                    card.content, passed, card.next_review
                );
            }
        }

        while new_cards_today < self.max_new_cards_per_day && !self.learning_queue.is_empty() {
            let mut card = self.learning_queue.pop_front().unwrap();
            let passed = true;
            card.update(passed);
            self.cards.push_back(card);
            new_cards_today += 1;
        }
    }
}

#[cfg(test)]
pub mod tests {

    use log::info;

    use super::*;

    // #[tokio::test]
    async fn create_db() {
        let pool = SqlitePool::connect("sqlite://flashcards.db").await.unwrap();
        sqlx::query(include_str!("schema.sql"))
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn main() {
        let pool = SqlitePool::connect("sqlite://flashcards.db").await.unwrap();

        let mut deck = Deck::load_from_db(&pool).await;

        for i in 1..=100 {
            deck.add_card(format!("Card {}", i));
        }

        for day in 1..=10 {
            println!("Day {}: Learning session", day);
            deck.learn_day();
        }

        deck.save_to_db(&pool).await;

        println!("Final state of the deck: {:?}", deck);
    }
}
