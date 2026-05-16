# 后端模块

## Commands 层

### db.rs (8 个命令)
**文件**: `src-tauri/src/commands/db.rs`

**核心命令**:
- `init_db()` - 初始化数据库表结构
- `get_all_feeds()` - 获取所有 Feeds
- `get_articles(filter)` - 查询文章（支持分页/筛选）
- `update_article(id, is_read, is_starred)` - 更新文章状态
- `delete_feed(id)` - 删除 Feed（级联删除文章）
- `get_categories()` - 获取分类及未读计数

**风险**:
- `get_articles()` 无 limit 时可能返回大量数据
- `delete_feed()` 级联删除不可逆

### feed.rs (6 个命令)
**文件**: `src-tauri/src/commands/feed.rs`

**核心命令**:
- `add_feed(url, category)` - 添加 Feed
- `fetch_and_update_feed(id)` - 单个 Feed 刷新
- `batch_refresh_feeds()` - 批量刷新（带事件推送）
- `cancel_batch_refresh()` - 取消刷新
- `export_opml()` - 导出 OPML
- `import_opml(content)` - 导入 OPML

**风险**:
- `batch_refresh_feeds()` 长时间运行，需要取消机制
- `import_opml()` 可能插入大量数据

## Core 层

### database.rs
**文件**: `src-tauri/src/core/database.rs`

**职责**: SQLite CRUD 操作

**关键函数**:
- `db_init()` - 创建 3 个表（feeds, articles, fetch_logs）
- `db_insert_articles()` - 批量插入文章（使用 `INSERT OR IGNORE`）
- `db_query_articles()` - 复杂查询（分页、筛选、排序）

**风险**:
- IO 操作，可能阻塞（但 Tauri 自动异步化）
- 大量数据插入时性能问题

### network.rs
**文件**: `src-tauri/src/core/network.rs`

**职责**: HTTP 客户端

**配置**:
- 超时: 15 秒
- 重试: 3 次（指数退避）
- 并发: 最多 3 个

**风险**:
- 网络请求可能失败（需要重试机制）
- 恶意 URL 可能导致安全问题（已验证 URL 格式）

### parser.rs
**文件**: `src-tauri/src/core/parser.rs`

**职责**: RSS/Atom 解析

**特性**:
- 使用 `feed-rs` 库
- 支持 GBK 编码（通过 `encoding_rs`）
- 自动提取标题、链接、内容

**风险**:
- 某些 Feed 格式不规范可能解析失败
- 大量文章解析可能占用 CPU

## 异步任务

### 批量刷新
**位置**: `feed.rs::batch_refresh_feeds`

**实现**:
```rust
for (index, feed) in feeds.iter().enumerate() {
    // 检查取消
    if token.is_cancelled() { return; }

    // 拉取并处理
    let content = network::fetch_feed(&feed.url).await?;
    let articles = parser::parse_feed(&feed.url, &content)?;

    // 插入数据库
    database::db_insert_articles(&app, &articles)?;

    // 推送进度
    app.emit("refresh-progress", json!({
        "type": "progress",
        "current": index + 1,
        "total": total
    }))?;
}
```

**风险**:
- 长时间运行（30s - 5min）
- 需要全局取消令牌（`CancellationToken`）

## 错误处理

**统一错误类型**: `core/error.rs::AppError`

```rust
pub enum AppError {
    DatabaseError(rusqlite::Error),
    NetworkError(reqwest::Error),
    ParseError(String),
}
```

**传播链**: Core Layer → Command Layer (`Result<T, String>`) → Frontend

## IO 风险点

| 操作 | 位置 | 风险 |
|------|------|------|
| 数据库写入 | `database.rs` | 可能阻塞 |
| 网络请求 | `network.rs` | 可能超时/失败 |
| 文件读写 | `commands/db.rs` (备份/恢复) | 大文件风险 |

**无文件系统遍历**: 只操作用户选择的文件
**无 Shell 调用**: 不执行系统命令
