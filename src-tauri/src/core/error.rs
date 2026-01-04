use thiserror::Error;

/// RSS 阅读器应用错误类型
///
/// 涵盖网络、解析、数据库和验证四大类错误
#[derive(Error, Debug)]
pub enum AppError {
    /// 网络请求错误
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// RSS/Atom 解析错误
    #[error("Feed parse error: {0}")]
    ParseError(String),

    /// 数据库操作错误
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    /// 数据验证错误（如无效的 URL）
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// 实现 Clone 以便在多线程环境中传递错误信息
impl Clone for AppError {
    fn clone(&self) -> Self {
        match self {
            AppError::NetworkError(e) => {
                // reqwest::Error 无法克隆，转换为 ParseError 保留错误信息
                AppError::ParseError(format!("Network error: {}", e))
            }
            AppError::ParseError(msg) => AppError::ParseError(msg.clone()),
            AppError::DatabaseError(e) => {
                AppError::ParseError(format!("Database error: {}", e))
            }
            AppError::ValidationError(msg) => AppError::ValidationError(msg.clone()),
            AppError::JsonError(e) => AppError::ParseError(format!("JSON error: {}", e)),
            AppError::IoError(e) => AppError::ParseError(format!("IO error: {}", e)),
        }
    }
}

/// 将 AppError 转换为 String，供 Tauri 使用
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

/// 应用结果类型别名
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::ParseError("Invalid RSS format".to_string());
        assert_eq!(err.to_string(), "Feed parse error: Invalid RSS format");
    }

    #[test]
    fn test_error_clone() {
        let err1 = AppError::ValidationError("Invalid URL".to_string());
        let err2 = err1.clone();
        assert_eq!(err1.to_string(), err2.to_string());
    }
}
