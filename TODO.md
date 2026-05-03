# SCX-RSS 实施清单

本文档基于 `DESIGN.md` 的设计，将开发任务分解为可执行的 TODO 列表。

---

## 📋 阶段 0：环境准备

### 系统环境
- [x] 安装 Rust 1.80+ (`rustup --version`)
- [x] 安装 Node.js 18+ (`node --version`)
- [x] 安装 pnpm (`pnpm --version`)
- [ ] 安装 VS Code 及插件：
  - [ ] Vue - Official
  - [ ] rust-analyzer
  - [ ] Tauri
  - [ ] UnoCSS

### 项目初始化
- [x] 使用 `create-tauri-app` 创建项目
- [x] 配置 UnoCSS
- [x] 配置 TypeScript 路径别名
- [x] 添加 Rust 依赖到 `Cargo.toml`
- [x] 配置 ESLint 和 Prettier
- [x] 设置 .gitignore
- [x] 初始化 Git 仓库
- [x] 运行 `pnpm tauri dev` 确保项目启动成功

---

## 🚀 阶段 1：核心功能（MVP）

### 1.1 后端基础设施

#### 数据库模块 (`src-tauri/src/core/database.rs`)
- [x] 创建 `models.rs` 定义数据结构
  - [x] `Feed` 结构体
  - [x] `Article` 结构体
  - [x] `FetchLog` 结构体
- [x] 实现 `db_init()` 命令
  - [x] 创建 `feeds` 表
  - [x] 创建 `articles` 表
  - [x] 创建 `fetch_logs` 表
  - [x] 创建索引（`feed_id`, `published_at`, `is_read`, `is_starred`）
- [x] 实现 `db_insert_feed()` 命令
- [x] 实现 `db_insert_articles()` 命令（使用事务）
- [x] 实现 `db_query_articles()` 命令（支持分页、筛选）
- [x] 实现 `db_update_article()` 命令
- [x] 实现 `db_delete_feed()` 命令（CASCADE）
- [x] 编写单元测试

#### 网络模块 (`src-tauri/src/core/network.rs`)
- [x] 实现 `fetch_feed()` 函数
  - [x] 设置 User-Agent
  - [x] 超时 15s
  - [x] 自动重试 3 次（指数退避）
  - [x] 支持 HTTP 重定向（最多 5 次）
- [x] 实现 `batch_fetch_feeds()` 函数
  - [x] 并发控制（最多 3 个并发）
  - [x] 请求间隔 100ms
- [x] 编写单元测试（mock HTTP 请求）

#### 解析模块 (`src-tauri/src/core/parser.rs`)
- [x] 实现 `parse_feed()` 函数
  - [x] 支持 RSS 格式
  - [x] 支持 Atom 格式
  - [x] 提取元数据（title、description、icon）
  - [x] 处理编码问题（UTF-8、GBK、ISO-8859-1）
- [x] 实现 `parse_opml()` 函数（基础版本）
- [x] 编写单元测试（使用真实 RSS 样本）

#### 错误处理 (`src-tauri/src/core/error.rs`)
- [x] 定义 `AppError` 枚举
  - [x] `NetworkError`
  - [x] `ParseError`
  - [x] `DatabaseError`
  - [x] `ValidationError`
- [x] 实现错误转换到 `String`（Tauri 要求）

#### Tauri Commands (`src-tauri/src/commands/`)
- [x] 创建 `db.rs` 命令文件
  - [x] 注册所有数据库命令
- [x] 创建 `feed.rs` 命令文件
  - [x] `fetch_and_update_feed(feed_id)`
  - [x] `add_feed(url, category)`
  - [x] `batch_refresh_feeds()`
- [x] 在 `main.rs` 中注册所有命令

---

### 1.2 前端基础设施

#### 类型定义 (`src/types/`)
- [x] 创建 `feed.ts`（Feed 类型定义）
- [x] 创建 `article.ts`（Article 类型定义）
- [x] 创建 `index.ts`（导出所有类型）

#### 工具函数 (`src/utils/`)
- [x] 创建 `formatters.ts`
  - [x] `formatDate()` - 日期格式化
  - [x] `formatFileSize()` - 文件大小格式化
- [x] 创建 `validators.ts`
  - [x] `validateUrl()` - URL 验证
- [x] 创建 `constants.ts`
  - [x] 应用常量定义

#### 组合式函数 (`src/composables/`)
- [x] 创建 `useToast.ts`
  - [x] `showSuccess(message)`
  - [x] `showError(message)`
  - [x] `showInfo(message)`

---

### 1.3 核心组件

#### 布局组件
- [x] 创建 `App.vue`
  - [x] 侧边栏布局
  - [x] 主内容区域
  - [x] 响应式设计
- [x] 创建 `Sidebar.vue`
  - [x] Feed 列表容器
  - [x] 设置入口

#### Feed 管理
- [x] 创建 `FeedList.vue`
  - [x] 显示 Feed 列表
  - [x] 添加 Feed 按钮
  - [x] 删除 Feed 功能
  - [x] 刷新 Feed 按钮
- [x] 创建 `useFeeds.ts`
  - [x] `addFeed(url, category)`
  - [x] `deleteFeed(id)`
  - [x] `refreshFeed(id)`
  - [x] `listFeeds()`

#### 文章展示
- [x] 创建 `ArticleList.vue`
  - [x] 显示文章列表
  - [x] 基础样式（标题、时间、摘要）
- [x] 创建 `ArticleView.vue`
  - [x] 渲染文章内容
  - [x] 显示元数据
  - [x] 标记已读按钮
- [x] 创建 `useArticles.ts`
  - [x] `listArticles(feedId)`
  - [x] `markAsRead(id)`
  - [x] `toggleStar(id)`

---

### 1.4 集成与测试
- [ ] 连接前后端
  - [ ] 测试添加 Feed
  - [ ] 测试刷新 Feed
  - [ ] 测试显示文章列表
  - [ ] 测试查看文章详情
  - [ ] 测试标记已读
- [ ] 端到端测试
  - [ ] 完整用户流程测试

---

## 🎨 阶段 2：用户体验优化

### 2.1 批量刷新
- [x] 创建 `RefreshProgress.vue` 组件
  - [x] 进度条显示
  - [x] 当前刷新的 Feed
  - [x] 成功/失败统计
- [x] 实现 Tauri Events 推送进度
- [x] 添加取消刷新功能

### 2.2 筛选与排序
- [ ] 在 `ArticleList.vue` 添加筛选
  - [ ] 全部/未读/收藏切换
- [ ] 添加排序功能
  - [ ] 按时间排序
  - [ ] 按标题排序
- [ ] 更新 `useArticles.ts` 支持筛选参数

### 2.3 键盘快捷键
- [x] 创建 `useKeyboard.ts`
  - [x] `j/k` - 上下篇
  - [x] `r` - 标记已读
  - [x] `s` - 收藏
- [x] 在 `ArticleList.vue` 中集成快捷键

### 2.4 错误提示
- [x] 实现 Toast 通知组件
  - [x] 创建 ToastContainer.vue 组件
  - [x] 在 App.vue 中集成 Toast 容器
  - [x] 支持成功、错误、信息、警告四种类型
  - [x] 自动关闭和手动关闭功能
  - [x] 优雅的进入/退出动画
- [x] 在所有操作中添加错误处理
  - [x] 网络错误提示（添加 Feed、刷新 Feed）
  - [x] 解析错误提示（URL 验证失败）
  - [x] 数据库错误提示（删除 Feed、更新文章状态）
  - [x] 批量刷新错误提示（取消刷新失败）

---

## 📦 阶段 3：数据管理

### 3.1 OPML 导入/导出
- [x] 后端实现
  - [x] `opml_export()` 命令
  - [x] `opml_import()` 命令
  - [x] 解析 OPML XML
- [x] 前端实现
  - [x] 创建 `useOpml.ts`
  - [x] 在 `Settings.vue` 添加导入/导出按钮
  - [x] 文件选择器
  - [x] 导入结果显示

### 3.2 数据备份/恢复
- [x] 实现数据库备份功能
  - [x] 手动触发备份
  - [x] 备份文件命名（带时间戳）
- [x] 实现数据恢复功能
  - [x] 从备份文件恢复
  - [x] 恢复前确认对话框

### 3.3 分类管理
- [x] 创建 `CategoryList.vue`
  - [x] 显示分类列表
  - [x] 折叠/展开
  - [x] 未读计数
- [x] 在添加/编辑 Feed 时支持选择分类
- [x] 集成到 Sidebar 显示分类列表

---

## 🚀 阶段 4：高级功能（未来）

### 4.1 全文搜索
- [ ] 数据库 FTS5 索引
- [ ] 搜索框 UI
- [ ] 搜索结果高亮

### 4.2 自动刷新
- [x] 定时任务调度
- [x] 用户配置刷新间隔
- [x] 后台静默刷新

### 4.3 全文提取
- [ ] 集成 `readability` crate
- [ ] 自动提取网页正文
- [ ] 缓存提取结果

### 4.4 订阅源发现
- [ ] 从网页 URL 发现 RSS Feed
- [ ] 解析 `<link rel="alternate">`
- [ ] 自动填充 Feed URL

### 4.5 主题切换
- [ ] 浅色/深色主题
- [ ] 使用 `@tauri-apps/plugin-store` 保存偏好
- [ ] 主题切换动画

---

## 🧪 测试清单

### 单元测试
- [ ] Rust 模块测试 (`cargo test`)
  - [ ] 数据库操作测试
  - [ ] 网络请求测试（mock）
  - [ ] 解析器测试（使用样本文件）
- [ ] 前端工具函数测试 (Vitest)
  - [ ] 格式化函数
  - [ ] 验证函数

### 集成测试
- [ ] Feed 拉取流程
- [ ] OPML 导入/导出
- [ ] 数据库事务回滚

### E2E 测试（可选）
- [ ] 完整用户流程
- [ ] 跨平台兼容性测试

---

## 📝 发布检查清单

### 代码质量
- [ ] 运行 `cargo fmt` 格式化 Rust 代码
- [ ] 运行 `cargo clippy` 检查 Rust 代码
- [ ] 运行 ESLint 检查 TypeScript 代码
- [ ] 所有测试通过

### 文档
- [ ] 更新 README.md
  - [ ] 功能介绍
  - [ ] 安装说明
  - [ ] 使用指南
- [ ] 更新 CHANGELOG.md
- [ ] 创建 API.md（如需要）

### 构建与发布
- [ ] 构建生产版本 (`pnpm tauri build`)
- [ ] 测试安装包
- [ ] 创建 GitHub Release
- [ ] 上传构建产物

---

## 💡 开发建议

### 优先级
1. **先完成后端**：数据库、网络、解析模块
2. **再开发前端**：从基础组件开始，逐步完善
3. **持续集成**：每完成一个模块就集成测试

### 注意事项
- **不要过度优化**：先保证功能可用，再优化性能
- **及时提交**：每个小功能完成后提交代码
- **编写测试**：核心模块必须有测试覆盖
- **保存样本**：收集各种 RSS/Atom 样本用于测试

### 调试技巧
- 使用 `tracing` 查看后端日志
- 使用 Tauri DevTools 查看前端状态
- 使用 SQLite Browser 查看数据库
- 保存失败请求的 URL 用于调试

---

## 📊 进度跟踪

当前阶段：`阶段 3 - 数据管理`

总体进度：`████████░░ 60%`

- 阶段 0：`██████████ 100%` ✅
- 阶段 1：`██████████ 100%` (1.1 ✅ | 1.2 ✅ | 1.3 ✅ | 1.4 待集成)
- 阶段 2：`███████░░░ 50%` (2.1 ✅ | 2.2 待开发 | 2.3 待开发 | 2.4 ✅)
- 阶段 3：`█████░░░░░ 50%` (3.1 ✅ | 3.2 待开发 | 3.3 ✅)
- 阶段 4：`░░░░░░░░░░ 0%`
