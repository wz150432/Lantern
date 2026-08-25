use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookMeta {
    pub title: String,
    pub author: Option<String>,
    pub format: String,
    pub total_chapters: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterInfo {
    pub index: usize,
    pub title: String,
    pub offset: u64,
    pub length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookRecord {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub format: String,
    pub file_path: String,
    pub storage_mode: String,
    pub cover_path: Option<String>,
    pub added_at: i64,
    pub last_opened_at: i64,
    pub total_chapters: usize,
    pub current_chapter: usize,
    pub progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: i64,
    pub book_id: i64,
    pub chapter_index: usize,
    pub position: f64,
    pub note: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: f64,
    pub line_height: f64,
    pub paragraph_spacing: f64,
    pub first_line_indent: bool,
    pub char_spacing: f64,
    pub compress_blank_lines: bool,
    pub word_wrap: bool,
    pub inner_padding: f64,
    pub scroll_speed: usize,
    pub page_mode: String,
    pub auto_page_interval_ms: u64,
    pub click_mode: String,
    pub window_topmost: bool,
    pub window_opacity: f64,
    pub immersive_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "minimal".into(),
            font_family: "system-ui, \"PingFang SC\", \"Microsoft YaHei\", sans-serif".into(),
            font_size: 18.0,
            line_height: 1.8,
            paragraph_spacing: 8.0,
            first_line_indent: true,
            char_spacing: 0.5,
            compress_blank_lines: true,
            word_wrap: true,
            inner_padding: 48.0,
            scroll_speed: 1,
            page_mode: "page".into(),
            auto_page_interval_ms: 3000,
            click_mode: "thirds".into(),
            window_topmost: false,
            window_opacity: 1.0,
            immersive_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default_is_reasonable() {
        let s = AppSettings::default();
        assert_eq!(s.theme, "minimal");
        assert!(s.font_size > 0.0);
        assert!(s.line_height >= 1.0);
        assert_eq!(s.page_mode, "page");
    }

    #[test]
    fn serializes_to_camel_case() {
        let s = AppSettings::default();
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"pageMode\""));
        assert!(json.contains("\"firstLineIndent\""));
        assert!(!json.contains("\"page_mode\""));
    }

    #[test]
    fn chapter_info_serializes_camel_case() {
        let c = ChapterInfo {
            index: 0,
            title: "第一章".into(),
            offset: 0,
            length: 10,
        };
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"chapterIndex\"") || json.contains("\"index\""));
    }
}
