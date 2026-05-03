# 架构设计

最后更新：2026-05-03

## 目录

- [整体架构](#整体架构)
- [目录结构](#目录结构)
- [技术分层](#技术分层)
- [数据流](#数据流)
- [模块依赖](#模块依赖)

## 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                     │
│  ┌──────────────────────┐  ┌──────────────────────────┐ │
│  │   Frontend (Vue 3)   │  │   Backend (Rust)         │ │
│  │                      │  │                          │ │
│  │  ┌────────────────┐  │  │  ┌────────────────────┐ │ │
│  │  │  Components    │  │  │  │  Tauri Commands    │ │ │
│  │  │  - App.vue     │  │◄─┼──┤  - db_*            │ │ │
│  │  │  - FeedList    │  │  │  │  - fetch_feed      │ │ │
│  │  │  - ArticleList │  │  │  │  - parse_feed      │ │ │
│  │  │  - ArticleView │  │  │  │  - opml_*          │ │ │
│  │  └────────────────┘  │  │  └────────────────────┘ │ │
│  │                      │  │                          │ │
│  │  ┌────────────────┐  │  │  ┌────────────────────┐ │ │
│  │  │  Composables   │  │  │  │  Core Modules      │ │ │
│  │  │  - useFeeds    │  │  │  │  - Network         │ │ │
│  │  │  - useArticles │  │  │  │  - Parser          │ │ │
│  │  │  - useAutoRefresh│ │  │  │  - Database        │ │ │
│  │  │  - useOpml     │  │  │  └────────────────────┘ │ │
│  │  └────────────────┘  │  │                          │ │
│  └──────────────────────┘  └──────────────────────────┘ │
└────────────────────────────┼─────────────────────────────┘
                             │
                    ┌────────▼─────────┐
                    │  SQLite Database │
                    │  - feeds         │
                    │  - articles      │
                    │  - fetch_logs    │
                    └──────────────────┘
```

应用采用 **Tauri v2** 桌面应用架构，前端通过 `invoke()` 调用 Rust 后端命令，后端操作 SQLite 数据库。

## 目录结构

```
scx-rss/
├── src/                        # Vue 前端
│   ├── components/             # Vue 组件
│   │   ├── App.vue             # 主布局（三栏）
│   │   ├── Sidebar.vue         # 侧边栏容器
│   │   ├── FeedList.vue        # Feed 列表
│   │   ├── CategoryList.vue    # 分类列表
│   │   ├── ArticleList.vue     # 文章列表
│   │   ├── ArticleView.vue     # 文章阅读器
│   │   ├── Settings.vue        # 设置面板
│   │   ├── RefreshProgress.vue # 刷新进度
│   │   └── ToastContainer.vue  # Toast 通知
│   ├── composables/            # 组合式函数
│   │   ├── useFeeds.ts         # Feed 操作
│   │   ├── useArticles.ts      # 文章操作
│   │   ├── useOpml.ts          # OPML 导入/导出
│   │   ├── useCategories.ts    # 分类操作
│   │   ├── useAutoRefresh.ts   # 自动刷新定时器
│   │   └── useToast.ts         # 通知系统
│   ├── types/                  # 类型定义
│   │   ├── feed.ts
│   │   ├── article.ts
│   │   ├── category.ts
│   │   └── index.ts
│   ├── utils/                  # 工具函数
│   │   ├── formatters.ts
│   │   ├── validators.ts
│   │   └── constants.ts
│   ├── styles/
│   │   └── theme.css           # 基础样式（Vuetify 处理组件样式）
│   └── main.ts                 # 应用入口
│
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── main.rs             # Tauri 入口，注册命令
│   │   ├── commands/           # Tauri 命令层
│   │   │   ├── db.rs           # 数据库命令
│   │   │   └── feed.rs         # Feed/OPML 命令
│   │   └── core/               # 核心业务模块
│   │       ├── database.rs     # 数据库操作
│   │       ├── network.rs      # HTTP 请求
│   │       ├── parser.rs       # RSS/Atom 解析
│   │       └── error.rs        # 错误类型
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── .wiki/                      # 项目 Wiki 文档
├── DESIGN.md                   # 原始设计文档
├── TODO.md                     # 开发任务清单
└── package.json
```

## 技术分层

### 前端分层

```
┌─────────────────────────────────────┐
│  Presentation Layer (Components)    │  ← UI 展示（Vue SFC）
├─────────────────────────────────────┤
│  Business Logic Layer (Composables) │  ← 业务逻辑（组合式函数）
├─────────────────────────────────────┤
│  Data Access Layer (Tauri Invoke)   │  ← 数据访问（IPC 调用）
└─────────────────────────────────────┘
```

### 后端分层

```
┌─────────────────────────────────────┐
│  Command Layer (Tauri Commands)     │  ← IPC 命令接口
├─────────────────────────────────────┤
│  Service Layer (Core Modules)       │  ← 业务服务
│  - Network, Parser, Database        │
├─────────────────────────────────────┤
│  Data Layer (SQLite)                │  ← 数据持久化
└─────────────────────────────────────┘
```

## 数据流

### 拉取 Feed

```
用户操作 → 前端事件 → invoke('fetch_and_update_feed', { feedId })
    → Rust Command 接收
    → Network Module (HTTP GET, 超时15s, 重试3次)
    → Parser Module (解析 RSS/Atom XML)
    → Database Module (事务批量插入)
    → 返回结果到前端
    → 更新 UI 状态
```

### 批量刷新

```
用户点击"全部刷新"
    → invoke('batch_refresh_feeds')
    → 获取所有 Feed URL
    → 并发控制（最多 3 并发，间隔 100ms）
    → 每个 Feed 执行完整流程
    → 通过 Tauri Events 实时推送进度
    → 前端 RefreshProgress 组件显示进度
```

### OPML 导入

```
用户选择 OPML 文件 → FileReader 读取
    → invoke('opml_import', { content })
    → 解析 OPML XML → 提取 outline 元素
    → 批量插入数据库（去重）
    → 返回统计结果 { total, imported, skipped, failed }
    → 前端刷新 Feed 列表
```

## 模块依赖

### 前端依赖链

```
ArticleView.vue → useArticles.ts → useFeeds.ts → invoke()
FeedList.vue    → useFeeds.ts → invoke()
Settings.vue   → useOpml.ts + useAutoRefresh.ts → invoke()
Sidebar.vue    → useFeeds.ts + useCategories.ts (监听 feeds-refreshed 事件刷新列表)
App.vue        → useToast.ts (全局通知)
```

### 后端依赖链

```
commands/feed.rs → core/network.rs + core/parser.rs + core/database.rs
commands/db.rs   → core/database.rs
所有模块          → models.rs (数据结构) + error.rs (错误类型)
```
