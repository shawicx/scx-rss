use thiserror::Error;

/// RSS 阅读器应用错误类型
///
/// 涵盖网络、解析、数据库和验证四大类错误
#[derive(Error, Debug)]
pub enum AppError {
    /// 网络请求错误
    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// RSS/Atom 解析错误
    #[error("订阅源解析错误: {0}")]
    ParseError(String),

    /// 数据库操作错误
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    /// 数据验证错误（如无效的 URL）
    #[error("验证错误: {0}")]
    ValidationError(String),

    /// JSON 序列化/反序列化错误
    #[error("JSON 错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO 错误
    #[error("IO 错误: {0}")]
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

impl AppError {
    /// 返回错误码，用于前端国际化
    pub fn code(&self) -> &'static str {
        match self {
            AppError::NetworkError(_) => "errors.network",
            AppError::ParseError(_) => "errors.parse",
            AppError::DatabaseError(_) => "errors.database",
            AppError::ValidationError(_) => "errors.validation",
            AppError::JsonError(_) => "errors.json",
            AppError::IoError(_) => "errors.io",
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
        assert_eq!(err.to_string(), "订阅源解析错误: Invalid RSS format");
    }

    #[test]
    fn test_error_clone() {
        let err1 = AppError::ValidationError("Invalid URL".to_string());
        let err2 = err1.clone();
        assert_eq!(err1.to_string(), err2.to_string());
    }

    #[test]
    fn test_error_code() {
        // Test each error variant returns the correct error code
        let parse_err = AppError::ParseError("test".to_string());
        assert_eq!(parse_err.code(), "errors.parse");

        let validation_err = AppError::ValidationError("test".to_string());
        assert_eq!(validation_err.code(), "errors.validation");

        let io_err = AppError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert_eq!(io_err.code(), "errors.io");

        let db_err = AppError::DatabaseError(rusqlite::Error::QueryReturnedNoRows);
        assert_eq!(db_err.code(), "errors.database");

        // Test that all error codes are unique and follow the expected pattern
        let all_codes = vec![
            ("errors.network", "network"),
            ("errors.parse", "parse"),
            ("errors.database", "database"),
            ("errors.validation", "validation"),
            ("errors.json", "json"),
            ("errors.io", "io"),
        ];

        for (code, name) in all_codes {
            assert!(code.starts_with("errors."), "Error code '{}' should start with 'errors.'", code);
            assert!(code.contains(name), "Error code '{}' should contain '{}'", code, name);
        }
    }
}
