# 后端模块

最后更新：2026-04-29

## 目录

- [入口 (main.rs)](#入口-mainrs)
- [命令层 (commands/)](#命令层-commands)
- [核心模块 (core/)](#核心模块-core)
- [错误处理](#错误处理)

## 入口 (main.rs)

> `src-tauri/src/main.rs`

### 启动流程

1. 初始化 `tracing` 日志（默认 INFO 级别）
2. 注册 Tauri 插件：`shell`、`dialog`、`fs`
3. 在 `setup` 中调用 `db::init_db()` 初始化数据库
4. 注册所有 IPC 命令到 `invoke_handler`

### 注册的命令

| 命令 | 模块 | 说明 |
|------|------|------|
| `init_db` | db | 初始化数据库（建表、索引） |
| `get_all_feeds` | db | 获取所有 Feed |
| `get_articles` | db | 查询文章（支持筛选） |
| `update_article` | db | 更新文章状态（已读/收藏） |
| `delete_feed` | db | 删除 Feed 及关联文章 |
| `get_categories` | db | 获取分类列表 |
| `add_feed` | feed | 添加新 Feed |
| `fetch_and_update_feed` | feed | 拉取并更新单个 Feed |
| `batch_refresh_feeds` | feed | 批量刷新所有 Feed |
| `cancel_batch_refresh` | feed | 取消批量刷新 |
| `export_opml` | feed | 导出 OPML |
| `import_opml` | feed | 导入 OPML |

## 命令层 (commands/)

> `src-tauri/src/commands/`

### db.rs — 数据库命令

> `src-tauri/src/commands/db.rs`

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `init_db` | app_handle | `()` | 建表、创建索引 |
| `get_all_feeds` | app_handle | `Vec<Feed>` | 返回所有订阅源 |
| `get_articles` | app_handle, feed_id?, filter? | `Vec<Article>` | 条件查询文章 |
| `update_article` | app_handle, id, is_read?, is_starred? | `()` | 更新文章状态 |
| `delete_feed` | app_handle, id | `()` | CASCADE 删除 Feed |
| `get_categories` | app_handle | `Vec<Category>` | 获取分类列表 |

### feed.rs — Feed/OPML 命令

> `src-tauri/src/commands/feed.rs`

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `add_feed` | app_handle, url, category? | `Feed` | 添加并首次拉取 |
| `fetch_and_update_feed` | app_handle, feed_id | `FetchResult` | 拉取解析入库 |
| `batch_refresh_feeds` | app_handle | `BatchResult` | 并发刷新所有 Feed |
| `cancel_batch_refresh` | — | `()` | 取消进行中的刷新 |
| `export_opml` | app_handle | `String` | 导出 OPML XML |
| `import_opml` | app_handle, content | `ImportResult` | 解析并导入 OPML |

## 核心模块 (core/)

> `src-tauri/src/core/`

### database.rs — 数据库操作

> `src-tauri/src/core/database.rs`

使用 `rusqlite` 操作 SQLite，数据库文件存储在 Tauri 应用数据目录。

**核心操作：**

- 初始化表结构（`feeds`、`articles`、`fetch_logs`）
- 创建索引（`feed_id`、`published_at DESC`、`is_read`、`is_starred`）
- Feed CRUD
- 文章批量插入（事务 + GUID 去重）
- 文章状态更新

### network.rs — 网络请求

> `src-tauri/src/core/network.rs`

基于 `reqwest` 的 HTTP 客户端。

**配置：**
- 超时：15 秒
- User-Agent：`RSS Reader/1.0`
- 自动重试：最多 3 次，指数退避（1s → 2s → 4s）
- HTTP 重定向：最多 5 次
- 批量并发控制：最多 3 个并发请求，间隔 100ms

**核心函数：**
- `fetch_feed(url)` — 拉取单个 Feed
- `batch_fetch_feeds(urls)` — 批量拉取

### parser.rs — Feed 解析器

> `src-tauri/src/core/parser.rs`

使用 `feed-rs` crate 解析 RSS/Atom 格式。

**支持的格式：**
- RSS 2.0
- Atom
- JSON Feed（规划中）

**解析逻辑：**
- 提取 Feed 元数据（title、description、icon）
- 提取文章列表（title、link、summary、content、author、published_at）
- 处理编码问题（UTF-8、GBK、ISO-8859-1）
- 解析失败时记录日志但不中断整体流程

### error.rs — 错误类型

> `src-tauri/src/core/error.rs`

使用 `thiserror` 定义结构化错误类型：

```rust
enum AppError {
    NetworkError(String),    // 网络请求失败
    ParseError(String),      // XML/Feed 解析失败
    DatabaseError(String),   // 数据库操作失败
    ValidationError(String), // 数据验证失败
}
```

所有错误最终转换为 `String` 以满足 Tauri 命令的序列化要求。

## 错误处理

### 网络错误

- 自动重试 3 次（指数退避）
- 最终失败记录到 `feeds.fetch_error`
- 前端显示友好错误提示

### 解析错误

- 记录到 `fetch_logs` 表
- 跳过失败项，继续处理
- 前端提示"部分文章解析失败"

### 数据库错误

- 使用事务确保原子性
- 失败时回滚
- 记录到应用日志文件

### 关联文件

- `src-tauri/src/main.rs`
- `src-tauri/src/commands/db.rs`
- `src-tauri/src/commands/feed.rs`
- `src-tauri/src/core/database.rs`
- `src-tauri/src/core/network.rs`
- `src-tauri/src/core/parser.rs`
- `src-tauri/src/core/error.rs`
