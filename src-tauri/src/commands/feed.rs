use crate::core::database;
use crate::core::models::{Feed, NewFeed, NewArticle};
use crate::core::network;
use crate::core::parser;
use tauri::{AppHandle, Emitter};
use tokio_util::sync::CancellationToken;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 全局取消令牌
static CANCEL_TOKEN: Lazy<Mutex<Option<CancellationToken>>> = Lazy::new(|| Mutex::new(None));

/// 添加新的 Feed
#[tauri::command]
pub async fn add_feed(
    app: AppHandle,
    url: String,
    category: Option<String>,
) -> Result<Feed, String> {
    // 1. 获取 Feed 内容
    let content = network::fetch_feed(&url).await.map_err(|e| e.to_string())?;

    // 2. 解析 Feed
    let (mut new_feed, articles) = parser::parse_feed(&url, &content)
        .map_err(|e| format!("订阅源解析失败: {}", e))?;

    // 3. 设置分类
    new_feed.category = category;

    // 4. 插入数据库
    let feed = database::db_insert_feed(&app, &new_feed).map_err(|e| e.to_string())?;

    // 5. 插入文章
    let articles: Vec<NewArticle> = articles
        .into_iter()
        .map(|mut a| {
            a.feed_id = feed.id;
            a
        })
        .collect();

    if !articles.is_empty() {
        let count = database::db_insert_articles(&app, &articles).map_err(|e| e.to_string())?;
        tracing::info!("Inserted {} articles for feed {}", count, feed.id);
    }

    Ok(feed)
}

/// 更新 Feed 信息
#[tauri::command]
pub async fn update_feed(
    app: AppHandle,
    feed_id: i64,
    title: Option<String>,
    url: Option<String>,
    category: Option<String>,
) -> Result<Feed, String> {
    let update = crate::core::models::UpdateFeed {
        feed_id,
        title,
        url,
        category,
    };

    crate::core::database::db_update_feed(&app, &update).map_err(|e| e.to_string())
}

/// 拉取并更新单个 Feed
#[tauri::command]
pub async fn fetch_and_update_feed(app: AppHandle, feed_id: i64) -> Result<String, String> {
    let start_time = std::time::Instant::now();

    // 1. 获取 Feed 信息
    let feeds = database::db_get_all_feeds(&app).map_err(|e| e.to_string())?;
    let feed = feeds
        .iter()
        .find(|f| f.id == feed_id)
        .ok_or_else(|| format!("未找到 id 为 {} 的订阅源", feed_id))?;

    // 2. 获取最新内容
    let content =
        network::fetch_feed(&feed.url).await.map_err(|e| format!("网络错误: {}", e))?;

    // 3. 解析 Feed
    let (_parsed_feed, articles) = parser::parse_feed(&feed.url, &content)
        .map_err(|e| format!("解析错误: {}", e))?;

    // 4. 插入新文章
    let articles: Vec<NewArticle> = articles
        .into_iter()
        .map(|mut a| {
            a.feed_id = feed.id;
            a
        })
        .collect();

    let inserted_count = if !articles.is_empty() {
        database::db_insert_articles(&app, &articles).map_err(|e| e.to_string())?
    } else {
        0
    };

    // 5. 更新最后拉取时间
    database::db_update_feed_last_fetched(&app, feed_id).map_err(|e| e.to_string())?;

    let duration = start_time.elapsed().as_millis();

    tracing::info!(
        "Feed {} updated: {} new articles in {}ms",
        feed_id,
        inserted_count,
        duration
    );

    Ok(format!(
        "Successfully updated feed. {} new articles added.",
        inserted_count
    ))
}

/// 批量刷新所有 Feeds
#[tauri::command]
pub async fn batch_refresh_feeds(app: AppHandle) -> Result<String, String> {
    // 1. 获取所有 Feeds
    let feeds = database::db_get_all_feeds(&app).map_err(|e| e.to_string())?;

    if feeds.is_empty() {
        return Ok("No feeds to refresh".to_string());
    }

    let total = feeds.len();
    tracing::info!("Starting batch refresh for {} feeds", total);

    // 2. 创建并存储取消令牌
    let token = CancellationToken::new();
    *CANCEL_TOKEN.lock().unwrap() = Some(token.clone());

    // 3. 发送开始事件
    let _ = app.emit("refresh-progress", serde_json::json!({
        "type": "start",
        "total": total,
    }));

    let mut success_count = 0;
    let mut failure_count = 0;
    let mut total_new_articles = 0;

    // 4. 逐个刷新 Feeds（为了进度报告）
    for (index, feed) in feeds.iter().enumerate() {
        // 检查是否被取消
        if token.is_cancelled() {
            tracing::info!("Batch refresh cancelled at {}/{}", index + 1, total);

            let _ = app.emit("refresh-progress", serde_json::json!({
                "type": "cancelled",
                "current": index + 1,
                "total": total,
            }));

            return Ok("Batch refresh cancelled".to_string());
        }

        // 发送当前进度
        let _ = app.emit("refresh-progress", serde_json::json!({
            "type": "progress",
            "current": index + 1,
            "total": total,
            "feed_title": feed.title.clone(),
        }));

        // 拉取并处理 Feed
        match network::fetch_feed(&feed.url).await {
            Ok(content) => {
                match parser::parse_feed(&feed.url, &content) {
                    Ok((_parsed_feed, articles)) => {
                        let articles: Vec<NewArticle> = articles
                            .into_iter()
                            .map(|mut a| {
                                a.feed_id = feed.id;
                                a
                            })
                            .collect();

                        match database::db_insert_articles(&app, &articles) {
                            Ok(count) => {
                                total_new_articles += count;
                                success_count += 1;
                                let _ = database::db_update_feed_last_fetched(&app, feed.id);

                                // 发送成功事件
                                let _ = app.emit("refresh-progress", serde_json::json!({
                                    "type": "feed-success",
                                    "current": index + 1,
                                    "total": total,
                                    "feed_title": feed.title.clone(),
                                    "new_articles": count,
                                }));
                            }
                            Err(e) => {
                                tracing::error!("Failed to insert articles for feed {}: {}", feed.id, e);
                                failure_count += 1;

                                // 发送失败事件
                                let _ = app.emit("refresh-progress", serde_json::json!({
                                    "type": "feed-error",
                                    "current": index + 1,
                                    "total": total,
                                    "feed_title": feed.title.clone(),
                                    "error": e.to_string(),
                                }));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse feed {}: {}", feed.id, e);
                        failure_count += 1;

                        let _ = app.emit("refresh-progress", serde_json::json!({
                            "type": "feed-error",
                            "current": index + 1,
                            "total": total,
                            "feed_title": feed.title.clone(),
                            "error": e.to_string(),
                        }));
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch feed {}: {}", feed.id, e);
                failure_count += 1;

                let _ = app.emit("refresh-progress", serde_json::json!({
                    "type": "feed-error",
                    "current": index + 1,
                    "total": total,
                    "feed_title": feed.title.clone(),
                    "error": e.to_string(),
                }));
            }
        }
    }

    // 5. 清除取消令牌
    *CANCEL_TOKEN.lock().unwrap() = None;

    // 6. 发送完成事件
    let message = format!(
        "Batch refresh completed. Success: {}, Failed: {}, New articles: {}",
        success_count, failure_count, total_new_articles
    );

    let _ = app.emit("refresh-progress", serde_json::json!({
        "type": "complete",
        "success": success_count,
        "failed": failure_count,
        "total_new_articles": total_new_articles,
    }));

    tracing::info!("{}", message);
    Ok(message)
}

/// 取消批量刷新
#[tauri::command]
pub async fn cancel_batch_refresh() -> Result<bool, String> {
    // 尝试获取并取消当前刷新
    let mut token_guard = CANCEL_TOKEN.lock().unwrap();
    if let Some(token) = token_guard.take() {
        token.cancel();
        Ok(true)
    } else {
        Err("No active refresh to cancel".to_string())
    }
}

/// 导出 OPML
#[tauri::command]
pub async fn export_opml(app: AppHandle) -> Result<String, String> {
    use opml::{OPML, Outline};

    // 1. 获取所有 Feeds
    let feeds = database::db_get_all_feeds(&app).map_err(|e| e.to_string())?;

    // 2. 按分类分组
    let mut categories: std::collections::HashMap<Option<String>, Vec<&Feed>> =
        std::collections::HashMap::new();

    for feed in &feeds {
        categories
            .entry(feed.category.clone())
            .or_insert_with(Vec::new)
            .push(feed);
    }

    // 3. 创建 OPML 文档
    let mut body_outlines = Vec::new();

    for (category, category_feeds) in categories {
        if let Some(cat_name) = category {
            // 创建分类 outline
            let mut category_outlines = Vec::new();
            for feed in category_feeds {
                category_outlines.push(Outline {
                    text: feed.title.clone(),
                    title: Some(feed.title.clone()),
                    xml_url: Some(feed.url.clone()),
                    ..Outline::default()
                });
            }

            body_outlines.push(Outline {
                text: cat_name,
                outlines: category_outlines,
                ..Outline::default()
            });
        } else {
            // 未分类的 feeds
            for feed in category_feeds {
                body_outlines.push(Outline {
                    text: feed.title.clone(),
                    title: Some(feed.title.clone()),
                    xml_url: Some(feed.url.clone()),
                    ..Outline::default()
                });
            }
        }
    }

    let opml = OPML {
        version: "2.0".to_string(),
        head: None,
        body: opml::Body { outlines: body_outlines },
    };

    // 4. 序列化为 XML
    let opml_xml = opml.to_string()
        .map_err(|e| format!("Failed to serialize OPML: {}", e))?;

    Ok(opml_xml)
}

/// 导入 OPML
#[tauri::command]
pub async fn import_opml(app: AppHandle, opml_content: String) -> Result<String, String> {
    // 1. 解析 OPML
    let feeds = parser::parse_opml(&opml_content).map_err(|e| e.to_string())?;

    if feeds.is_empty() {
        return Ok("No feeds found in OPML file".to_string());
    }

    tracing::info!("Importing {} feeds from OPML", feeds.len());

    // 2. 批量添加 Feeds
    let mut success_count = 0;
    let mut failure_count = 0;

    for (url, title, category) in feeds {
        // 创建 NewFeed
        let new_feed = NewFeed {
            url: url.clone(),
            title,
            description: None,
            icon_url: None,
            category,
        };

        // 尝试插入数据库（忽略已存在的）
        match database::db_insert_feed(&app, &new_feed) {
            Ok(_) => {
                success_count += 1;
                tracing::info!("Imported feed: {}", url);
            }
            Err(e) => {
                // 可能是重复的 URL
                if e.to_string().contains("UNIQUE constraint failed") {
                    tracing::warn!("Feed already exists: {}", url);
                } else {
                    tracing::error!("Failed to import feed {}: {}", url, e);
                    failure_count += 1;
                }
            }
        }
    }

    let message = format!(
        "OPML import completed. Added: {}, Failed: {}",
        success_count, failure_count
    );

    tracing::info!("{}", message);
    Ok(message)
}
