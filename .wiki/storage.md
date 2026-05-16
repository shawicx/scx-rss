# 数据存储

## SQLite 数据库

### 存储位置
**路径**: `app_data_dir()/scx_rss.db`

**实际位置**:
- macOS: `~/Library/Application Support/com.scx.rss/scx_rss.db`
- Windows: `%APPDATA%\com.scx.rss\scx_rss.db`
- Linux: `~/.config/com.scx.rss/scx_rss.db`

### 数据库结构

#### feeds 表
```sql
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL UNIQUE,          -- Feed URL
    title TEXT NOT NULL,                -- 标题
    description TEXT,                   -- 描述
    icon_url TEXT,                      -- 图标 URL
    category TEXT,                      -- 分类
    created_at TEXT NOT NULL,           -- 创建时间
    updated_at TEXT NOT NULL,           -- 更新时间
    last_fetched_at TEXT                -- 最后拉取时间
);

-- 索引
CREATE INDEX idx_feeds_category ON feeds(category);
```

#### articles 表
```sql
CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id INTEGER NOT NULL,           -- 外键 → feeds.id
    title TEXT NOT NULL,                -- 标题
    link TEXT NOT NULL,                 -- 文章链接
    content TEXT,                       -- 内容
    description TEXT,                   -- 摘要
    author TEXT,                        -- 作者
    published_at TEXT,                  -- 发布时间
    is_read BOOLEAN DEFAULT 0,          -- 已读状态
    is_starred BOOLEAN DEFAULT 0,       -- 收藏状态
    created_at TEXT NOT NULL,           -- 抓取时间
    FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE,
    UNIQUE(feed_id, link)               -- 去重约束
);

-- 索引
CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_is_read ON articles(is_read);
CREATE INDEX idx_articles_is_starred ON articles(is_starred);
CREATE INDEX idx_articles_published_at ON articles(published_at DESC);
```

#### fetch_logs 表
```sql
CREATE TABLE fetch_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id INTEGER NOT NULL,           -- 外键 → feeds.id
    success BOOLEAN NOT NULL,           -- 是否成功
    error_message TEXT,                 -- 错误信息
    article_count INTEGER DEFAULT 0,    -- 文章数量
    duration_ms INTEGER DEFAULT 0,      -- 耗时（毫秒）
    fetched_at TEXT NOT NULL,           -- 拉取时间
    FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_fetch_logs_feed_id ON fetch_logs(feed_id);
CREATE INDEX idx_fetch_logs_fetched_at ON fetch_logs(fetched_at DESC);
```

## 数据生命周期

### Feed 数据
**创建**: 用户添加 Feed
**更新**:
- 自动刷新（30 分钟）
- 手动刷新
**删除**:
- 用户删除 Feed（级联删除所有文章）

### Article 数据
**创建**: Feed 刷新时插入新文章
**更新**: 用户标记已读/收藏
**删除**:
- Feed 删除时级联删除
- 无自动清理机制

### FetchLog 数据
**创建**: 每次 Feed 拉取后
**删除**: 无（可能需要定期清理）

## 关键约束

### 唯一约束
```sql
UNIQUE(feed_id, link)  -- articles 表
```

**作用**: 防止重复文章

**实现**: `INSERT OR IGNORE INTO articles`

### 外键约束
```sql
FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
```

**作用**: Feed 删除时自动删除关联文章

## 数据完整性

### 去重机制
通过 `(feed_id, link)` 唯一约束自动去重

**失败场景**:
- Feed 没有提供 `link` 元素
- 不同 Feed 有相同文章（允许，因为 `feed_id` 不同）

### 事务支持
```rust
conn.execute("BEGIN TRANSACTION")?;
// 多个 INSERT
conn.execute("COMMIT")?;
```

**当前**: 未使用事务（单条插入）
**风险**: 批量插入时可能部分失败

## 备份与恢复

### 备份
**位置**: 用户选择路径
**方式**: 直接复制数据库文件
**风险**: 可能覆盖现有文件

### 恢复
**方式**: 复制备份文件替换原数据库
**风险**:
- 覆盖当前数据（不可逆）
- 需要重启应用

## 性能优化

### 索引覆盖
- `feed_id`: 加速按 Feed 查询
- `is_read`: 加速未读筛选
- `is_starred`: 加速收藏筛选
- `published_at`: 加速时间排序

### 查询优化
```sql
-- 分页查询
SELECT * FROM articles
WHERE feed_id = ?
ORDER BY published_at DESC
LIMIT ? OFFSET ?;
```

### 插入优化
```rust
// 批量插入
db_insert_articles(&app, &articles)  // 一次插入多条
```

## 风险点

1. **数据库文件损坏**: 无自动备份机制
2. **大数据量**: 长期使用后 articles 表可能很大（10 万+）
3. **并发访问**: SQLite 单连接，无并发问题
4. **磁盘空间**: 无自动清理 fetch_logs
