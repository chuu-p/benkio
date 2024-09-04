mod db;
pub mod models;
pub mod schema;

use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
use log::{info, LevelFilter};
use models::{Flashcard, NewCard};
use schema::flashcards;
use std::collections::VecDeque;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn get_flashcards_for_today() -> String {
    info!("TODO");

    let json = format!(
        "[{{ \"id\": {}, \"sideA\": \"{}\", \"sideB\": \"{}\" }}]",
        1, "A", "B"
    );
    json
}

pub fn create_card(
    conn: &mut SqliteConnection,
    content_front: &str,
    content_back: &str,
) -> Flashcard {
    use crate::schema::flashcards;

    let new_card = NewCard {
        content_front: content_front,
        content_back: content_back,
        interval: 1,
        next_review: Utc::now().naive_utc() + Duration::days(1),
    };

    diesel::insert_into(flashcards::table)
        .values(&new_card)
        .returning(Flashcard::as_returning())
        .get_result(conn)
        .expect("Error saving new flashcard")
}

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

#[derive(Debug)]
struct Deck {
    cards: VecDeque<Flashcard>,
    learning_queue: VecDeque<Flashcard>,
}

impl Deck {
    fn load_from_db(conn: &SqliteConnection) -> Self {
        // let results = flashcards
        //     .limit(5)
        //     .select(Flashcard::as_select())
        //     .load(conn)
        //     .expect("Error loading posts");

        // println!("Displaying {} posts", results.len());
        // for flashcard in results {
        //     println!("{}", flashcard.content_front);
        //     println!("{}", flashcard.content_back);
        //     println!("-----------");
        // }

        Deck {
            cards: VecDeque::new(),
            learning_queue: VecDeque::new(),
        };

        todo!()
    }

    async fn save_to_db(&self, conn: &SqliteConnection) {
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

    // fn learn_day(&mut self) {
    //     let mut new_cards_today = 0;

    //     for card in &mut self.cards {
    //         if card.next_review <= Utc::now().naive_utc() {
    //             let passed = rand::random::<bool>();
    //             card.update(passed);
    //             println!(
    //                 "Reviewed: {:?}, Passed: {}, Next Review: {:?}",
    //                 card.content_front, passed, card.next_review
    //             );
    //         }
    //     }

    //     while new_cards_today < MAX_NEW_CARDS_PER_DAY && !self.learning_queue.is_empty() {
    //         let mut card = self.learning_queue.pop_front().unwrap();
    //         let passed = true;
    //         card.update(passed);
    //         self.cards.push_back(card);
    //         new_cards_today += 1;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use db::establish_connection;

    use self::schema::flashcards::dsl::*;
    use super::*;

    #[test]
    fn it_create_card() {
        let connection = &mut establish_connection();

        let contents = vec![
            ("A", "B"),
            ("C", "D"),
            ("E", "F"),
            ("G", "H"),
            ("I", "J"),
            ("K", "L"),
            ("M", "N"),
            ("O", "P"),
            ("Q", "R"),
            ("S", "T"),
            ("U", "V"),
            ("W", "X"),
            ("Y", "Z"),
        ];

        for (front, back) in contents {
            create_card(connection, front, back);
        }
    }

    #[test]
    fn it_works() {
        let connection = &mut establish_connection();

        let results = flashcards
            .limit(5)
            .select(Flashcard::as_select())
            .load(connection)
            .expect("Error loading cards");

        println!("Displaying {} cards", results.len());
        for flashcard in results {
            println!("{}", flashcard.content_front);
            println!("{}", flashcard.content_back);
            println!("-----------");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    // Target::new(TargetKind::Webview),
                ])
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_flashcards_for_today])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
