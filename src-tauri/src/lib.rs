mod db;
mod flashcard;
pub mod models;
pub mod schema;

use crate::flashcard::MAX_NEW_CARDS_PER_DAY;
use chrono::{Duration, Utc};
use db::establish_connection;
use diesel::prelude::*;
use log::{info, LevelFilter};
use models::{Flashcard, NewCard};
use schema::flashcards::{self, interval};
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn get_flashcards_for_today() -> String {
    info!("TODO");

    let connection = &mut establish_connection();

    let results = flashcards::dsl::flashcards
        .select(Flashcard::as_select())
        .filter(flashcards::next_review.lt(Utc::now().naive_utc()))
        .limit(MAX_NEW_CARDS_PER_DAY)
        .order(flashcards::id)
        .load(connection)
        .expect("Error loading cards");

    println!("Displaying {} cards", results.len());
    for flashcard in &results {
        println!("{}", flashcard.content_front);
        println!("{}", flashcard.content_back);
        println!("-----------");
    }

    let json = serde_json::to_string_pretty(&results).unwrap();

    json
}

#[tauri::command]
fn update_flashcard(id: i32, passed: bool) -> Result<String, String> {
    let connection = &mut establish_connection();

    let mut card = flashcards::dsl::flashcards
        .filter(flashcards::id.eq(id))
        .get_result::<Flashcard>(connection)
        .expect("Error loading card");

    card.update(passed);

    diesel::update(flashcards::dsl::flashcards.find(id))
        .set((
            flashcards::interval.eq(card.interval),
            flashcards::next_review.eq(card.next_review),
        ))
        .execute(connection)
        .expect("Error updating card");

    // diesel::update(flashcards::dsl::flashcards.find(id))
    //     .set(interval.eq(card.interval))
    //     .execute(connection)
    //     .expect("Error updating card");

    // diesel::update(flashcards::dsl::flashcards.find(id))
    //     .set(flashcards::next_review.eq(card.next_review))
    //     .execute(connection)
    //     .expect("Error updating card");

    Ok("Success".to_string())
}

#[tauri::command]
fn debug_create_card() -> String {
    let connection = &mut establish_connection();

    let new_card = NewCard {
        content_front: "あ",
        content_back: "a",
        interval: 1,
        next_review: Utc::now().naive_utc() + Duration::days(1),
    };

    let card = diesel::insert_into(flashcards::table)
        .values(&new_card)
        .returning(Flashcard::as_returning())
        .get_result(connection)
        .expect("Error saving new flashcard");

    let json = serde_json::to_string_pretty(&card).unwrap();

    json
}

fn generate_json_schemas() -> Result<(), Box<dyn std::error::Error>> {
    use models::{Flashcard, Review};
    use schemars::schema_for;
    use std::fs;
    let schemas: Vec<(&str, schemars::schema::RootSchema)> = vec![
        ("flashcard", schema_for!(Flashcard)),
        ("review", schema_for!(Review)),
    ];

    let folder = "../shared/schemas";
    fs::create_dir_all(folder).unwrap();

    for (name, schema) in schemas {
        let file_path = format!("{}/{}.schema.json", folder, name);
        let json = serde_json::to_string_pretty(&schema)?;

        println!("Writing schema {} to {}", name, file_path);
        fs::write(file_path, json).expect("Unable to write file");
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    generate_json_schemas().unwrap();

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
        .invoke_handler(tauri::generate_handler![
            get_flashcards_for_today,
            update_flashcard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
