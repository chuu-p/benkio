mod db;
// mod flashcard;
pub mod models;
pub mod schema;

use chrono::{Duration, Utc};
use diesel::prelude::*;
use log::{info, LevelFilter};

use models::{Flashcard, NewCard};
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

pub fn create_card(conn: &mut SqliteConnection, content_front: &str, content_back: &str) -> Flashcard{
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
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
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
