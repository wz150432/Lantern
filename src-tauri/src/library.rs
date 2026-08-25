use crate::error::{AppError, AppResult};
use crate::models::{AppSettings, BookRecord, Bookmark};
use crate::storage::Storage;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportMode {
    Linked,
    Copied,
}

pub struct Library {
    storage: Storage,
    books_dir: std::path::PathBuf,
}

impl Library {
    pub fn open(data_dir: &Path) -> AppResult<Library> {
        let books_dir = data_dir.join("books");
        std::fs::create_dir_all(&books_dir)?;
        Ok(Library {
            storage: Storage::open(data_dir)?,
            books_dir,
        })
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    pub fn import_book(&self, source: &Path, mode: ImportMode) -> AppResult<BookRecord> {
        if !source.is_file() {
            return Err(AppError::Invalid("文件不存在".into()));
        }
        let file_path = match mode {
            ImportMode::Linked => source.to_path_buf(),
            ImportMode::Copied => {
                let fname = source
                    .file_name()
                    .ok_or_else(|| AppError::Invalid("无效文件名".into()))?;
                let dest =
                    self.books_dir
                        .join(format!("{}-{}", Self::now(), fname.to_string_lossy()));
                std::fs::copy(source, &dest)?;
                dest
            }
        };
        let now = Self::now();
        let rec = BookRecord {
            id: 0,
            title: source
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("未命名")
                .to_string(),
            author: None,
            format: "txt".into(),
            file_path: file_path.to_string_lossy().into_owned(),
            storage_mode: if mode == ImportMode::Linked {
                "linked"
            } else {
                "copied"
            }
            .into(),
            cover_path: None,
            added_at: now,
            last_opened_at: now,
            total_chapters: 0,
            current_chapter: 0,
            progress: 0.0,
        };
        let id = self.storage.upsert_book(&rec)?;
        Ok(BookRecord { id, ..rec })
    }

    pub fn open_into_recent(&self, source: &Path) -> AppResult<BookRecord> {
        let abs = std::fs::canonicalize(source)?;
        let existing = self.list()?.into_iter().find(|b| {
            Path::new(&b.file_path)
                .canonicalize()
                .map(|p| p == abs)
                .unwrap_or(false)
        });
        match existing {
            Some(mut rec) => {
                self.storage.update_progress(
                    rec.id,
                    rec.current_chapter,
                    rec.progress,
                    Self::now(),
                )?;
                rec.last_opened_at = Self::now();
                Ok(rec)
            }
            None => self.import_book(source, ImportMode::Linked),
        }
    }

    pub fn list(&self) -> AppResult<Vec<BookRecord>> {
        self.storage.list_books()
    }

    pub fn remove(&self, id: i64, delete_copy: bool) -> AppResult<()> {
        let rec = self.storage.get_book(id)?;
        if let Some(r) = rec {
            if delete_copy && r.storage_mode == "copied" {
                let p = Path::new(&r.file_path);
                if p.exists() {
                    std::fs::remove_file(p)?;
                }
            }
        }
        self.storage.delete_book(id)
    }

    pub fn update_progress(&self, id: i64, chapter: usize, progress: f64) -> AppResult<()> {
        self.storage
            .update_progress(id, chapter, progress, Self::now())
    }

    pub fn add_bookmark(
        &self,
        book_id: i64,
        chapter: usize,
        position: f64,
        note: Option<String>,
    ) -> AppResult<i64> {
        let bm = Bookmark {
            id: 0,
            book_id,
            chapter_index: chapter,
            position,
            note,
            created_at: Self::now(),
        };
        self.storage.add_bookmark(&bm)
    }

    pub fn list_bookmarks(&self, book_id: i64) -> AppResult<Vec<Bookmark>> {
        self.storage.list_bookmarks(book_id)
    }
    pub fn delete_bookmark(&self, id: i64) -> AppResult<()> {
        self.storage.delete_bookmark(id)
    }
    pub fn load_settings(&self) -> AppResult<AppSettings> {
        self.storage.load_settings()
    }

    pub fn save_settings(&self, settings: &AppSettings) -> AppResult<()> {
        self.storage.save_settings(settings)
    }
    pub fn books_dir(&self) -> &Path {
        &self.books_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn write_sample_txt(dir: &Path) -> std::path::PathBuf {
        let p = dir.join("sample.txt");
        let mut f = std::fs::File::create(&p).unwrap();
        f.write_all("第一章 开始\n内容。\n第二章 结束\n内容。\n".as_bytes())
            .unwrap();
        p
    }

    #[test]
    fn linked_import_keeps_source() {
        let tmp = TempDir::new().unwrap();
        let src = write_sample_txt(tmp.path());
        let lib = Library::open(tmp.path()).unwrap();
        let rec = lib.import_book(&src, ImportMode::Linked).unwrap();
        assert_eq!(rec.storage_mode, "linked");
        assert_eq!(rec.file_path, src.to_str().unwrap());
        assert!(src.exists());
        assert_eq!(lib.list().unwrap().len(), 1);
    }

    #[test]
    fn copied_import_copies_file() {
        let tmp = TempDir::new().unwrap();
        let src = write_sample_txt(tmp.path());
        let lib = Library::open(tmp.path()).unwrap();
        let rec = lib.import_book(&src, ImportMode::Copied).unwrap();
        assert_eq!(rec.storage_mode, "copied");
        let copied = Path::new(&rec.file_path);
        assert_ne!(copied, src.as_path());
        assert!(copied.exists());
        assert_eq!(std::fs::read(copied).unwrap(), std::fs::read(&src).unwrap());
    }

    #[test]
    fn open_into_recent_dedupes() {
        let tmp = TempDir::new().unwrap();
        let src = write_sample_txt(tmp.path());
        let lib = Library::open(tmp.path()).unwrap();
        let a = lib.open_into_recent(&src).unwrap();
        let b = lib.open_into_recent(&src).unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(lib.list().unwrap().len(), 1);
    }

    #[test]
    fn remove_linked_keeps_file() {
        let tmp = TempDir::new().unwrap();
        let src = write_sample_txt(tmp.path());
        let lib = Library::open(tmp.path()).unwrap();
        let rec = lib.import_book(&src, ImportMode::Linked).unwrap();
        lib.remove(rec.id, false).unwrap();
        assert!(src.exists());
        assert_eq!(lib.list().unwrap().len(), 0);
    }

    #[test]
    fn remove_copied_with_delete_copy_removes_file() {
        let tmp = TempDir::new().unwrap();
        let src = write_sample_txt(tmp.path());
        let lib = Library::open(tmp.path()).unwrap();
        let rec = lib.import_book(&src, ImportMode::Copied).unwrap();
        let copied = Path::new(&rec.file_path).to_path_buf();
        lib.remove(rec.id, true).unwrap();
        assert!(!copied.exists());
        assert_eq!(lib.list().unwrap().len(), 0);
    }
}
