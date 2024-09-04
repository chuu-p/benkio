mod flashcard;

use log::{info, LevelFilter};

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
