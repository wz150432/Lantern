use crate::error::AppResult;
use crate::parsers::BookFormat;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub chapter_index: usize,
    pub offset_in_chapter: usize,
    pub length: usize,
    pub snippet: String,
}

pub fn search_book(book: &dyn BookFormat, query: &str, limit: usize) -> AppResult<Vec<SearchHit>> {
    let query = query.trim();
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let chapters = book.chapter_list();
    let mut hits = Vec::new();
    'outer: for ch in &chapters {
        let text = book.chapter_text(ch.index)?;
        let mut search_from = 0usize;
        while let Some(rel) = text[search_from..].find(query) {
            let abs = search_from + rel;
            let start = text.floor_char_boundary(abs.saturating_sub(20));
            let end = text.ceil_char_boundary((abs + query.len() + 20).min(text.len()));
            let snippet = text[start..end].replace('\n', " ");
            hits.push(SearchHit {
                chapter_index: ch.index,
                offset_in_chapter: abs,
                length: query.len(),
                snippet,
            });
            if hits.len() >= limit {
                break 'outer;
            }
            search_from = abs + query.len();
        }
    }
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::txt::TxtBook;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const SAMPLE: &str = "第一章 开端\n他叫李四。\n李四喜欢读书。\n\n第二章 发展\n李四去了远方。\n";

    fn open_book() -> (TxtBook, NamedTempFile) {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(SAMPLE.as_bytes()).unwrap();
        let book = TxtBook::open(f.path(), None).unwrap();
        (book, f)
    }

    #[test]
    fn finds_all_hits_across_chapters() {
        let (book, _tmp) = open_book();
        let hits = search_book(&book, "李四", 100).unwrap();
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].chapter_index, 0);
        assert_eq!(hits[2].chapter_index, 1);
    }

    #[test]
    fn snippet_contains_context() {
        let (book, _tmp) = open_book();
        let hits = search_book(&book, "读书", 100).unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].snippet.contains("李四喜欢读书"));
    }

    #[test]
    fn limit_respected() {
        let (book, _tmp) = open_book();
        let hits = search_book(&book, "李四", 2).unwrap();
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn no_match_returns_empty() {
        let (book, _tmp) = open_book();
        let hits = search_book(&book, "不存在的词", 100).unwrap();
        assert!(hits.is_empty());
    }
}
