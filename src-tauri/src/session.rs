use crate::error::{AppError, AppResult};
use crate::models::{BookMeta, ChapterInfo};
use crate::parsers::txt::TxtBook;
use crate::parsers::BookFormat;
use std::collections::HashMap;
use std::path::Path;

struct SessionEntry {
    book: Box<dyn BookFormat>,
    mtime: std::time::SystemTime,
    path: std::path::PathBuf,
}

pub struct SessionManager {
    entries: HashMap<i64, SessionEntry>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
impl SessionManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn open(&mut self, book_id: i64, path: &Path, custom_regex: Option<&str>) -> AppResult<()> {
        // 已打开且文件未变则复用
        if let Some(e) = self.entries.get(&book_id) {
            let mtime = std::fs::metadata(&e.path)?.modified()?;
            if mtime == e.mtime {
                return Ok(());
            }
        }
        let book = TxtBook::open(path, custom_regex)?;
        let mtime = std::fs::metadata(path)?.modified()?;
        self.entries.insert(
            book_id,
            SessionEntry {
                book: Box::new(book),
                mtime,
                path: path.to_path_buf(),
            },
        );
        Ok(())
    }

    pub fn chapters(&mut self, book_id: i64) -> AppResult<Vec<ChapterInfo>> {
        self.entries
            .get(&book_id)
            .map(|e| e.book.chapter_list())
            .ok_or_else(|| AppError::NotFound("会话不存在".into()))
    }

    pub fn chapter_text(&mut self, book_id: i64, index: usize) -> AppResult<String> {
        self.entries
            .get(&book_id)
            .ok_or_else(|| AppError::NotFound("会话不存在".into()))?
            .book
            .chapter_text(index)
    }

    pub fn meta(&mut self, book_id: i64) -> AppResult<BookMeta> {
        self.entries
            .get(&book_id)
            .map(|e| e.book.meta())
            .ok_or_else(|| AppError::NotFound("会话不存在".into()))
    }

    pub fn close(&mut self, book_id: i64) {
        self.entries.remove(&book_id);
    }

    /// 供命令层复用的只读访问器（例如搜索时交给 `search_book`）。
    pub fn book(&self, book_id: i64) -> Option<&dyn BookFormat> {
        self.entries.get(&book_id).map(|e| e.book.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn open_and_read_chapter() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all("第一章 甲\n内容。\n第二章 乙\n内容。\n".as_bytes())
            .unwrap();
        let mut sm = SessionManager::new();
        sm.open(1, f.path(), None).unwrap();
        let chapters = sm.chapters(1).unwrap();
        assert_eq!(chapters.len(), 2);
        assert!(sm.chapter_text(1, 1).unwrap().contains("内容。"));
    }

    #[test]
    fn reuses_session_when_unchanged() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all("第一章 甲\n内容。\n".as_bytes()).unwrap();
        let mut sm = SessionManager::new();
        sm.open(1, f.path(), None).unwrap();
        sm.open(1, f.path(), None).unwrap(); // 不应重建
        assert_eq!(sm.chapters(1).unwrap().len(), 1);
    }
}
