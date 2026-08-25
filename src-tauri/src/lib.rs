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
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed
                        && shortcut.matches(Modifiers::ALT, Code::KeyH)
                    {
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
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
            // Alt+H：全局隐藏/显示窗口（注册失败不阻断启动，仅告警）
            let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyH);
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("警告：Alt+H 全局快捷键注册失败（可能已被其它程序占用）：{e}");
            }
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
            commands::toggle_window_visible,
            commands::exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
