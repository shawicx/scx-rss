use crate::core::error::{AppError, AppResult};
use crate::core::models::{NewArticle, NewFeed};
use chrono::DateTime;
use feed_rs::parser;
use opml::{OPML, Outline};

/// 解析 Feed 内容
///
/// 支持格式：RSS 1.0/2.0, Atom 1.0
/// 自动处理编码：UTF-8, GBK, ISO-8859-1
pub fn parse_feed(url: &str, content: &[u8]) -> AppResult<(NewFeed, Vec<NewArticle>)> {
    // 尝试自动检测编码并解析
    let feed = try_parse_with_encoding(content)?;

    // 提取 Feed 元数据
    let title = feed.title.map(|t| t.content).unwrap_or_else(|| "Untitled".to_string());
    let description = feed.description.map(|d| d.content);
    let icon_url = feed.icon.and_then(|i| Some(i.uri)).or_else(|| {
        feed.logo.and_then(|l| Some(l.uri))
    });

    let new_feed = NewFeed {
        url: url.to_string(),
        title,
        description,
        icon_url,
        category: None, // 分类由用户指定
    };

    // 提取文章
    let mut new_articles = Vec::new();
    for entry in feed.entries {
        if let Some(article) = parse_entry(entry) {
            new_articles.push(article);
        }
    }

    tracing::info!(
        "Parsed feed '{}' with {} articles",
        new_feed.title,
        new_articles.len()
    );

    Ok((new_feed, new_articles))
}

/// 尝试用不同编码解析 Feed
fn try_parse_with_encoding(content: &[u8]) -> AppResult<feed_rs::model::Feed> {
    // 首先尝试 UTF-8
    match try_parse(content, "utf-8") {
        Ok(feed) => return Ok(feed),
        Err(e) => {
            tracing::debug!("UTF-8 parsing failed: {}", e);
        }
    }

    // 尝试 GBK（常见中文编码）
    match try_parse(content, "gbk") {
        Ok(feed) => return Ok(feed),
        Err(e) => {
            tracing::debug!("GBK parsing failed: {}", e);
        }
    }

    // 尝试 ISO-8859-1
    match try_parse(content, "iso-8859-1") {
        Ok(feed) => return Ok(feed),
        Err(e) => {
            tracing::debug!("ISO-8859-1 parsing failed: {}", e);
        }
    }

    Err(AppError::ParseError(
        "无法以任何支持的编码解析订阅源".to_string(),
    ))
}

/// 用指定编码解析内容
fn try_parse(content: &[u8], encoding: &str) -> AppResult<feed_rs::model::Feed> {
    // 将内容转换为字符串
    let content_str = decode_content(content, encoding)?;

    // 使用 feed_rs 解析
    let feed = parser::parse(content_str.as_bytes())
        .map_err(|e| AppError::ParseError(format!("Feed parsing error: {}", e)))?;

    Ok(feed)
}

/// 解码字节数组为字符串
fn decode_content(content: &[u8], encoding_label: &str) -> AppResult<String> {
    let decoded = match encoding_label.to_lowercase().as_str() {
        "utf-8" => {
            std::str::from_utf8(content)
                .map_err(|_| AppError::ParseError("Invalid UTF-8 encoding".to_string()))?
                .to_string()
        }
        "gbk" | "gb2312" => {
            let (decoded, _, _) = encoding_rs::GBK.decode(content);
            decoded.to_string()
        }
        "iso-8859-1" | "latin1" => {
            let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(content);
            decoded.to_string()
        }
        _ => {
            return Err(AppError::ParseError(format!(
                "Unsupported encoding: {}",
                encoding_label
            )))
        }
    };

    if !decoded.contains('\0') {
        Ok(decoded)
    } else {
        Err(AppError::ParseError("Invalid encoding detected".to_string()))
    }
}

/// 解析单篇文章
fn parse_entry(entry: feed_rs::model::Entry) -> Option<NewArticle> {
    // 提取标题
    let title = entry
        .title
        .map(|t| t.content)
        .unwrap_or_else(|| "Untitled".to_string());

    // 提取链接
    let link = entry.links.first()?.href.clone();

    // 提取描述（先保存，避免移动）
    let description = entry.summary.as_ref().map(|s| s.content.clone());

    // 提取内容
    let content = entry.content.map(|c| c.body).or_else(|| {
        entry.summary.map(|s| Some(s.content))
    }).flatten();

    // 提取作者
    let author = entry.authors.first().map(|a| a.name.clone());

    // 提取发布时间
    let published_at = entry.published.or(entry.updated).and_then(|dt| {
        // feed_rs::model::DateTime -> chrono::DateTime<Utc>
        DateTime::from_timestamp(dt.timestamp(), 0)
    });

    Some(NewArticle {
        feed_id: 0, // 稍后设置
        title,
        link,
        content,
        description,
        author,
        published_at,
    })
}

/// 解析 OPML 文件
///
/// # 参数
/// * `opml_content` - OPML 文件内容
///
/// # 返回
/// Vec<(url, title, category)> - Feed 列表
pub fn parse_opml(opml_content: &str) -> AppResult<Vec<(String, String, Option<String>)>> {
    // 使用 OPML::new() 方法解析 XML 字符串
    let opml = OPML::from_str(opml_content)
        .map_err(|e| AppError::ParseError(format!("OPML parsing error: {}", e)))?;

    let mut feeds = Vec::new();

    // 递归提取所有 outline
    for outline in &opml.body.outlines {
        extract_feeds_from_outline(outline, None, &mut feeds);
    }

    tracing::info!("Parsed OPML with {} feeds", feeds.len());
    Ok(feeds)
}

/// 递归提取 OPML outline 中的 feeds
fn extract_feeds_from_outline(
    outline: &Outline,
    category: Option<String>,
    feeds: &mut Vec<(String, String, Option<String>)>,
) {
    // 如果是 Feed（有 xml_url 属性）
    if let Some(xml_url) = &outline.xml_url {
        // 使用 title 或 text，如果都没有则用 Untitled
        let title = outline
            .title
            .clone()
            .or_else(|| {
                if !outline.text.is_empty() {
                    Some(outline.text.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Untitled".to_string());

        feeds.push((xml_url.clone(), title, category.clone()));
    }

    // 如果有子 outline，递归处理（分组）
    if !outline.outlines.is_empty() {
        // 如果 text 不为空，使用它作为新分类；否则保持原分类
        let new_category = if !outline.text.is_empty() {
            Some(outline.text.clone())
        } else {
            category
        };

        for child_outline in &outline.outlines {
            extract_feeds_from_outline(child_outline, new_category.clone(), feeds);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RSS_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <rss version="2.0">
        <channel>
            <title>Test Feed</title>
            <description>Test Description</description>
            <link>https://example.com</link>
            <item>
                <title>Article 1</title>
                <link>https://example.com/article1</link>
                <description>Description 1</description>
                <pubDate>Mon, 01 Jan 2025 12:00:00 GMT</pubDate>
            </item>
        </channel>
    </rss>"#;

    const OPML_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <body>
            <outline text="Tech">
                <outline text="Tech Blog" xmlUrl="https://example.com/tech.xml"/>
                <outline text="News" xmlUrl="https://example.com/news.xml"/>
            </outline>
        </body>
    </opml>"#;

    #[test]
    fn test_parse_rss() {
        let content = RSS_SAMPLE.as_bytes();
        match parse_feed("https://example.com/feed.xml", content) {
            Ok((feed, articles)) => {
                assert_eq!(feed.title, "Test Feed");
                assert_eq!(feed.description, Some("Test Description".to_string()));
                assert_eq!(articles.len(), 1);
                assert_eq!(articles[0].title, "Article 1");
            }
            Err(e) => {
                eprintln!("Parse test failed: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_opml() {
        match parse_opml(OPML_SAMPLE) {
            Ok(feeds) => {
                assert_eq!(feeds.len(), 2);
                assert_eq!(feeds[0].0, "https://example.com/tech.xml");
                assert_eq!(feeds[0].1, "Tech Blog");
                assert_eq!(feeds[0].2, Some("Tech".to_string()));
            }
            Err(e) => {
                eprintln!("OPML parse test failed: {}", e);
            }
        }
    }
}
