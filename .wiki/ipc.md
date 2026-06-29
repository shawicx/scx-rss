# IPC 调用链

## Command 映射表

| Command | Frontend Caller | Rust File | Purpose |
|---------|----------------|-----------|---------|
| `get_all_feeds` | `useFeeds.getAllFeeds()` | `db.rs:14` | 获取所有 Feeds |
| `add_feed` | `useFeeds.addFeed()` | `feed.rs:14` | 添加 Feed |
| `fetch_and_update_feed` | `useFeeds.refreshFeed()` | `feed.rs:71` | 单个刷新 |
| `batch_refresh_feeds` | `useFeeds.refreshAllFeeds()` | `feed.rs:124` | 批量刷新 |
| `cancel_batch_refresh` | `useFeeds.cancelRefresh()` | `feed.rs:266` | 取消刷新 |
| `get_articles` | `useArticles.getArticles()` | `db.rs:20` | 查询文章 |
| `update_article` | `useArticles.markAsRead()` | `db.rs:29` | 更新状态 |
| `delete_feed` | `useFeeds.deleteFeed()` | `db.rs:42` | 删除 Feed |
| `get_categories` | `useCategories.getCategories()` | `db.rs:49` | 获取分类 |
| `export_opml` | `useOpml.exportOpml()` | `feed.rs:278` | 导出 OPML |
| `import_opml` | `useOpml.importOpml()` | `feed.rs:344` | 导入 OPML |
| `backup_database` | `useOpml.backupDb()` | `db.rs:55` | 备份数据库 |
| `restore_database` | `useOpml.restoreDb()` | `db.rs:63` | 恢复数据库 |
| `get_user_setting` | `useI18n.init()/setLocale()` | `db.rs:69` | 获取用户设置 |
| `set_user_setting` | `useI18n.setLocale()` | `db.rs:75` | 设置用户设置 |
| `get_system_locale` | `useI18n.getSystemLocale()` | `system.rs:1` | 获取系统语言 |

## 前后端调用链

### 添加 Feed
```
FeedList.vue
  ↓ useFeeds.addFeed(url, category)
  ↓ invoke('add_feed', { url, category })
  ↓ feed::add_feed command
  ↓ network::fetch_feed(url)
  ↓ parser::parse_feed(content)
  ↓ database::db_insert_feed()
  ↓ database::db_insert_articles()
  ↓ 返回 Feed 对象
```

### 批量刷新
```
FeedList.vue
  ↓ useFeeds.refreshAllFeeds()
  ↓ invoke('batch_refresh_feeds')
  ↓ feed::batch_refresh_feeds command
  ↓ for each feed:
  ├─ network::fetch_feed(url)
  ├─ parser::parse_feed(content)
  ├─ database::db_insert_articles()
  └─ app.emit('refresh-progress', data)
  ↓ 返回汇总结果
  ↓ RefreshProgress.vue 监听事件
```

## 事件系统

### 后端推送
```rust
// feed.rs:140
app.emit("refresh-progress", serde_json::json!({
    "type": "progress",
    "current": index + 1,
    "total": total,
    "feed_title": feed.title,
}))?;
```

### 前端监听
```typescript
// useFeeds.ts
const unlisten = await listen<RefreshProgress>(
    'refresh-progress',
    (event) => callback(event.payload)
)
```

## 数据序列化

### Frontend → Rust
```typescript
// 自动序列化为 JSON
await invoke('add_feed', {
    url: 'https://example.com/feed.xml',
    category: 'Tech'
})
```

```rust
// 自动反序列化
#[tauri::command]
pub async fn add_feed(
    app: AppHandle,
    url: String,
    category: Option<String>,
) -> Result<Feed, String>
```

### Rust → Frontend
```rust
// 自动序列化
#[derive(Serialize)]
pub struct Feed {
    pub id: i64,
    pub url: String,
    // ...
}

Ok(feed)
```

```typescript
// 自动反序列化
const feed = await invoke<Feed>('add_feed', { url, category })
```

## 性能特性

| Command | 响应时间 | 阻塞 | 风险 |
|---------|----------|------|------|
| `get_all_feeds` | ~10ms | ❌ | 无 |
| `get_articles` | ~50ms | ❌ | 大数据风险 |
| `add_feed` | ~2s | ✅ | 网络风险 |
| `batch_refresh_feeds` | ~60s | ✅ | 长时间运行 |
| `update_article` | ~5ms | ❌ | 无 |

**优化**: 批量操作使用 `batch_*` 命令，避免循环调用
