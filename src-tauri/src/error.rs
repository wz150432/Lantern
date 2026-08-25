use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Parse(String),
    Storage(String),
    NotFound(String),
    Invalid(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {e}"),
            AppError::Parse(m) => write!(f, "解析错误: {m}"),
            AppError::Storage(m) => write!(f, "存储错误: {m}"),
            AppError::NotFound(m) => write!(f, "未找到: {m}"),
            AppError::Invalid(m) => write!(f, "参数错误: {m}"),
        }
    }
}

impl std::error::Error for AppError {}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Storage(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Storage(format!("JSON: {e}"))
    }
}
