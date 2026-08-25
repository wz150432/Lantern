pub mod txt;

use crate::error::AppResult;
use crate::models::{BookMeta, ChapterInfo};

pub trait BookFormat: Send + Sync {
    fn meta(&self) -> BookMeta;
    fn chapter_list(&self) -> Vec<ChapterInfo>;
    fn chapter_text(&self, index: usize) -> AppResult<String>;
}
