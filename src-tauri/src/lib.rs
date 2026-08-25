pub mod commands;
pub mod encodings;
pub mod error;
pub mod library;
pub mod models;
pub mod parsers;
pub mod search;
pub mod session;
pub mod storage;

use commands::AppState;
use library::Library;
use session::SessionManager;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let library = Library::open(&data_dir)?;
            let settings = library.load_settings()?;
            app.manage(AppState {
                library: std::sync::Mutex::new(library),
                sessions: std::sync::Mutex::new(SessionManager::new()),
                settings: std::sync::Mutex::new(settings),
                data_dir,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_library,
            commands::open_book,
            commands::import_book,
            commands::delete_book,
            commands::save_progress,
            commands::get_bookmarks,
            commands::add_bookmark,
            commands::delete_bookmark,
            commands::get_chapters,
            commands::get_chapter_text,
            commands::search_text,
            commands::get_settings,
            commands::save_settings,
            commands::set_fullscreen,
            commands::set_topmost,
            commands::set_opacity,
            commands::set_decorations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
