use crate::core::database;
use crate::core::models::{Article, ArticleFilter, Category, Feed};
use tauri::AppHandle;

/// 初始化数据库
#[tauri::command]
pub async fn init_db(app: AppHandle) -> Result<String, String> {
    database::db_init(&app).map_err(|e| e.to_string())?;
    Ok("Database initialized successfully".to_string())
}

/// 获取所有 Feeds
#[tauri::command]
pub async fn get_all_feeds(app: AppHandle) -> Result<Vec<Feed>, String> {
    database::db_get_all_feeds(&app).map_err(|e| e.to_string())
}

/// 查询文章列表（支持分页和筛选）
#[tauri::command]
pub async fn get_articles(
    app: AppHandle,
    filter: ArticleFilter,
) -> Result<Vec<Article>, String> {
    database::db_query_articles(&app, &filter).map_err(|e| e.to_string())
}

/// 更新文章状态
#[tauri::command]
pub async fn update_article(
    app: AppHandle,
    article_id: i64,
    is_read: Option<bool>,
    is_starred: Option<bool>,
) -> Result<String, String> {
    database::db_update_article(&app, article_id, is_read, is_starred)
        .map_err(|e| e.to_string())?;
    Ok("Article updated successfully".to_string())
}

/// 删除 Feed
#[tauri::command]
pub async fn delete_feed(app: AppHandle, feed_id: i64) -> Result<String, String> {
    database::db_delete_feed(&app, feed_id).map_err(|e| e.to_string())?;
    Ok("Feed deleted successfully".to_string())
}

/// 获取所有分类及其未读计数
#[tauri::command]
pub async fn get_categories(app: AppHandle) -> Result<Vec<Category>, String> {
    database::db_get_categories(&app).map_err(|e| e.to_string())
}

/// 备份数据库到指定路径
#[tauri::command]
pub async fn backup_database(app: AppHandle, backup_path: String) -> Result<String, String> {
    let path = std::path::Path::new(&backup_path);
    database::db_backup_database(&app, path).map_err(|e| e.to_string())?;
    Ok("Database backed up successfully".to_string())
}

/// 从备份文件恢复数据库
#[tauri::command]
pub async fn restore_database(app: AppHandle, backup_path: String) -> Result<String, String> {
    let path = std::path::Path::new(&backup_path);
    database::db_restore_database(&app, path).map_err(|e| e.to_string())?;
    Ok("Database restored successfully".to_string())
}

/// 获取用户设置
#[tauri::command]
pub async fn get_user_setting(app: AppHandle, key: String) -> Result<Option<String>, String> {
    database::db_get_user_setting(&app, &key).map_err(|e| e.to_string())
}

/// 设置用户设置
#[tauri::command]
pub async fn set_user_setting(
    app: AppHandle,
    key: String,
    value: String,
) -> Result<(), String> {
    database::db_set_user_setting(&app, &key, &value).map_err(|e| e.to_string())
}
