use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// RSS Feed 订阅源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    /// 数据库主键 ID
    pub id: i64,
    /// Feed URL
    pub url: String,
    /// Feed 标题
    pub title: String,
    /// Feed 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Feed 图标 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 分类（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 最后一次成功拉取的时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetched_at: Option<DateTime<Utc>>,
}

/// 文章/条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// 数据库主键 ID
    pub id: i64,
    /// 所属 Feed 的 ID
    pub feed_id: i64,
    /// 文章标题
    pub title: String,
    /// 文章链接
    pub link: String,
    /// 文章内容/摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 文章描述/摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 作者（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 发布时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    /// 是否已读
    pub is_read: bool,
    /// 是否收藏
    pub is_starred: bool,
    /// 创建时间（抓取时间）
    pub created_at: DateTime<Utc>,
}

/// Feed 拉取日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchLog {
    /// 数据库主键 ID
    pub id: i64,
    /// Feed ID
    pub feed_id: i64,
    /// 是否成功
    pub success: bool,
    /// 错误信息（失败时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 获取到的文章数量
    pub article_count: i64,
    /// 拉取耗时（毫秒）
    pub duration_ms: i64,
    /// 拉取时间
    pub fetched_at: DateTime<Utc>,
}

/// 用于创建新 Feed 的结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFeed {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub category: Option<String>,
}

/// 用于更新 Feed 的结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeed {
    pub feed_id: i64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub category: Option<String>,
}

/// 用于创建新 Article 的结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewArticle {
    pub feed_id: i64,
    pub title: String,
    pub link: String,
    pub content: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// 文章查询过滤器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArticleFilter {
    /// 按 Feed ID 过滤
    pub feed_id: Option<i64>,
    /// 只显示未读
    pub unread_only: bool,
    /// 只显示收藏
    pub starred_only: bool,
    /// 分页偏移量
    pub offset: Option<i64>,
    /// 分页限制
    pub limit: Option<i64>,
}

/// Feed 分类信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// 分类名称
    pub name: String,
    /// 该分类下的未读文章数
    pub unread_count: i64,
    /// 该分类下的 Feed 数量
    pub feed_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_serialization() {
        let feed = Feed {
            id: 1,
            url: "https://example.com/feed".to_string(),
            title: "Test Feed".to_string(),
            description: Some("Test Description".to_string()),
            icon_url: None,
            category: Some("Tech".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_fetched_at: None,
        };

        let json = serde_json::to_string(&feed).unwrap();
        assert!(json.contains("Test Feed"));
    }

    #[test]
    fn test_article_filter_default() {
        let filter = ArticleFilter::default();
        assert!(filter.feed_id.is_none());
        assert!(!filter.unread_only);
        assert!(!filter.starred_only);
    }
}
