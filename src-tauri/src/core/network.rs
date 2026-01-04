use crate::core::error::{AppError, AppResult};
use std::time::Duration;
use tokio::time::sleep;

/// HTTP 客户端配置
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) SCX-RSS/0.1.0 (+https://github.com/scx)";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_REDIRECTS: usize = 5;
const MAX_RETRIES: u32 = 3;

/// 获取单个 Feed 的内容
///
/// # 参数
/// * `url` - Feed URL
///
/// # 返回
/// Feed 的原始内容（字节数组）
pub async fn fetch_feed(url: &str) -> AppResult<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(REQUEST_TIMEOUT)
        .redirect(reqwest::redirect::Policy::limited(MAX_REDIRECTS))
        .build()?;

    let current_url = url.to_string();
    let mut last_error = None;

    // 重试逻辑：指数退避
    for attempt in 0..MAX_RETRIES {
        // 计算退避时间：100ms * 2^attempt
        if attempt > 0 {
            let backoff_ms = 100 * 2_u32.pow(attempt - 1);
            tracing::warn!(
                "Retry {}/{} for {} after {}ms",
                attempt,
                MAX_RETRIES,
                current_url,
                backoff_ms
            );
            sleep(Duration::from_millis(backoff_ms as u64)).await;
        }

        match client.get(&current_url).send().await {
            Ok(response) => {
                // 检查状态码
                if !response.status().is_success() {
                    let status = response.status();
                    let error_msg = format!(
                        "HTTP error {}: {}",
                        status.as_u16(),
                        status.canonical_reason().unwrap_or("Unknown")
                    );
                    tracing::error!("{} for URL: {}", error_msg, current_url);
                    last_error = Some(AppError::ParseError(error_msg));
                    continue;
                }

                // 获取内容长度
                let content_length = response.content_length().unwrap_or(0);
                tracing::info!("Successfully fetched {} ({} bytes)", current_url, content_length);

                // 获取响应体
                let bytes = response.bytes().await?;
                return Ok(bytes.to_vec());
            }
            Err(e) => {
                tracing::error!("Attempt {}/{} failed: {}", attempt + 1, MAX_RETRIES, e);
                last_error = Some(AppError::from(e));
            }
        }
    }

    // 所有重试都失败
    Err(last_error.unwrap_or_else(|| {
        AppError::ParseError("All retries exhausted".to_string())
    }))
}

/// 批量获取多个 Feeds（并发控制）
///
/// # 参数
/// * `urls` - Feed URL 列表
///
/// # 返回
/// Vec<(url, result)> - 每个 URL 及其获取结果
pub async fn batch_fetch_feeds(urls: Vec<String>) -> Vec<(String, AppResult<Vec<u8>>)> {
    use futures::stream::{self, StreamExt};

    const MAX_CONCURRENT: usize = 3;
    const REQUEST_INTERVAL: Duration = Duration::from_millis(100);

    let mut results = Vec::new();

    // 使用 stream 来控制并发
    let mut stream = stream::iter(urls)
        .map(|url| {
            async move {
                // 请求间隔
                sleep(REQUEST_INTERVAL).await;
                let result = fetch_feed(&url).await;
                (url, result)
            }
        })
        .buffer_unordered(MAX_CONCURRENT);

    while let Some((url, result)) = stream.next().await {
        results.push((url, result));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_feed() {
        // 使用公开的 RSS feed 进行测试
        let url = "https://feeds.feedburner.com/oreilly/radar";
        match fetch_feed(url).await {
            Ok(content) => {
                assert!(!content.is_empty());
                println!("Fetched {} bytes", content.len());
            }
            Err(e) => {
                eprintln!("Network test failed (expected in CI): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_batch_fetch_feeds() {
        let urls = vec![
            "https://feeds.feedburner.com/oreilly/radar".to_string(),
            "https://www.nasa.gov/rss/dyn/breaking_news.rss".to_string(),
        ];

        let results = batch_fetch_feeds(urls).await;
        assert_eq!(results.len(), 2);

        for (url, result) in results {
            match result {
                Ok(content) => {
                    println!("✓ {} - {} bytes", url, content.len());
                    assert!(!content.is_empty());
                }
                Err(e) => {
                    eprintln!("✗ {} - {}", url, e);
                }
            }
        }
    }
}
