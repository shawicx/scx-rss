# 数据模型

最后更新：2026-04-29

## 目录

- [数据库概览](#数据库概览)
- [feeds 表](#feeds-表)
- [articles 表](#articles-表)
- [fetch_logs 表](#fetch_logs-表)
- [索引策略](#索引策略)
- [Rust 数据结构](#rust-数据结构)

## 数据库概览

- **引擎**：SQLite（通过 `rusqlite` bundled 模式集成）
- **存储位置**：Tauri 应用数据目录
- **初始化**：`src-tauri/src/core/database.rs` 中的 `init_db()` 函数
- **访问方式**：通过 Tauri 命令层，前端不直接操作数据库

## feeds 表

存储 RSS/Atom 订阅源信息。

```sql
CREATE TABLE feeds (
    id            INTEGER PRIMARY KEY,
    url           TEXT NOT NULL UNIQUE,
    title         TEXT,
    description   TEXT,              -- Feed 描述
    icon_url      TEXT,              -- Feed 图标 URL
    category      TEXT,              -- 分类/文件夹
    last_fetched  DATETIME,         -- 最后拉取时间
    fetch_error   TEXT,              -- 最后一次错误信息
    created_at    DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | INTEGER PK | 自增主键 |
| `url` | TEXT UNIQUE | Feed URL，唯一约束 |
| `title` | TEXT | Feed 标题（从 XML 解析） |
| `description` | TEXT | Feed 描述 |
| `icon_url` | TEXT | 图标链接 |
| `category` | TEXT | 所属分类名称 |
| `last_fetched` | DATETIME | 最后成功拉取时间 |
| `fetch_error` | TEXT | 最后拉取的错误信息 |
| `created_at` | DATETIME | 创建时间 |
| `updated_at` | DATETIME | 更新时间 |

## articles 表

存储文章内容，通过 `feed_id` 关联 `feeds` 表。

```sql
CREATE TABLE articles (
    id            INTEGER PRIMARY KEY,
    feed_id       INTEGER NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    guid          TEXT NOT NULL,            -- RSS/Atom GUID
    title         TEXT,
    link          TEXT,
    summary       TEXT,                     -- 文章摘要
    content       TEXT,                     -- 完整 HTML 内容
    author        TEXT,
    published_at  DATETIME,                -- 发布时间
    is_read       BOOLEAN DEFAULT 0,
    is_starred    BOOLEAN DEFAULT 0,
    created_at    DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(feed_id, guid)                  -- 联合唯一，防止重复
);
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | INTEGER PK | 自增主键 |
| `feed_id` | INTEGER FK | 关联 feeds.id，CASCADE 删除 |
| `guid` | TEXT | 文章唯一标识（URL/UUID/数字） |
| `title` | TEXT | 文章标题 |
| `link` | TEXT | 原文链接 |
| `summary` | TEXT | 摘要文本 |
| `content` | TEXT | 完整 HTML 内容 |
| `author` | TEXT | 作者名 |
| `published_at` | DATETIME | 发布时间 |
| `is_read` | BOOLEAN | 已读标记 |
| `is_starred` | BOOLEAN | 收藏标记 |
| `UNIQUE(feed_id, guid)` | — | 联合唯一约束，去重依据 |

## fetch_logs 表

记录 Feed 拉取日志，用于调试和监控。

```sql
CREATE TABLE fetch_logs (
    id                 INTEGER PRIMARY KEY,
    feed_id            INTEGER REFERENCES feeds(id),
    status             TEXT NOT NULL,          -- 'success' | 'error'
    error_message      TEXT,
    articles_count     INTEGER,               -- 本次拉取文章数
    new_articles_count INTEGER,               -- 实际入库新文章数
    created_at         DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | INTEGER PK | 自增主键 |
| `feed_id` | INTEGER FK | 关联 feeds.id |
| `status` | TEXT | `success` 或 `error` |
| `error_message` | TEXT | 错误详情 |
| `articles_count` | INTEGER | 拉取到的文章总数 |
| `new_articles_count` | INTEGER | 新增入库的文章数 |
| `created_at` | DATETIME | 日志创建时间 |

## 索引策略

```sql
-- 文章查询优化
CREATE INDEX idx_articles_feed_id    ON articles(feed_id);
CREATE INDEX idx_articles_published  ON articles(published_at DESC);
CREATE INDEX idx_articles_is_read    ON articles(is_read);
CREATE INDEX idx_articles_is_starred ON articles(is_starred);
```

| 索引 | 用途 |
|------|------|
| `idx_articles_feed_id` | 按 Feed 查询文章 |
| `idx_articles_published` | 按时间排序文章列表 |
| `idx_articles_is_read` | 筛选未读文章 |
| `idx_articles_is_starred` | 筛选收藏文章 |

## Rust 数据结构

> `src-tauri/src/models.rs`（概念路径，实际在 core/ 中定义）

### Feed

```rust
struct Feed {
    id: i64,
    url: String,
    title: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    category: Option<String>,
    last_fetched: Option<String>,
    fetch_error: Option<String>,
    created_at: String,
    updated_at: String,
}
```

### Article

```rust
struct Article {
    id: i64,
    feed_id: i64,
    guid: String,
    title: Option<String>,
    link: Option<String>,
    summary: Option<String>,
    content: Option<String>,
    author: Option<String>,
    published_at: Option<String>,
    is_read: bool,
    is_starred: bool,
    created_at: String,
    updated_at: String,
}
```

### 关联文件

- `src-tauri/src/core/database.rs`
- `src-tauri/src/commands/db.rs`
- `src/types/feed.ts`
- `src/types/article.ts`
- `src/types/category.ts`
