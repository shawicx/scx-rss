# AI Context

> 让 AI 在最少 token 下理解项目

## 核心架构

```
Frontend (Vue 3) → IPC (Tauri) → Backend (Rust) → SQLite
```

- **Frontend**: 9 个 Vuetify 组件，11 个 Composables（含 useI18n/useAutoUpdate）
- **IPC**: 16 个 commands
- **Backend**: 3 个 command 文件（db.rs, feed.rs, system.rs），5 个 core 模块
- **Storage**: 4 个表（feeds, articles, fetch_logs, user_settings）
- **自动更新**: 前端 `useAutoUpdate` 驱动，经 `@tauri-apps/plugin-updater` 检查 GitHub Releases 的 `latest.json`（minisign 验签）；Settings 提供手动检查入口。详见 [backend.md](backend.md#自动更新与签名)。

## 关键文件

| 文件 | 作用 |
|------|------|
| `src-tauri/src/main.rs` | 应用入口，插件初始化 |
| `src-tauri/src/commands/feed.rs` | Feed 拉取、刷新（6 个命令） |
| `src-tauri/src/commands/db.rs` | 数据库 CRUD（10 个命令） |
| `src-tauri/src/core/network.rs` | HTTP 客户端（15s 超时，3 次重试） |
| `src-tauri/src/core/parser.rs` | RSS/Atom 解析 |
| `src-tauri/src/core/database.rs` | SQLite 操作 |
| `src-tauri/src/commands/system.rs` | 系统语言检测 |
| `src/composables/useFeeds.ts` | Feed 业务逻辑 |
| `src/composables/useArticles.ts` | 文章业务逻辑 |
| `src/composables/useI18n.ts` | 国际化管理 |
| `src/i18n.ts` | vue-i18n 配置 |

## 核心调用链

### 添加 Feed
```
useFeeds.addFeed(url)
  → invoke('add_feed')
  → network::fetch_feed(url)
  → parser::parse_feed(content)
  → database::db_insert_feed()
  → database::db_insert_articles()
```

### 批量刷新
```
useFeeds.refreshAllFeeds()
  → invoke('batch_refresh_feeds')
  → for each feed:
      → network::fetch_feed(url)
      → parser::parse_feed(content)
      → database::db_insert_articles()
      → app.emit('refresh-progress')
```

## 模块职责

### Frontend
- **Components**: UI 展示
- **Composables**: 业务逻辑，IPC 封装
- **State**: `ref()` 响应式（无 Vuex/Pinia）

### Backend
- **Commands**: IPC handlers，参数验证
- **Core**: 业务逻辑
  - `database`: SQLite 操作
  - `network`: HTTP 客户端
  - `parser`: RSS/Atom 解析

### IPC
- **正向**: `invoke()` → Command → Core
- **反向**: `app.emit()` → `listen()`

## 数据模型

### Feed
```rust
id, url, title, description, icon_url, category,
created_at, updated_at, last_fetched_at
```

### Article
```rust
id, feed_id, title, link, content, description,
author, published_at, is_read, is_starred, created_at
```

### 唯一约束
- `(feed_id, link)` - 防止重复文章

### user_settings
```rust
key (PK), value, updated_at
```

## 国际化 (i18n)

- **方案**: vue-i18n (前端) + sys-locale (后端系统语言检测)
- **支持语言**: zh-CN（默认）、en
- **持久化**: 语言偏好存储在 `user_settings` 表（key='language'）
- **错误翻译**: Rust 返回错误码（`errors.network`），前端 `$t()` 翻译
- **详细文档**: [architecture.md](architecture.md#国际化-i18n)

## 性能特性

| 操作 | 耗时 |
|------|------|
| 获取 Feeds | ~10ms |
| 查询文章 | ~50ms |
| 添加 Feed | ~2s |
| 批量刷新 | ~60s |

## 关键风险

1. **循环 IPC**: 使用 `batch_*` 命令
2. **大数据**: 使用分页（limit/offset）
3. **长时间运行**: 支持取消
4. **内存泄漏**: 清理事件监听

## 开发规范

1. **IPC 调用**: 统一在 Composables 中封装
2. **状态管理**: 使用 `ref()`，无全局状态库
3. **错误处理**: 统一返回 `Result<T, String>`
4. **Git 提交**: Conventional Commits

## 重要约定

- 禁止擅自提交代码
- Wiki 驱动开发（代码变更后更新 Wiki）
- 安全优先（输入验证、参数化查询）
- 性能优先（批量操作、分页加载）

## 快速导航

- **添加功能**: [ipc.md](ipc.md) (Command 映射)
- **修复 bug**: [risks.md](risks.md) (风险分析)
- **理解架构**: [architecture.md](architecture.md) (数据流)
- **调用关系**: [ipc.md](ipc.md) (调用链)

## Token 优化

优先阅读：
1. [ai-context.md](./ai-context.md) (本文件)
2. [architecture.md](./architecture.md)
3. [ipc.md](./ipc.md)
4. [risks.md](./risks.md)
