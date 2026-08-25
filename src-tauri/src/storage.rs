use crate::error::{AppError, AppResult};
use crate::models::{AppSettings, BookRecord, Bookmark};
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Storage {
    db: Connection,
    settings_path: std::path::PathBuf,
}

impl Storage {
    pub fn open(data_dir: &Path) -> AppResult<Storage> {
        std::fs::create_dir_all(data_dir)?;
        let db_path = data_dir.join("library.db");
        let db = Connection::open(&db_path)?;
        db.pragma_update(None, "journal_mode", "WAL")?;
        db.pragma_update(None, "foreign_keys", "ON")?;
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                author TEXT,
                format TEXT NOT NULL,
                file_path TEXT NOT NULL,
                storage_mode TEXT NOT NULL,
                cover_path TEXT,
                added_at INTEGER NOT NULL,
                last_opened_at INTEGER NOT NULL,
                total_chapters INTEGER NOT NULL DEFAULT 0,
                current_chapter INTEGER NOT NULL DEFAULT 0,
                progress REAL NOT NULL DEFAULT 0.0
            );
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id INTEGER NOT NULL,
                chapter_index INTEGER NOT NULL,
                position REAL NOT NULL,
                note TEXT,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
            );",
        )?;
        Ok(Storage {
            db,
            settings_path: data_dir.join("settings.json"),
        })
    }

    pub fn upsert_book(&self, b: &BookRecord) -> AppResult<i64> {
        if b.id == 0 {
            self.db.execute(
                "INSERT INTO books (title, author, format, file_path, storage_mode, cover_path, added_at, last_opened_at, total_chapters, current_chapter, progress)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                params![b.title, b.author, b.format, b.file_path, b.storage_mode, b.cover_path, b.added_at, b.last_opened_at, b.total_chapters as i64, b.current_chapter as i64, b.progress],
            )?;
            Ok(self.db.last_insert_rowid())
        } else {
            self.db.execute(
                "UPDATE books SET title=?1, author=?2, format=?3, file_path=?4, storage_mode=?5, cover_path=?6, added_at=?7, last_opened_at=?8, total_chapters=?9, current_chapter=?10, progress=?11 WHERE id=?12",
                params![b.title, b.author, b.format, b.file_path, b.storage_mode, b.cover_path, b.added_at, b.last_opened_at, b.total_chapters as i64, b.current_chapter as i64, b.progress, b.id],
            )?;
            Ok(b.id)
        }
    }

    pub fn list_books(&self) -> AppResult<Vec<BookRecord>> {
        let mut stmt = self.db.prepare("SELECT id,title,author,format,file_path,storage_mode,cover_path,added_at,last_opened_at,total_chapters,current_chapter,progress FROM books ORDER BY last_opened_at DESC")?;
        let rows = stmt.query_map([], row_to_book)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn get_book(&self, id: i64) -> AppResult<Option<BookRecord>> {
        let mut stmt = self.db.prepare("SELECT id,title,author,format,file_path,storage_mode,cover_path,added_at,last_opened_at,total_chapters,current_chapter,progress FROM books WHERE id=?1")?;
        let mut rows = stmt.query_map([id], row_to_book)?;
        match rows.next() {
            Some(Ok(b)) => Ok(Some(b)),
            Some(Err(e)) => Err(AppError::Storage(e.to_string())),
            None => Ok(None),
        }
    }

    pub fn delete_book(&self, id: i64) -> AppResult<()> {
        self.db.execute("DELETE FROM books WHERE id=?1", [id])?;
        Ok(())
    }

    pub fn update_progress(
        &self,
        id: i64,
        chapter: usize,
        progress: f64,
        last_opened_at: i64,
    ) -> AppResult<()> {
        self.db.execute(
            "UPDATE books SET current_chapter=?1, progress=?2, last_opened_at=?3 WHERE id=?4",
            params![chapter as i64, progress, last_opened_at, id],
        )?;
        Ok(())
    }

    pub fn add_bookmark(&self, b: &Bookmark) -> AppResult<i64> {
        self.db.execute("INSERT INTO bookmarks (book_id, chapter_index, position, note, created_at) VALUES (?1,?2,?3,?4,?5)",
            params![b.book_id, b.chapter_index as i64, b.position, b.note, b.created_at])?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn list_bookmarks(&self, book_id: i64) -> AppResult<Vec<Bookmark>> {
        let mut stmt = self.db.prepare("SELECT id,book_id,chapter_index,position,note,created_at FROM bookmarks WHERE book_id=?1 ORDER BY created_at")?;
        let rows = stmt.query_map([book_id], |r| {
            Ok(Bookmark {
                id: r.get(0)?,
                book_id: r.get(1)?,
                chapter_index: r.get::<_, i64>(2)? as usize,
                position: r.get(3)?,
                note: r.get(4)?,
                created_at: r.get(5)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn delete_bookmark(&self, id: i64) -> AppResult<()> {
        self.db.execute("DELETE FROM bookmarks WHERE id=?1", [id])?;
        Ok(())
    }

    pub fn load_settings(&self) -> AppResult<AppSettings> {
        if !self.settings_path.exists() {
            return Ok(AppSettings::default());
        }
        let raw = std::fs::read_to_string(&self.settings_path)?;
        let parsed: serde_json::Value = serde_json::from_str(&raw)?;
        let defaults = AppSettings::default();
        let default_font_family = defaults.font_family.clone();
        let mut merged = defaults;
        if let Some(v) = parsed.get("theme") {
            merged.theme = v.as_str().unwrap_or("minimal").into();
        }
        if let Some(v) = parsed.get("fontFamily") {
            merged.font_family = v.as_str().unwrap_or(default_font_family.as_str()).into();
        }
        if let Some(v) = parsed.get("fontSize") {
            merged.font_size = v.as_f64().unwrap_or(18.0);
        }
        if let Some(v) = parsed.get("lineHeight") {
            merged.line_height = v.as_f64().unwrap_or(1.8);
        }
        if let Some(v) = parsed.get("paragraphSpacing") {
            merged.paragraph_spacing = v.as_f64().unwrap_or(8.0);
        }
        if let Some(v) = parsed.get("firstLineIndent") {
            merged.first_line_indent = v.as_bool().unwrap_or(true);
        }
        if let Some(v) = parsed.get("charSpacing") {
            merged.char_spacing = v.as_f64().unwrap_or(0.5);
        }
        if let Some(v) = parsed.get("compressBlankLines") {
            merged.compress_blank_lines = v.as_bool().unwrap_or(true);
        }
        if let Some(v) = parsed.get("wordWrap") {
            merged.word_wrap = v.as_bool().unwrap_or(true);
        }
        if let Some(v) = parsed.get("innerPadding") {
            merged.inner_padding = v.as_f64().unwrap_or(48.0);
        }
        if let Some(v) = parsed.get("scrollSpeed") {
            merged.scroll_speed = v.as_u64().unwrap_or(1) as usize;
        }
        if let Some(v) = parsed.get("pageMode") {
            merged.page_mode = v.as_str().unwrap_or("page").into();
        }
        if let Some(v) = parsed.get("autoPageIntervalMs") {
            merged.auto_page_interval_ms = v.as_u64().unwrap_or(3000);
        }
        if let Some(v) = parsed.get("clickMode") {
            merged.click_mode = v.as_str().unwrap_or("thirds").into();
        }
        if let Some(v) = parsed.get("pageDouble") {
            merged.page_double = v.as_bool().unwrap_or(false);
        }
        if let Some(v) = parsed.get("windowTopmost") {
            merged.window_topmost = v.as_bool().unwrap_or(false);
        }
        if let Some(v) = parsed.get("windowOpacity") {
            merged.window_opacity = v.as_f64().unwrap_or(1.0);
        }
        if let Some(v) = parsed.get("immersiveMode") {
            merged.immersive_mode = v.as_bool().unwrap_or(false);
        }
        Ok(merged)
    }

    pub fn save_settings(&self, s: &AppSettings) -> AppResult<()> {
        let json = serde_json::to_string_pretty(s)?;
        std::fs::write(&self.settings_path, json)?;
        Ok(())
    }
}

fn row_to_book(r: &rusqlite::Row) -> rusqlite::Result<BookRecord> {
    Ok(BookRecord {
        id: r.get(0)?,
        title: r.get(1)?,
        author: r.get(2)?,
        format: r.get(3)?,
        file_path: r.get(4)?,
        storage_mode: r.get(5)?,
        cover_path: r.get(6)?,
        added_at: r.get(7)?,
        last_opened_at: r.get(8)?,
        total_chapters: r.get::<_, i64>(9)? as usize,
        current_chapter: r.get::<_, i64>(10)? as usize,
        progress: r.get(11)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_book(id: i64) -> BookRecord {
        BookRecord {
            id,
            title: "测试书".into(),
            author: Some("作者".into()),
            format: "txt".into(),
            file_path: "C:/books/a.txt".into(),
            storage_mode: "linked".into(),
            cover_path: None,
            added_at: 1000,
            last_opened_at: 1000,
            total_chapters: 3,
            current_chapter: 0,
            progress: 0.0,
        }
    }

    fn open_storage() -> (TempDir, Storage) {
        let dir = TempDir::new().unwrap();
        let s = Storage::open(dir.path()).unwrap();
        (dir, s)
    }

    #[test]
    fn upsert_and_get_book() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        let got = s.get_book(id).unwrap().unwrap();
        assert_eq!(got.title, "测试书");
        assert_eq!(got.storage_mode, "linked");
    }

    #[test]
    fn update_progress_persists() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        s.update_progress(id, 2, 0.5, 2000).unwrap();
        let got = s.get_book(id).unwrap().unwrap();
        assert_eq!(got.current_chapter, 2);
        assert!((got.progress - 0.5).abs() < 1e-6);
    }

    #[test]
    fn delete_book_removes_row() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        s.delete_book(id).unwrap();
        assert!(s.get_book(id).unwrap().is_none());
    }

    #[test]
    fn bookmarks_crud() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        let bm = Bookmark {
            id: 0,
            book_id: id,
            chapter_index: 1,
            position: 0.3,
            note: Some("名场面".into()),
            created_at: 1000,
        };
        let bm_id = s.add_bookmark(&bm).unwrap();
        let list = s.list_bookmarks(id).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].note.as_deref(), Some("名场面"));
        s.delete_bookmark(bm_id).unwrap();
        assert_eq!(s.list_bookmarks(id).unwrap().len(), 0);
    }

    #[test]
    fn delete_book_cascades_to_bookmarks() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        let bm = Bookmark {
            id: 0,
            book_id: id,
            chapter_index: 0,
            position: 0.0,
            note: None,
            created_at: 1000,
        };
        s.add_bookmark(&bm).unwrap();
        assert_eq!(s.list_bookmarks(id).unwrap().len(), 1);
        s.delete_book(id).unwrap();
        assert!(s.list_bookmarks(id).unwrap().is_empty());
    }

    #[test]
    fn upsert_existing_book_updates_in_place() {
        let (_d, s) = open_storage();
        let id = s.upsert_book(&sample_book(0)).unwrap();
        let mut updated = sample_book(id);
        updated.title = "更新标题".into();
        updated.author = Some("新作者".into());
        updated.progress = 0.75;
        let returned = s.upsert_book(&updated).unwrap();
        assert_eq!(returned, id);
        let got = s.get_book(id).unwrap().unwrap();
        assert_eq!(got.title, "更新标题");
        assert_eq!(got.author.as_deref(), Some("新作者"));
        assert!((got.progress - 0.75).abs() < 1e-6);
        assert_eq!(s.list_books().unwrap().len(), 1);
    }

    #[test]
    fn settings_roundtrip() {
        let (_d, s) = open_storage();
        let cfg = AppSettings {
            font_size: 22.0,
            page_mode: "scroll".into(),
            ..AppSettings::default()
        };
        s.save_settings(&cfg).unwrap();
        let loaded = s.load_settings().unwrap();
        assert_eq!(loaded.font_size, 22.0);
        assert_eq!(loaded.page_mode, "scroll");
    }

    #[test]
    fn settings_default_when_missing() {
        let (_d, s) = open_storage();
        let loaded = s.load_settings().unwrap();
        assert_eq!(loaded.theme, "minimal");
    }
}
