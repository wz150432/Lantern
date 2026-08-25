use crate::encodings::{decode_bytes, detect_encoding, DetectedEncoding};
use crate::error::{AppError, AppResult};
use crate::models::{BookMeta, ChapterInfo};
use crate::parsers::BookFormat;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub struct TxtBook {
    meta: BookMeta,
    chapters: Vec<ChapterInfo>,
    path: std::path::PathBuf,
    encoding: DetectedEncoding,
    #[allow(dead_code)]
    chapter_regex: Option<Regex>,
}

impl TxtBook {
    pub fn open(path: &Path, custom_regex: Option<&str>) -> AppResult<TxtBook> {
        let bytes = std::fs::read(path)?;
        let encoding = detect_encoding(&bytes);
        let text = decode_bytes(&bytes)?;
        let regex = match custom_regex {
            Some(r) if !r.trim().is_empty() => {
                Some(Regex::new(r).map_err(|e| AppError::Invalid(format!("正则无效: {e}")))?)
            }
            _ => None,
        };

        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("未命名")
            .to_string();

        let default_regex = Self::default_regex();
        let re = regex.as_ref().unwrap_or(&default_regex);
        let total_chars = text.chars().count();
        let mut chapters: Vec<ChapterInfo> = Vec::new();
        let mut current_title = title.clone();
        let mut current_start = 0usize;
        let mut line_start = 0usize;
        let mut seen_title = false;

        for line in text.lines() {
            let line_end = line_start + line.chars().count();
            let next_start = if line_end < total_chars {
                line_end + 1
            } else {
                line_end
            };

            if re.is_match(line.trim()) && !line.trim().is_empty() {
                if seen_title {
                    chapters.push(ChapterInfo {
                        index: chapters.len(),
                        title: current_title.clone(),
                        offset: byte_offset(&text, current_start),
                        length: byte_offset(&text, line_start) - byte_offset(&text, current_start),
                    });
                }
                current_title = line.trim().to_string();
                current_start = line_start;
                seen_title = true;
            }
            line_start = next_start;
        }

        chapters.push(ChapterInfo {
            index: chapters.len(),
            title: current_title.clone(),
            offset: byte_offset(&text, current_start),
            length: byte_offset(&text, total_chars) - byte_offset(&text, current_start),
        });

        let meta = BookMeta {
            title: title.clone(),
            author: None,
            format: "txt".into(),
            total_chapters: chapters.len(),
        };

        Ok(TxtBook {
            meta,
            chapters,
            path: path.to_path_buf(),
            encoding,
            chapter_regex: regex,
        })
    }

    pub fn default_regex() -> Regex {
        Regex::new(r"^\s*(第[0-9零一二三四五六七八九十百千万两]+[章卷节回部集]|序章|序言|前言|楔子|番外|后记|尾声|正文)([\s:：、.．·\-—–].*)?\s*$").unwrap()
    }

    pub fn encoding(&self) -> DetectedEncoding {
        self.encoding
    }

    pub fn file_mtime(&self) -> AppResult<std::time::SystemTime> {
        Ok(std::fs::metadata(&self.path)?.modified()?)
    }
}

/// 计算 UTF-8 字符串中字符偏移 `char_pos` 对应的字节偏移。
fn byte_offset(text: &str, char_pos: usize) -> u64 {
    text.char_indices()
        .nth(char_pos)
        .map(|(b, _)| b as u64)
        .unwrap_or(text.len() as u64)
}

impl BookFormat for TxtBook {
    fn meta(&self) -> BookMeta {
        self.meta.clone()
    }

    fn chapter_list(&self) -> Vec<ChapterInfo> {
        self.chapters.clone()
    }

    fn chapter_text(&self, index: usize) -> AppResult<String> {
        let ch = self
            .chapters
            .get(index)
            .ok_or_else(|| AppError::NotFound(format!("章节 {index} 不存在")))?;
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(ch.offset))?;
        let mut buf = vec![0u8; ch.length as usize];
        file.read_exact(&mut buf)?;
        decode_bytes(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const SAMPLE: &str =
        "第一章 风起\n内容一。\n内容二。\n\n第二章 云涌\n内容三。\n第三章 结束\n内容四。\n";

    fn write_tmp(content: &[u8]) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn parses_default_chapters() {
        let f = write_tmp(SAMPLE.as_bytes());
        let book = TxtBook::open(f.path(), None).unwrap();
        let chapters = book.chapter_list();
        assert_eq!(chapters.len(), 3);
        assert_eq!(chapters[0].title, "第一章 风起");
        assert_eq!(chapters[2].title, "第三章 结束");
    }

    #[test]
    fn chapter_text_by_offset() {
        let f = write_tmp(SAMPLE.as_bytes());
        let book = TxtBook::open(f.path(), None).unwrap();
        let text = book.chapter_text(1).unwrap();
        assert!(text.contains("内容三。"));
        assert!(!text.contains("第一章"));
    }

    #[test]
    fn no_chapters_yields_single() {
        let f = write_tmp("没有任何章节标记的普通文本。\n第二行。".as_bytes());
        let book = TxtBook::open(f.path(), None).unwrap();
        assert_eq!(book.chapter_list().len(), 1);
        assert_eq!(book.meta().total_chapters, 1);
    }

    #[test]
    fn custom_regex_overrides_default() {
        let f = write_tmp("PART 1 开头\n内容。\nPART 2 结尾\n内容。".as_bytes());
        let book = TxtBook::open(f.path(), Some(r"^PART \d+")).unwrap();
        assert_eq!(book.chapter_list().len(), 2);
    }

    #[test]
    fn gbk_file_decodes_and_parses() {
        use encoding_rs::GBK;
        let (bytes, _, _) = GBK.encode(SAMPLE);
        let f = write_tmp(&bytes.into_owned());
        let book = TxtBook::open(f.path(), None).unwrap();
        assert_eq!(book.encoding(), DetectedEncoding::Gbk);
        assert_eq!(book.chapter_list().len(), 3);
        assert!(book.chapter_text(0).unwrap().contains("内容一"));
    }

    #[test]
    fn missing_file_errors() {
        assert!(TxtBook::open(Path::new("/nonexistent/definitely_missing.txt"), None).is_err());
    }
}
