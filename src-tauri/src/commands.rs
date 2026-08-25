use crate::error::{AppError, AppResult};
use crate::library::{ImportMode, Library};
use crate::models::{AppSettings, BookRecord, Bookmark, ChapterInfo};
use crate::search::SearchHit;
use crate::session::SessionManager;
use std::path::Path;
use std::sync::Mutex;
use tauri::Window;

pub struct AppState {
    pub library: Mutex<Library>,
    pub sessions: Mutex<SessionManager>,
    pub settings: Mutex<AppSettings>,
    pub data_dir: std::path::PathBuf,
}

fn lock_error<T>(_: std::sync::PoisonError<T>) -> AppError {
    AppError::Storage("锁错误".into())
}

#[tauri::command]
pub fn list_library(state: tauri::State<'_, AppState>) -> AppResult<Vec<BookRecord>> {
    state.library.lock().map_err(lock_error)?.list()
}

#[tauri::command]
pub fn open_book(state: tauri::State<'_, AppState>, path: String) -> AppResult<BookRecord> {
    let mut rec = {
        let lib = state.library.lock().map_err(lock_error)?;
        lib.open_into_recent(Path::new(&path))?
    };
    // 建会话（顺便拿到章节数回写记录）；先释放书库锁再碰会话锁，避免锁顺序问题
    let chapters = {
        let mut sessions = state.sessions.lock().map_err(lock_error)?;
        sessions.open(rec.id, Path::new(&rec.file_path), None)?;
        sessions.chapters(rec.id)?
    };
    rec.total_chapters = chapters.len();
    state.library.lock().map_err(lock_error)?.update_progress(
        rec.id,
        rec.current_chapter,
        rec.progress,
    )?;
    Ok(rec)
}

#[tauri::command]
pub fn import_book(
    state: tauri::State<'_, AppState>,
    path: String,
    mode: String,
) -> AppResult<BookRecord> {
    let m = if mode == "copied" {
        ImportMode::Copied
    } else {
        ImportMode::Linked
    };
    state
        .library
        .lock()
        .map_err(lock_error)?
        .import_book(Path::new(&path), m)
}

#[tauri::command]
pub fn delete_book(state: tauri::State<'_, AppState>, id: i64, delete_copy: bool) -> AppResult<()> {
    state.sessions.lock().map_err(lock_error)?.close(id);
    state
        .library
        .lock()
        .map_err(lock_error)?
        .remove(id, delete_copy)
}

#[tauri::command]
pub fn save_progress(
    state: tauri::State<'_, AppState>,
    id: i64,
    chapter: usize,
    progress: f64,
) -> AppResult<()> {
    state
        .library
        .lock()
        .map_err(lock_error)?
        .update_progress(id, chapter, progress)
}

#[tauri::command]
pub fn get_bookmarks(state: tauri::State<'_, AppState>, book_id: i64) -> AppResult<Vec<Bookmark>> {
    state
        .library
        .lock()
        .map_err(lock_error)?
        .list_bookmarks(book_id)
}

#[tauri::command]
pub fn add_bookmark(
    state: tauri::State<'_, AppState>,
    book_id: i64,
    chapter: usize,
    position: f64,
    note: Option<String>,
) -> AppResult<i64> {
    state
        .library
        .lock()
        .map_err(lock_error)?
        .add_bookmark(book_id, chapter, position, note)
}

#[tauri::command]
pub fn delete_bookmark(state: tauri::State<'_, AppState>, id: i64) -> AppResult<()> {
    state
        .library
        .lock()
        .map_err(lock_error)?
        .delete_bookmark(id)
}

#[tauri::command]
pub fn get_chapters(
    state: tauri::State<'_, AppState>,
    book_id: i64,
    path: String,
) -> AppResult<Vec<ChapterInfo>> {
    let mut sessions = state.sessions.lock().map_err(lock_error)?;
    sessions.open(book_id, Path::new(&path), None)?;
    sessions.chapters(book_id)
}

#[tauri::command]
pub fn get_chapter_text(
    state: tauri::State<'_, AppState>,
    book_id: i64,
    index: usize,
) -> AppResult<String> {
    state
        .sessions
        .lock()
        .map_err(lock_error)?
        .chapter_text(book_id, index)
}

#[tauri::command]
pub fn search_text(
    state: tauri::State<'_, AppState>,
    book_id: i64,
    query: String,
    limit: usize,
) -> AppResult<Vec<SearchHit>> {
    let sessions = state.sessions.lock().map_err(lock_error)?;
    let book = sessions
        .book(book_id)
        .ok_or_else(|| AppError::NotFound("会话不存在".into()))?;
    crate::search::search_book(book, &query, limit)
}

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> AppSettings {
    state.settings.lock().map(|s| s.clone()).unwrap_or_default()
}

#[tauri::command]
pub fn save_settings(state: tauri::State<'_, AppState>, settings: AppSettings) -> AppResult<()> {
    let mut current = state.settings.lock().map_err(lock_error)?;
    *current = settings.clone();
    drop(current);
    state
        .library
        .lock()
        .map_err(lock_error)?
        .save_settings(&settings)
}

#[tauri::command]
pub fn set_fullscreen(window: Window, full: bool) -> AppResult<()> {
    window
        .set_fullscreen(full)
        .map_err(|e| AppError::Invalid(e.to_string()))
}

#[tauri::command]
pub fn set_topmost(window: Window, top: bool) -> AppResult<()> {
    window
        .set_always_on_top(top)
        .map_err(|e| AppError::Invalid(e.to_string()))
}

// tauri 2.11 未提供 `Window::set_opacity`，这里把透明度写回设置持久化。
#[tauri::command]
pub fn set_opacity(state: tauri::State<'_, AppState>, opacity: f64) -> AppResult<()> {
    let mut settings = state.settings.lock().map_err(lock_error)?;
    settings.window_opacity = opacity;
    let snapshot = settings.clone();
    drop(settings);
    state
        .library
        .lock()
        .map_err(lock_error)?
        .save_settings(&snapshot)
}

#[tauri::command]
pub fn set_decorations(window: Window, decorated: bool) -> AppResult<()> {
    window
        .set_decorations(decorated)
        .map_err(|e| AppError::Invalid(e.to_string()))
}
