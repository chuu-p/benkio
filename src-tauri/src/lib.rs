use flashcard::Flashcard;

mod db;
mod flashcard;

use lazy_static::lazy_static;
use log::LevelFilter;
use std::sync::Mutex;

use tauri_plugin_log::{Target, TargetKind};

lazy_static! {
    static ref INDEX: Mutex<usize> = Mutex::new(0);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_flashcards(amount: usize) -> String {
    let db = db::FlashcardDB::default();
    let res = db
        ._get_db()
        .unwrap()
        .iter()
        .map(|e| match e {
            Ok((k, _)) => std::str::from_utf8(&k).unwrap().to_string(),
            Err(e) => panic!("Error: {}", e),
        })
        .collect::<Vec<_>>();

    let mut index = INDEX.lock().unwrap();
    let json = res
        .iter()
        .map(|k| db.get::<Flashcard>(&k).unwrap())
        .map(|f| {
            format!(
                "{{ \"id\": {}, \"sideA\": \"{}\", \"sideB\": \"{}\" }}",
                f.id, f.side_a, f.side_b
            )
        })
        .skip(*index)
        .take(amount)
        .collect::<Vec<_>>()
        .join(",");
    let json = format!("[{}]", json);

    *index += amount;

    json
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
        .invoke_handler(tauri::generate_handler![get_flashcards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
