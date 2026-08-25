pub mod auto_hide;
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
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::str::FromStr;
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let Some(state) = app.try_state::<AppState>() else { return };
                    let settings = state.settings.lock().unwrap();
                    let wh = settings
                        .hotkeys
                        .get("toggleWindowVisible")
                        .cloned()
                        .unwrap_or_else(|| "Alt+H".into());
                    let ah = settings
                        .hotkeys
                        .get("toggleAutoHide")
                        .cloned()
                        .unwrap_or_else(|| "Ctrl+Alt+Shift+P".into());
                    drop(settings);
                    if let Ok(parsed) = Shortcut::from_str(&wh) {
                        if shortcut == &parsed {
                            if let Some(w) = app.get_webview_window("main") {
                                if w.is_visible().unwrap_or(false) {
                                    let _ = w.hide();
                                } else {
                                    let _ = w.show();
                                    let _ = w.set_focus();
                                }
                            }
                            return;
                        }
                    }
                    if let Ok(parsed) = Shortcut::from_str(&ah) {
                        if shortcut == &parsed {
                            let _ = commands::toggle_auto_hide_inner(&state);
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
            let auto_hide = Arc::new(AtomicBool::new(settings.auto_hide_on_leave));
            app.manage(AppState {
                library: std::sync::Mutex::new(library),
                sessions: std::sync::Mutex::new(SessionManager::new()),
                settings: std::sync::Mutex::new(settings),
                auto_hide: auto_hide.clone(),
                auto_hide_started: Arc::new(AtomicBool::new(false)),
                global_shortcuts: std::sync::Mutex::new(Vec::new()),
                data_dir,
            });
            // 全局快捷键跟随用户录制；自动隐藏线程懒启动
            commands::reload_global_hotkeys(app.handle());
            commands::ensure_auto_hide(app.handle(), &app.state::<AppState>());
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
            commands::toggle_auto_hide,
            commands::sync_global_hotkeys,
            commands::exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
