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
- [ ] 创建 `models.rs` 定义数据结构
  - [ ] `Feed` 结构体
  - [ ] `Article` 结构体
  - [ ] `FetchLog` 结构体
- [ ] 实现 `db_init()` 命令
  - [ ] 创建 `feeds` 表
  - [ ] 创建 `articles` 表
  - [ ] 创建 `fetch_logs` 表
  - [ ] 创建索引（`feed_id`, `published_at`, `is_read`, `is_starred`）
- [ ] 实现 `db_insert_feed()` 命令
- [ ] 实现 `db_insert_articles()` 命令（使用事务）
- [ ] 实现 `db_query_articles()` 命令（支持分页、筛选）
- [ ] 实现 `db_update_article()` 命令
- [ ] 实现 `db_delete_feed()` 命令（CASCADE）
- [ ] 编写单元测试

#### 网络模块 (`src-tauri/src/core/network.rs`)
- [ ] 实现 `fetch_feed()` 函数
  - [ ] 设置 User-Agent
  - [ ] 超时 15s
  - [ ] 自动重试 3 次（指数退避）
  - [ ] 支持 HTTP 重定向（最多 5 次）
- [ ] 实现 `batch_fetch_feeds()` 函数
  - [ ] 并发控制（最多 3 个并发）
  - [ ] 请求间隔 100ms
- [ ] 编写单元测试（mock HTTP 请求）

#### 解析模块 (`src-tauri/src/core/parser.rs`)
- [ ] 实现 `parse_feed()` 函数
  - [ ] 支持 RSS 格式
  - [ ] 支持 Atom 格式
  - [ ] 提取元数据（title、description、icon）
  - [ ] 处理编码问题（UTF-8、GBK、ISO-8859-1）
- [ ] 实现 `parse_opml()` 函数（基础版本）
- [ ] 编写单元测试（使用真实 RSS 样本）

#### 错误处理 (`src-tauri/src/core/error.rs`)
- [ ] 定义 `AppError` 枚举
  - [ ] `NetworkError`
  - [ ] `ParseError`
  - [ ] `DatabaseError`
  - [ ] `ValidationError`
- [ ] 实现错误转换到 `String`（Tauri 要求）

#### Tauri Commands (`src-tauri/src/commands/`)
- [ ] 创建 `db.rs` 命令文件
  - [ ] 注册所有数据库命令
- [ ] 创建 `feed.rs` 命令文件
  - [ ] `fetch_and_update_feed(feed_id)`
  - [ ] `add_feed(url, category)`
  - [ ] `batch_refresh_feeds()`
- [ ] 在 `main.rs` 中注册所有命令

---

### 1.2 前端基础设施

#### 类型定义 (`src/types/`)
- [ ] 创建 `feed.ts`（Feed 类型定义）
- [ ] 创建 `article.ts`（Article 类型定义）
- [ ] 创建 `index.ts`（导出所有类型）

#### 工具函数 (`src/utils/`)
- [ ] 创建 `formatters.ts`
  - [ ] `formatDate()` - 日期格式化
  - [ ] `formatFileSize()` - 文件大小格式化
- [ ] 创建 `validators.ts`
  - [ ] `validateUrl()` - URL 验证
- [ ] 创建 `constants.ts`
  - [ ] 应用常量定义

#### 组合式函数 (`src/composables/`)
- [ ] 创建 `useToast.ts`
  - [ ] `showSuccess(message)`
  - [ ] `showError(message)`
  - [ ] `showInfo(message)`

---

### 1.3 核心组件

#### 布局组件
- [ ] 创建 `App.vue`
  - [ ] 侧边栏布局
  - [ ] 主内容区域
  - [ ] 响应式设计
- [ ] 创建 `Sidebar.vue`
  - [ ] Feed 列表容器
  - [ ] 设置入口

#### Feed 管理
- [ ] 创建 `FeedList.vue`
  - [ ] 显示 Feed 列表
  - [ ] 添加 Feed 按钮
  - [ ] 删除 Feed 功能
  - [ ] 刷新 Feed 按钮
- [ ] 创建 `useFeeds.ts`
  - [ ] `addFeed(url, category)`
  - [ ] `deleteFeed(id)`
  - [ ] `refreshFeed(id)`
  - [ ] `listFeeds()`

#### 文章展示
- [ ] 创建 `ArticleList.vue`
  - [ ] 显示文章列表
  - [ ] 基础样式（标题、时间、摘要）
- [ ] 创建 `ArticleView.vue`
  - [ ] 渲染文章内容
  - [ ] 显示元数据
  - [ ] 标记已读按钮
- [ ] 创建 `useArticles.ts`
  - [ ] `listArticles(feedId)`
  - [ ] `markAsRead(id)`
  - [ ] `toggleStar(id)`

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
- [ ] 创建 `RefreshProgress.vue` 组件
  - [ ] 进度条显示
  - [ ] 当前刷新的 Feed
  - [ ] 成功/失败统计
- [ ] 实现 Tauri Events 推送进度
- [ ] 添加取消刷新功能

### 2.2 筛选与排序
- [ ] 在 `ArticleList.vue` 添加筛选
  - [ ] 全部/未读/收藏切换
- [ ] 添加排序功能
  - [ ] 按时间排序
  - [ ] 按标题排序
- [ ] 更新 `useArticles.ts` 支持筛选参数

### 2.3 键盘快捷键
- [ ] 创建 `useKeyboard.ts`
  - [ ] `j/k` - 上下篇
  - [ ] `r` - 标记已读
  - [ ] `s` - 收藏
- [ ] 在 `ArticleView.vue` 中集成快捷键

### 2.4 错误提示
- [ ] 实现 Toast 通知组件
- [ ] 在所有操作中添加错误处理
  - [ ] 网络错误提示
  - [ ] 解析错误提示
  - [ ] 数据库错误提示

---

## 📦 阶段 3：数据管理

### 3.1 OPML 导入/导出
- [ ] 后端实现
  - [ ] `opml_export()` 命令
  - [ ] `opml_import()` 命令
  - [ ] 解析 OPML XML
- [ ] 前端实现
  - [ ] 创建 `useOpml.ts`
  - [ ] 在 `Settings.vue` 添加导入/导出按钮
  - [ ] 文件选择器
  - [ ] 导入结果显示

### 3.2 数据备份/恢复
- [ ] 实现数据库备份功能
  - [ ] 手动触发备份
  - [ ] 备份文件命名（带时间戳）
- [ ] 实现数据恢复功能
  - [ ] 从备份文件恢复
  - [ ] 恢复前确认对话框

### 3.3 分类管理
- [ ] 创建 `CategoryList.vue`
  - [ ] 显示分类列表
  - [ ] 折叠/展开
  - [ ] 未读计数
- [ ] 在添加/编辑 Feed 时支持选择分类

---

## 🚀 阶段 4：高级功能（未来）

### 4.1 全文搜索
- [ ] 数据库 FTS5 索引
- [ ] 搜索框 UI
- [ ] 搜索结果高亮

### 4.2 自动刷新
- [ ] 定时任务调度
- [ ] 用户配置刷新间隔
- [ ] 后台静默刷新

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

当前阶段：`阶段 1 - 核心功能（MVP）`

总体进度：`███░░░░░░░░ 10%`

- 阶段 0：`██████████ 100%` ✅
- 阶段 1：`░░░░░░░░░░ 0%`
- 阶段 2：`░░░░░░░░░░ 0%`
- 阶段 3：`░░░░░░░░░░ 0%`
- 阶段 4：`░░░░░░░░░░ 0%`
