一个基于 Tauri v2 + Vue 3 + UnoCSS 的个人 RSS 阅读器设计方案如下：

## 一、总体目标
构建一个**单用户、本地优先**的桌面 RSS 阅读器，支持手动订阅、按需拉取、本地存储与阅读管理。数据完全存储在本地，支持 OPML 格式的**导入/导出**，便于订阅源备份和迁移。

## 二、项目架构设计

### 1. 整体架构
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
│  │  │  - useOpml     │  │  │  │  - Database        │ │ │
│  │  └────────────────┘  │  │  └────────────────────┘ │ │
│  └──────────────────────┘  └──────────────────────────┘ │
│                            │                             │
└────────────────────────────┼─────────────────────────────┘
                             │
                    ┌────────▼─────────┐
                    │  SQLite Database │
                    │  - feeds         │
                    │  - articles      │
                    │  - fetch_logs    │
                    └──────────────────┘
```

### 2. 目录结构
```
scx-rss/
├── src-tauri/                 # Rust 后端
│   ├── src/
│   │   ├── main.rs            # Tauri 入口，注册命令
│   │   ├── commands/          # Tauri 命令
│   │   │   ├── mod.rs
│   │   │   ├── db.rs          # 数据库命令
│   │   │   ├── feed.rs        # Feed 拉取和解析
│   │   │   └── opml.rs        # OPML 导入/导出
│   │   ├── core/              # 核心模块
│   │   │   ├── mod.rs
│   │   │   ├── network.rs     # HTTP 请求客户端
│   │   │   ├── parser.rs      # RSS/Atom 解析器
│   │   │   ├── database.rs    # 数据库操作
│   │   │   └── error.rs       # 错误类型定义
│   │   └── models.rs          # 数据模型（Feed、Article 等）
│   ├── Cargo.toml             # Rust 依赖配置
│   ├── tauri.conf.json        # Tauri 配置
│   └── build.rs               # 构建脚本（可选）
│
├── src/                       # Vue 前端
│   ├── components/            # Vue 组件
│   │   ├── App.vue
│   │   ├── Sidebar.vue
│   │   ├── FeedList.vue
│   │   ├── CategoryList.vue
│   │   ├── ArticleList.vue
│   │   ├── ArticleView.vue
│   │   ├── Settings.vue
│   │   └── RefreshProgress.vue
│   │
│   ├── composables/           # 组合式函数
│   │   ├── useFeeds.ts
│   │   ├── useArticles.ts
│   │   ├── useOpml.ts
│   │   ├── useToast.ts
│   │   └── useKeyboard.ts
│   │
│   ├── stores/                # Pinia 状态管理（可选）
│   │   ├── feeds.ts
│   │   ├── articles.ts
│   │   ├── ui.ts
│   │   └── settings.ts
│   │
│   ├── types/                 # TypeScript 类型定义
│   │   ├── feed.ts
│   │   ├── article.ts
│   │   └── index.ts
│   │
│   ├── utils/                 # 工具函数
│   │   ├── formatters.ts      # 格式化函数（日期、文件大小等）
│   │   ├── validators.ts      # 验证函数
│   │   └── constants.ts       # 常量定义
│   │
│   ├── assets/                # 静态资源
│   │   └── styles/            # 全局样式
│   │       └── main.css
│   │
│   ├── App.tsx                # Vue 入口
│   ├── main.ts                # 应用启动
│   └── vite-env.d.ts          # Vite 类型声明
│
├── public/                    # 公共静态资源
│
├── tests/                     # 测试文件
│   ├── unit/                  # 单元测试
│   └── integration/           # 集成测试
│
├── docs/                      # 文档
│   ├── DESIGN.md              # 设计文档（本文件）
│   ├── API.md                 # API 文档
│   └── CHANGELOG.md           # 变更日志
│
├── .gitignore
├── package.json               # Node.js 依赖
├── vite.config.ts             # Vite 配置
├── tsconfig.json              # TypeScript 配置
├── unocss.config.ts           # UnoCSS 配置
└── README.md                  # 项目说明
```

### 3. 技术分层

**前端层次**：
```
┌─────────────────────────────────────┐
│  Presentation Layer (Components)    │  ← UI 展示
├─────────────────────────────────────┤
│  Business Logic Layer (Composables) │  ← 业务逻辑
├─────────────────────────────────────┤
│  Data Access Layer (Tauri Invoke)   │  ← 数据访问
└─────────────────────────────────────┘
```

**后端层次**：
```
┌─────────────────────────────────────┐
│  Command Layer (Tauri Commands)     │  ← 命令接口
├─────────────────────────────────────┤
│  Service Layer (Core Modules)       │  ← 业务服务
│  - Network, Parser, Database        │
├─────────────────────────────────────┤
│  Data Layer (SQLite)                │  ← 数据持久化
└─────────────────────────────────────┘
```

### 4. 数据流

**拉取 Feed 流程**：
```
用户操作 → 前端事件处理 → Tauri.invoke()
    ↓
Rust Command 接收
    ↓
Network Module (HTTP 请求 + 重试)
    ↓
Parser Module (解析 XML/Atom)
    ↓
Database Module (事务插入)
    ↓
返回结果到前端
    ↓
更新 UI 状态
```

**OPML 导入流程**：
```
用户选择文件 → FileReader 读取内容
    ↓
Tauri.invoke('opml_import', { content })
    ↓
解析 OPML XML
    ↓
批量验证 URL
    ↓
并发拉取 Feed 元数据
    ↓
批量插入数据库（去重）
    ↓
返回统计结果
    ↓
更新 Feed 列表
```

### 5. 模块依赖关系

**前端依赖**：
```
ArticleView.vue
    ↓ 使用
useArticles.ts (Composable)
    ↓ 调用
useFeeds.ts (Composable)
    ↓ 调用
Tauri Commands (invoke)
```

**后端依赖**：
```
commands/feed.rs
    ↓ 依赖
core/network.rs
core/parser.rs
core/database.rs
    ↓ 依赖
models.rs (数据结构)
```

## 三、前置任务与开发准备

### 1. 环境准备

**系统要求**：
- macOS 12+ / Windows 10+ / Linux (Ubuntu 20.04+)
- Node.js 18+ / 20+
- Rust 1.80+ （使用 `rustup` 安装）
- npm / pnpm / yarn （推荐 pnpm）

**开发工具**：
- VS Code + 插件：
  - `Vue - Official` (Vue 语言支持)
  - `rust-analyzer` (Rust 语言支持)
  - `Tauri` (Tauri 支持)
  - `UnoCSS` (原子化 CSS 提示)

### 2. 初始化项目

**步骤 1：创建 Tauri 项目**
```bash
# 使用 create-tauri-app 创建项目
npm create tauri-app@latest

# 交互式选择：
# - Project name: scx-rss
# - Frontend framework: Vue
# - Use TypeScript: Yes
# - Package manager: pnpm
# - UI template: UnoCSS (如果没有，后续手动配置)
```

**步骤 2：配置 UnoCSS**
```bash
# 安装 UnoCSS
pnpm add -D unocss unocss-preset-wind

# 创建 unocss.config.ts
```

**步骤 3：配置 TypeScript**
```json
// tsconfig.json 添加路径别名
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
```

**步骤 4：添加 Rust 依赖**
```toml
# src-tauri/Cargo.toml

[dependencies]
tauri = { version = "2", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "cookies"] }
feed = "0.5"
rusqlite = { version = "0.32", features = ["bundled"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
opml = "1.1"
anyhow = "1"  # 错误处理
thiserror = "1"  # 自定义错误类型
url = "2"  # URL 验证
chrono = "0.4"  # 时间处理
```

### 3. 开发规范

**代码风格**：
- Rust：使用 `cargo fmt` 格式化，`cargo clippy` 检查
- TypeScript/Vue：使用 ESLint + Prettier
- Git 提交：使用 Conventional Commits 规范

**测试策略**：
- 单元测试：Rust 模块测试（`cargo test`）
- 集成测试：前端组件测试（Vitest）
- E2E 测试：使用 Tauri API 测试（可选）

**版本控制**：
- 主分支：`main`
- 开发分支：`develop`
- 功能分支：`feature/xxx`
- 修复分支：`fix/xxx`

### 4. 开发流程检查清单

在开始编码前，确保完成以下任务：

- [ ] 安装 Rust 1.80+ (`rustup --version`)
- [ ] 安装 Node.js 18+ (`node --version`)
- [ ] 安装 pnpm (`pnpm --version`)
- [ ] 创建 Tauri 项目并运行成功 (`pnpm tauri dev`)
- [ ] 配置 UnoCSS 并测试样式
- [ ] 配置 TypeScript 路径别名
- [ ] 添加所有 Rust 依赖并编译通过
- [ ] 设置 Git 仓库和 .gitignore
- [ ] 配置 ESLint 和 Prettier
- [ ] （可选）配置 GitHub Actions CI/CD

## 四、技术栈
- **框架**：Tauri v2.1+（Rust 1.80+）
- **前端**：Vue 3.5（Composition API + `<script setup>`） + TypeScript 5.6+
- **样式**：UnoCSS（原子化 CSS，预设 `presetWind`）
- **构建**：Vite 6
- **状态管理**：Pinia 2.2+（可选，根据项目复杂度决定）
- **数据存储**：SQLite（通过 `rusqlite` 集成）
- **HTTP 客户端**：`reqwest` 0.12（支持重试、超时、cookies）
- **RSS 解析**：`feed` 0.5（支持 RSS、Atom、JSON Feed）
- **日志**：`tracing` 0.1 + `tracing-appender` 0.2
- **OPML 解析**：`opml` 1.1 或自实现简单解析器
- **项目模板**：采用 `create-tauri-app` 生成的官方 Vue + TypeScript 模板

## 五、核心模块设计

### 1. 前端（Vue 3）

   **组件结构**：
   - `App.vue`：主布局，包含侧边栏和主内容区
     - 支持响应式布局（移动端侧边栏可折叠）
     - 集成全局通知系统（toast/notifications）
   - `Sidebar.vue`：侧边栏容器
     - 显示应用标题和设置入口
     - 包含 `FeedList.vue` 和 `CategoryList.vue`
   - `FeedList.vue`：展示已订阅源
     - 支持按分类分组
     - 显示未读计数徽章
     - 支持右键菜单（刷新、编辑、删除）
     - 拖拽排序（未来版本）
   - `CategoryList.vue`：分类列表
     - 显示每个分类的未读数
     - 支持折叠/展开
   - `ArticleList.vue`：文章列表
     - 支持筛选（全部/未读/收藏）
     - 支持排序（时间/标题）
     - 虚拟滚动（处理大量文章）
     - 预览模式（标题+摘要）/ 紧凑模式（仅标题）
   - `ArticleView.vue`：文章详情页
     - 渲染文章内容（HTML 安全渲染）
     - 显示元数据（发布时间、作者、来源）
     - 工具栏：标记已读/收藏、在浏览器中打开
     - 键盘快捷键支持（j/k 上下篇，r 标记已读，s 收藏）
   - `Settings.vue`：设置页面
     - OPML 导入/导出
     - 主题切换
     - 查看拉取日志
     - 数据备份/恢复
   - `RefreshProgress.vue`：刷新进度组件
     - 显示当前刷新的 Feed
     - 进度条和统计信息
     - 支持取消刷新

   **组合式函数（Composables）**：
   - `useFeeds.ts`：管理 Feed 相关操作
     ```typescript
     const { feeds, categories, loading, error } = useFeeds()
     const { addFeed, updateFeed, deleteFeed, refreshFeed, refreshAll } = useFeedActions()
     ```
   - `useArticles.ts`：管理文章列表和状态
     ```typescript
     const { articles, filters, pagination, loading } = useArticles(feedId)
     const { markAsRead, toggleStar, bulkMarkAsRead } = useArticleActions()
     ```
   - `useOpml.ts`：OPML 导入/导出
     ```typescript
     const { importOpml, exportOpml } = useOpml()
     ```
   - `useToast.ts`：全局通知
     ```typescript
     const { showSuccess, showError, showInfo } = useToast()
     ```
   - `useKeyboard.ts`：键盘快捷键
     ```typescript
     const { registerShortcut, unregisterAll } = useKeyboard()
     ```

   **状态管理**：
   - 使用 `Pinia` 进行全局状态管理（可选，或直接使用 Composables）
   - Stores：
     - `feedsStore`：Feed 列表和分类
     - `articlesStore`：当前文章列表和筛选条件
     - `uiStore`：UI 状态（侧边栏展开/折叠、当前视图等）
     - `settingsStore`：用户偏好设置  

### 2. 后端逻辑（Tauri Command，Rust）

   **网络请求层**：
   - `fetch_feed(url: String) -> Result<FeedData, FetchError>`：
     - 使用 `reqwest` 发起 HTTP GET，超时设为 15s
     - 设置 User-Agent：`"RSS Reader/1.0 (+https://github.com/yourusername/scx-rss)"`
     - 支持自动重试：最多 3 次，间隔递增（1s → 2s → 4s）
     - 遵循 HTTP 重定向（最多 5 次）
     - 返回原始 XML/Atom 字符串及元数据（状态码、响应头）
   - `batch_fetch_feeds(urls: Vec<String>) -> Vec<FetchResult>`：
     - 支持批量拉取，自动并发控制（最多 3 个并发请求）
     - 每个请求间隔 100ms，避免触发服务器速率限制
     - 返回每个 URL 的拉取结果（成功/失败）

   **解析层**：
   - `parse_feed(xml: String, url: String) -> Result<ParsedFeed, ParseError>`：
     - 使用 `rss` 和 `atom_syndication` crate 解析
     - 统一为内部 `Feed` 和 `Article` 结构
     - 自动提取：title、description、icon（从 `<atom:link>` 或 `<image>`）
     - 对解析失败的项记录到 `fetch_logs`，但不中断整体流程
     - 处理常见编码问题（自动检测 UTF-8、GBK、ISO-8859-1）

   **数据库操作**：
   - `db_init()`：创建所有表和索引
   - `db_insert_feed(url: String, title: Option<String>, category: Option<String>) -> i64`
   - `db_update_feed_metadata(id: i64, title: String, description: Option<String>, icon_url: Option<String>)`
   - `db_insert_articles(feed_id: i64, articles: Vec<Article>) -> InsertResult`：
     - 返回插入成功数、跳过数（重复 GUID）、失败数
     - 使用事务确保原子性
   - `db_update_article(id: i64, is_read: Option<bool>, is_starred: Option<bool>)`
   - `db_query_articles(...)`：支持条件查询（feed_id、is_read、is_starred、category）、分页、排序
   - `db_delete_feed(id: i64)`：删除 Feed 及其所有文章（CASCADE）
   - `db_get_feed_stats(id: i64) -> FeedStats`：返回总文章数、未读数、收藏数

   **OPML 导入/导出**：
   - `opml_export() -> Result<String, String>`：
     - 导出所有订阅源为 OPML 格式字符串
     - 包含 title、xmlUrl、htmlUrl、category 等字段
   - `opml_import(opml_content: String) -> Result<ImportResult, String>`：
     - 解析 OPML 文件，提取所有 outline 元素
     - 自动创建分类和订阅源
     - 返回导入成功数、跳过数（已存在）、失败数
     - 使用 `opml` crate 或自实现简单解析器  

### 3. 数据模型（SQLite）
   - `feeds` 表：
     ```sql
     id INTEGER PRIMARY KEY,
     url TEXT NOT NULL UNIQUE,
     title TEXT,
     description TEXT,              -- Feed 描述
     icon_url TEXT,                 -- Feed 图标 URL
     category TEXT,                 -- 分类/文件夹
     last_fetched DATETIME,         -- 最后拉取时间
     fetch_error TEXT,              -- 最后一次错误信息
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
     ```

   - `articles` 表：
     ```sql
     id INTEGER PRIMARY KEY,
     feed_id INTEGER NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
     guid TEXT NOT NULL,            -- RSS/Atom 的 GUID（可能是 URL、数字、字符串）
     title TEXT,
     link TEXT,
     summary TEXT,                  -- 文章摘要
     content TEXT,                  -- 完整内容（可选择存储或仅存链接）
     author TEXT,                   -- 作者
     published_at DATETIME,         -- 发布时间
     is_read BOOLEAN DEFAULT 0,
     is_starred BOOLEAN DEFAULT 0,
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     UNIQUE(feed_id, guid)          -- 联合唯一索引，避免重复
     ```

   - `fetch_logs` 表（可选，用于调试）：
     ```sql
     id INTEGER PRIMARY KEY,
     feed_id INTEGER REFERENCES feeds(id),
     status TEXT NOT NULL,          -- 'success' | 'error'
     error_message TEXT,
     articles_count INTEGER,        -- 本次拉取到的文章数
     new_articles_count INTEGER,    -- 实际入库的新文章数
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP
     ```

   **优化说明**：
   - 添加了 `ON DELETE CASCADE`，删除 Feed 时自动清理文章
   - 添加了 `description`、`icon_url`、`category`、`author` 等元数据字段
   - `guid` 字段保持 TEXT 类型以兼容各种 GUID 格式（URL、uuid、数字等）
   - 添加 `fetch_error` 字段记录拉取失败信息

## 六、关键流程

### 1. 拉取单个 Feed
1. 用户在前端点击"刷新"某 Feed
2. 前端调用 `invoke('fetch_and_update_feed', { feedId })`
3. Rust 端执行：
   - 从数据库获取 Feed 信息（URL、上次拉取时间）
   - 发起 HTTP 请求（自动重试 3 次）
   - 解析 XML/Atom，提取文章列表
   - 使用事务批量插入新文章（忽略重复 GUID）
   - 更新 Feed 的 `last_fetched` 和 `fetch_error` 字段
   - 记录拉取日志到 `fetch_logs` 表
4. 返回结果：`{ success: bool, newCount: number, error?: string }`
5. 前端刷新文章列表，显示提示信息

### 2. 批量刷新所有 Feed
1. 用户点击"全部刷新"按钮
2. 前端调用 `invoke('batch_refresh_feeds')`
3. Rust 端执行：
   - 获取所有 Feed 的 URL 列表
   - 使用并发控制（最多 3 个并发请求）
   - 每个请求间隔 100ms，避免触发速率限制
   - 对每个 Feed 执行完整的"拉取 → 解析 → 入库"流程
   - 实时推送进度到前端（通过 Tauri Events）
4. 前端实时显示刷新进度：
   - 显示当前正在刷新的 Feed
   - 显示进度条（已完成/总数）
   - 显示成功/失败统计
5. 完成后显示汇总信息

### 3. 添加新 Feed
1. 用户在前端输入 Feed URL
2. 前端调用 `invoke('add_feed', { url, category })`
3. Rust 端执行：
   - 验证 URL 格式
   - 尝试拉取 Feed（自动重试）
   - 解析获取元数据（title、description、icon）
   - 插入数据库
   - 自动拉取第一篇文章列表
4. 返回新创建的 Feed ID 和信息
5. 前端刷新 Feed 列表

### 4. OPML 导入
1. 用户选择 OPML 文件或粘贴 OPML 内容
2. 前端调用 `invoke('opml_import', { content })`
3. Rust 端执行：
   - 解析 OPML XML
   - 提取所有 `<outline>` 元素
   - 按分类组织订阅源
   - 批量插入数据库（跳过已存在的 URL）
   - 对新订阅源自动拉取元数据
4. 返回导入统计：`{ total: number, imported: number, skipped: number, failed: number }`
5. 前端显示导入结果，刷新 Feed 列表

### 5. OPML 导出
1. 用户点击"导出 OPML"按钮
2. 前端调用 `invoke('opml_export')`
3. Rust 端执行：
   - 查询所有 Feed（包含分类）
   - 生成标准 OPML XML 格式
   - 返回 OPML 字符串
4. 前端触发文件下载或显示对话框让用户保存

## 七、错误处理与日志策略

### 1. 错误分类
- **网络错误**：超时、连接失败、DNS 解析失败
  - 自动重试 3 次（指数退避）
  - 最终失败时记录到 `feeds.fetch_error` 字段
  - 前端显示友好的错误提示
- **解析错误**：XML 格式错误、缺少必要字段
  - 记录到 `fetch_logs` 表，标记为 `parse_error`
  - 跳过该文章，继续处理其他文章
  - 前端显示"部分文章解析失败"提示
- **数据库错误**：约束冲突、磁盘空间不足
  - 使用事务确保原子性，失败时回滚
  - 记录到应用日志文件
  - 前端显示"数据库错误，请联系开发者"

### 2. 日志策略
- **应用日志**：使用 `tracing` crate，输出到文件
  - 路径：`~/.local/share/scx-rss/logs/app.log`
  - 级别：INFO（默认）、DEBUG（开发模式）
  - 日志轮转：单文件最大 10MB，保留 5 个历史文件
- **拉取日志**：存储在 `fetch_logs` 表
  - 保留最近 1000 条记录
  - 用于调试 Feed 拉取问题
  - 可在前端"设置 → 拉取日志"中查看

### 3. 用户反馈
- 成功操作：显示简洁的提示（如"已更新 3 篇文章"）
- 部分失败：显示详细信息（如"3 个成功，1 个失败"）
- 完全失败：显示错误原因和建议（如"网络超时，请检查网络连接后重试"）
- 长时间操作：显示进度条和取消按钮

## 八、扩展性设计

### 1. 数据管理
- **OPML 导入/导出**：支持与其他 RSS 阅读器互通
- **数据备份**：定期自动备份数据库到 `~/.local/share/scx-rss/backups/`
- **数据恢复**：提供"从备份恢复"功能

### 2. 功能增强（未来版本）
- **全文提取**：在 `parse_feed` 中集成 `readability` crate，自动提取网页正文
- **内容搜索**：使用 `FTS5` 全文搜索索引，支持跨文章搜索
- **智能分类**：基于文章内容自动推荐分类
- **订阅源发现**：从网页 URL 自动发现 RSS Feed（解析 `<link rel="alternate">`）
- **刷新调度**：支持定时自动刷新（如每小时刷新一次）
- **阅读模式**：内置浏览器阅读器，支持标记重点内容

### 3. 配置管理
- 使用 `@tauri-apps/plugin-store` 保存用户偏好：
  - 主题（浅色/深色/自动）
  - 默认视图（全部/未读/收藏）
  - 刷新间隔（手动/15分钟/30分钟/1小时）
  - 文章排序方式（时间/标题）
  - 是否自动标记已读

## 九、性能优化

### 1. 数据库优化
- 创建索引：
  - `CREATE INDEX idx_articles_feed_id ON articles(feed_id)`
  - `CREATE INDEX idx_articles_published ON articles(published_at DESC)`
  - `CREATE INDEX idx_articles_is_read ON articles(is_read)`
  - `CREATE INDEX idx_articles_is_starred ON articles(is_starred)`
- 使用 VACUUM 定期清理数据库（每月一次）
- 考虑使用 `journal_mode=WAL` 提升并发性能

### 2. 前端优化
- 使用虚拟滚动渲染文章列表（`vue-virtual-scroller`）
- 延迟加载文章内容（点击时才加载全文）
- 缓存 Feed 图标和元数据（使用 `localStorage`）
- 防抖处理用户输入（如搜索框）

### 3. 网络优化
- 支持 ETag/Last-Modified，避免重复拉取未更新的内容
- 启用 HTTP 压缩（gzip）
- 缓存 Feed 元数据，减少解析开销

## 十、开发优先级（MVP 路线图）

### Phase 1：核心功能（MVP）
**目标**：实现基本的 RSS 阅读功能
1. 数据库初始化和基础 CRUD
2. 添加/删除 Feed
3. 拉取单个 Feed（手动刷新）
4. 显示文章列表（按 Feed）
5. 查看文章详情
6. 标记已读/未读

### Phase 2：用户体验优化
**目标**：提升易用性
1. 批量刷新所有 Feed
2. 刷新进度显示
3. 筛选（未读/已读/收藏）
4. 文章排序
5. 键盘快捷键
6. 错误提示和日志记录

### Phase 3：数据管理
**目标**：支持数据导入/导出
1. OPML 导入
2. OPML 导出
3. 数据备份
4. 数据恢复
5. Feed 分类管理

### Phase 4：高级功能（未来版本）
1. 全文搜索
2. 自动刷新调度
3. 全文内容提取
4. 订阅源发现
5. 主题切换
6. 内置浏览器阅读器

## 十一、安全性考虑

### 1. 内容安全
- 使用 DOMPurify 清理用户生成的 HTML 内容，防止 XSS 攻击
- 设置 Content Security Policy (CSP)
- 禁用脚本执行和 iframe（除非用户明确信任）

### 2. 网络安全
- 验证 URL 格式，防止 SSRF 攻击
- 限制 HTTP 重定向次数（最多 5 次）
- 设置合理的超时时间（15s）
- 不支持认证（暂不考虑需要登录的私有 Feed）

### 3. 数据安全
- 数据库文件权限设置为用户可读
- 备份文件加密存储（可选）
- 定期提醒用户备份
