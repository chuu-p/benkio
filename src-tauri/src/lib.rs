mod db;
mod flashcard;
pub mod models;
pub mod schema;

use chrono::{Duration, Utc};
use db::establish_connection;
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

    let connection = &mut establish_connection();

    let results = flashcards::dsl::flashcards
        .limit(5)
        .select(Flashcard::as_select())
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
            debug_create_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
