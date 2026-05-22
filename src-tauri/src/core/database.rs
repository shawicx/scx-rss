use crate::core::error::AppResult;
use crate::core::models::{Article, ArticleFilter, Category, Feed, NewArticle, NewFeed, UpdateFeed};
use chrono::Utc;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// 获取数据库文件路径
pub fn get_db_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("获取应用数据目录失败")
        .join("scx_rss.db")
}

/// 初始化数据库，创建所有表和索引
pub fn db_init(app: &AppHandle) -> AppResult<()> {
    let db_path = get_db_path(app);

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&db_path)?;

    // 创建 feeds 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            description TEXT,
            icon_url TEXT,
            category TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            last_fetched_at TEXT
        )",
        [],
    )?;

    // 创建 articles 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feed_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            link TEXT NOT NULL,
            content TEXT,
            description TEXT,
            author TEXT,
            published_at TEXT,
            is_read BOOLEAN NOT NULL DEFAULT 0,
            is_starred BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY(feed_id) REFERENCES feeds(id) ON DELETE CASCADE,
            UNIQUE(feed_id, link)
        )",
        [],
    )?;

    // 创建 fetch_logs 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fetch_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feed_id INTEGER NOT NULL,
            success BOOLEAN NOT NULL,
            error_message TEXT,
            article_count INTEGER NOT NULL,
            duration_ms INTEGER NOT NULL,
            fetched_at TEXT NOT NULL,
            FOREIGN KEY(feed_id) REFERENCES feeds(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建 user_settings 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;

    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_feed_id ON articles(feed_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_published_at ON articles(published_at DESC)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_is_read ON articles(is_read)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_is_starred ON articles(is_starred)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_settings_key ON user_settings(key)",
        [],
    )?;

    tracing::info!("Database initialized at: {:?}", db_path);
    Ok(())
}

/// 插入新的 Feed
pub fn db_insert_feed(app: &AppHandle, new_feed: &NewFeed) -> AppResult<Feed> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let now = Utc::now();
    let created_at = now.to_rfc3339();
    let updated_at = now.to_rfc3339();

    conn.execute(
        "INSERT INTO feeds (url, title, description, icon_url, category, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            new_feed.url,
            new_feed.title,
            new_feed.description,
            new_feed.icon_url,
            new_feed.category,
            created_at,
            updated_at,
        ],
    )?;

    let id = conn.last_insert_rowid();

    // 查询并返回完整的 Feed 对象
    let feed = conn.query_row(
        "SELECT id, url, title, description, icon_url, category, created_at, updated_at, last_fetched_at
         FROM feeds WHERE id = ?1",
        params![id],
        |row| {
            Ok(Feed {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                icon_url: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get::<_, String>(6)?.parse().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                last_fetched_at: row
                    .get::<_, Option<String>>(8)?
                    .map(|s| s.parse().unwrap()),
            })
        },
    )?;

    Ok(feed)
}

/// 批量插入文章（使用事务）
pub fn db_insert_articles(app: &AppHandle, articles: &[NewArticle]) -> AppResult<usize> {
    if articles.is_empty() {
        return Ok(0);
    }

    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let tx = conn.unchecked_transaction()?;

    let mut inserted_count = 0;
    for article in articles {
        let now = Utc::now().to_rfc3339();

        match tx.execute(
            "INSERT OR IGNORE INTO articles
             (feed_id, title, link, content, description, author, published_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                article.feed_id,
                article.title,
                article.link,
                article.content,
                article.description,
                article.author,
                article.published_at.map(|dt| dt.to_rfc3339()),
                now,
            ],
        ) {
            Ok(rows_affected) => inserted_count += rows_affected,
            Err(e) => {
                tracing::warn!("Failed to insert article: {}", e);
            }
        }
    }

    tx.commit()?;
    tracing::info!("Inserted {} articles", inserted_count);
    Ok(inserted_count as usize)
}

/// 查询文章列表（支持分页和筛选）
pub fn db_query_articles(app: &AppHandle, filter: &ArticleFilter) -> AppResult<Vec<Article>> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let mut query = String::from(
        "SELECT id, feed_id, title, link, content, description, author, published_at, is_read, is_starred, created_at
         FROM articles WHERE 1=1",
    );
    let mut params = Vec::new();

    if let Some(feed_id) = filter.feed_id {
        query.push_str(" AND feed_id = ?");
        params.push(feed_id.to_string());
    }

    if filter.unread_only {
        query.push_str(" AND is_read = 0");
    }

    if filter.starred_only {
        query.push_str(" AND is_starred = 1");
    }

    query.push_str(" ORDER BY published_at DESC, created_at DESC");

    if let Some(limit) = filter.limit {
        query.push_str(&format!(" LIMIT {}", limit));
        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
    }

    let mut stmt = conn.prepare(&query)?;
    let mut articles = Vec::new();

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

    let article_iter = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Article {
            id: row.get(0)?,
            feed_id: row.get(1)?,
            title: row.get(2)?,
            link: row.get(3)?,
            content: row.get(4)?,
            description: row.get(5)?,
            author: row.get(6)?,
            published_at: row
                .get::<_, Option<String>>(7)?
                .map(|s| s.parse().unwrap()),
            is_read: row.get(8)?,
            is_starred: row.get(9)?,
            created_at: row.get::<_, String>(10)?.parse().unwrap(),
        })
    })?;

    for article in article_iter {
        articles.push(article?);
    }

    Ok(articles)
}

/// 更新文章状态（已读/收藏）
pub fn db_update_article(
    app: &AppHandle,
    article_id: i64,
    is_read: Option<bool>,
    is_starred: Option<bool>,
) -> AppResult<()> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(read) = is_read {
        updates.push("is_read = ?");
        params.push(if read { "1" } else { "0" }.to_string());
    }

    if let Some(starred) = is_starred {
        updates.push("is_starred = ?");
        params.push(if starred { "1" } else { "0" }.to_string());
    }

    if updates.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE articles SET {} WHERE id = ?", updates.join(", "));
    params.push(article_id.to_string());

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

    conn.execute(&query, param_refs.as_slice())?;
    Ok(())
}

/// 删除 Feed（级联删除相关文章）
pub fn db_delete_feed(app: &AppHandle, feed_id: i64) -> AppResult<()> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    conn.execute("DELETE FROM feeds WHERE id = ?", params![feed_id])?;
    tracing::info!("Deleted feed with id: {}", feed_id);
    Ok(())
}

/// 获取所有 Feeds
pub fn db_get_all_feeds(app: &AppHandle) -> AppResult<Vec<Feed>> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let mut stmt = conn.prepare(
        "SELECT id, url, title, description, icon_url, category, created_at, updated_at, last_fetched_at
         FROM feeds ORDER BY title",
    )?;

    let mut feeds = Vec::new();

    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            icon_url: row.get(4)?,
            category: row.get(5)?,
            created_at: row.get::<_, String>(6)?.parse().unwrap(),
            updated_at: row.get::<_, String>(7)?.parse().unwrap(),
            last_fetched_at: row
                .get::<_, Option<String>>(8)?
                .map(|s| s.parse().unwrap()),
        })
    })?;

    for feed in feed_iter {
        feeds.push(feed?);
    }

    Ok(feeds)
}

/// 更新 Feed 的最后拉取时间
pub fn db_update_feed_last_fetched(app: &AppHandle, feed_id: i64) -> AppResult<()> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE feeds SET last_fetched_at = ?1, updated_at = ?1 WHERE id = ?2",
        params![now, feed_id],
    )?;

    Ok(())
}

/// 更新 Feed 的可编辑字段（title, url, category）
pub fn db_update_feed(app: &AppHandle, update: &UpdateFeed) -> AppResult<Feed> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let now = Utc::now().to_rfc3339();

    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ref title) = update.title {
        updates.push("title = ?");
        params.push(Box::new(title.clone()));
    }

    if let Some(ref url) = update.url {
        updates.push("url = ?");
        params.push(Box::new(url.clone()));
    }

    if let Some(ref category) = update.category {
        updates.push("category = ?");
        params.push(Box::new(category.clone()));
    }

    if updates.is_empty() {
        return conn.query_row(
            "SELECT id, url, title, description, icon_url, category, created_at, updated_at, last_fetched_at
             FROM feeds WHERE id = ?1",
            params![update.feed_id],
            |row| {
                Ok(Feed {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    icon_url: row.get(4)?,
                    category: row.get(5)?,
                    created_at: row.get::<_, String>(6)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                    last_fetched_at: row.get::<_, Option<String>>(8)?.map(|s| s.parse().unwrap()),
                })
            },
        ).map_err(|e| e.into());
    }

    updates.push("updated_at = ?");
    params.push(Box::new(now));

    params.push(Box::new(update.feed_id));

    let query = format!(
        "UPDATE feeds SET {} WHERE id = ?",
        updates.join(", ")
    );

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    if let Err(e) = conn.execute(&query, param_refs.as_slice()) {
        if e.to_string().contains("UNIQUE constraint failed") {
            return Err(crate::core::error::AppError::ValidationError(
                "该 URL 已被其他订阅源使用".to_string(),
            ));
        }
        return Err(e.into());
    }

    conn.query_row(
        "SELECT id, url, title, description, icon_url, category, created_at, updated_at, last_fetched_at
         FROM feeds WHERE id = ?1",
        params![update.feed_id],
        |row| {
            Ok(Feed {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                icon_url: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get::<_, String>(6)?.parse().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                last_fetched_at: row.get::<_, Option<String>>(8)?.map(|s| s.parse().unwrap()),
            })
        },
    ).map_err(|e| e.into())
}

/// 获取所有分类及其未读计数
pub fn db_get_categories(app: &AppHandle) -> AppResult<Vec<Category>> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    // 查询所有分类、每个分类的 Feed 数量和未读文章数
    let mut stmt = conn.prepare(
        "SELECT
            TRIM(COALESCE(f.category, '未分类')) as category_name,
            COUNT(DISTINCT f.id) as feed_count,
            COUNT(DISTINCT CASE WHEN a.is_read = 0 THEN a.id END) as unread_count
        FROM feeds f
        LEFT JOIN articles a ON f.id = a.feed_id
        GROUP BY TRIM(COALESCE(f.category, '未分类'))
        ORDER BY category_name",
    )?;

    let mut categories = Vec::new();

    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            name: row.get(0)?,
            feed_count: row.get(1)?,
            unread_count: row.get(2)?,
        })
    })?;

    for category in category_iter {
        categories.push(category?);
    }

    Ok(categories)
}

/// 备份数据库到指定路径
pub fn db_backup_database(app: &AppHandle, backup_path: &std::path::Path) -> AppResult<()> {
    let db_path = get_db_path(app);
    if !db_path.exists() {
        return Err(crate::core::error::AppError::ValidationError(
            "Database file not found".to_string(),
        ));
    }
    std::fs::copy(&db_path, backup_path)?;
    tracing::info!("Database backed up to: {:?}", backup_path);
    Ok(())
}

/// 从备份文件恢复数据库
pub fn db_restore_database(app: &AppHandle, backup_path: &std::path::Path) -> AppResult<()> {
    let db_path = get_db_path(app);
    if !backup_path.exists() {
        return Err(crate::core::error::AppError::ValidationError(
            "Backup file not found".to_string(),
        ));
    }
    let tmp_path = db_path.with_extension("db.tmp");
    std::fs::copy(backup_path, &tmp_path)?;
    std::fs::rename(&tmp_path, &db_path)?;
    tracing::info!("Database restored from: {:?}", backup_path);
    Ok(())
}

/// 获取用户设置
pub fn db_get_user_setting(app: &AppHandle, key: &str) -> AppResult<Option<String>> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    match conn.query_row(
        "SELECT value FROM user_settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    ) {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// 设置用户设置
pub fn db_set_user_setting(app: &AppHandle, key: &str, value: &str) -> AppResult<()> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let now = Utc::now().timestamp();
    conn.execute(
        "INSERT OR REPLACE INTO user_settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
        params![key, value, now],
    )?;
    tracing::info!("User setting updated: {} = {}", key, value);
    Ok(())
}
