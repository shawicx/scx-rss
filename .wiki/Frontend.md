# 前端模块

最后更新：2026-04-29

## 目录

- [组件结构](#组件结构)
- [组合式函数 (Composables)](#组合式函数-composables)
- [类型定义](#类型定义)
- [工具函数](#工具函数)
- [样式系统](#样式系统)

## 组件结构

### App.vue — 主布局

> `src/App.vue`

三栏布局：侧边栏 (w-80) + 文章列表 (w-96) + 文章阅读器 (flex-1)。

- `onMounted` 时调用 `invoke('init_db')` 初始化数据库
- 管理 `currentFeedId` 和 `currentArticle` 状态
- 集成全局 `ToastContainer`

```
┌──────────┬────────────┬──────────────────┐
│ Sidebar  │ ArticleList│  ArticleView     │
│  (w-80)  │  (w-96)    │  (flex-1)        │
│          │            │                  │
│ Category │  文章列表   │   文章内容        │
│ FeedList │  筛选/排序  │   元数据/操作     │
│ Settings │            │                  │
└──────────┴────────────┴──────────────────┘
```

### Sidebar.vue — 侧边栏

> `src/components/Sidebar.vue`

侧边栏容器，包含应用标题、`CategoryList`、`FeedList` 和 `Settings` 入口。

### FeedList.vue — Feed 列表

> `src/components/FeedList.vue`

- 显示已订阅源列表
- 添加/删除 Feed
- 刷新单个 Feed
- 未读计数徽章

### CategoryList.vue — 分类列表

> `src/components/CategoryList.vue`

- 显示分类及其未读数
- 折叠/展开分类
- 点击分类筛选 Feed

### ArticleList.vue — 文章列表

> `src/components/ArticleList.vue`

- 根据 `feedId` 加载文章
- 显示标题、时间、摘要
- 点击选择文章

### ArticleView.vue — 文章阅读器

> `src/components/ArticleView.vue`

- 渲染文章 HTML 内容
- 显示元数据（发布时间、作者、来源）
- 标记已读/收藏操作
- 在浏览器中打开链接

### Settings.vue — 设置面板

> `src/components/Settings.vue`

- OPML 导入/导出
- 主题切换（规划中）

### RefreshProgress.vue — 刷新进度

> `src/components/RefreshProgress.vue`

- 显示批量刷新进度条
- 当前正在刷新的 Feed 名称
- 成功/失败统计
- 取消刷新按钮

### ToastContainer.vue — Toast 通知容器

> `src/components/ToastContainer.vue`

- 支持成功、错误、信息、警告四种类型
- 自动关闭（可配置延迟）
- 手动关闭
- 进入/退出动画

## 组合式函数 (Composables)

### useFeeds.ts

> `src/composables/useFeeds.ts`

Feed 管理，提供以下能力：

- `addFeed(url, category)` — 添加新订阅源
- `deleteFeed(id)` — 删除订阅源
- `refreshFeed(id)` — 刷新单个源
- `refreshAllFeeds()` — 批量刷新
- `listFeeds()` — 获取所有 Feed

### useArticles.ts

> `src/composables/useArticles.ts`

文章管理：

- `listArticles(feedId)` — 获取指定 Feed 的文章
- `markAsRead(id)` — 标记已读
- `toggleStar(id)` — 切换收藏状态

### useOpml.ts

> `src/composables/useOpml.ts`

OPML 导入/导出：

- `importOpml()` — 选择 OPML 文件并导入
- `exportOpml()` — 导出订阅源为 OPML

### useCategories.ts

> `src/composables/useCategories.ts`

分类管理：

- 获取分类列表
- 分类 CRUD 操作

### useToast.ts

> `src/composables/useToast.ts`

全局通知系统：

- `showSuccess(message)` — 成功提示
- `showError(message)` — 错误提示
- `showInfo(message)` — 信息提示
- `showWarning(message)` — 警告提示

## 类型定义

> `src/types/`

### Feed 类型 (`feed.ts`)

```typescript
interface Feed {
  id: number
  url: string
  title: string | null
  description: string | null
  icon_url: string | null
  category: string | null
  last_fetched: string | null
  fetch_error: string | null
  created_at: string
  updated_at: string
}
```

### Article 类型 (`article.ts`)

```typescript
interface Article {
  id: number
  feed_id: number
  guid: string
  title: string | null
  link: string | null
  summary: string | null
  content: string | null
  author: string | null
  published_at: string | null
  is_read: boolean
  is_starred: boolean
  created_at: string
  updated_at: string
}
```

### Category 类型 (`category.ts`)

分类相关类型定义。

### 统一导出 (`index.ts`)

从 `Feed`、`Article`、`Category` 等模块统一导出类型。

## 工具函数

> `src/utils/`

| 文件 | 函数 | 说明 |
|------|------|------|
| `formatters.ts` | `formatDate()` | 日期格式化 |
| `formatters.ts` | `formatFileSize()` | 文件大小格式化 |
| `validators.ts` | `validateUrl()` | URL 格式验证 |
| `constants.ts` | — | 应用常量定义 |

## 样式系统

### 设计语言：暖墨（Warm Ink）

> `src/styles/theme.css`

编辑式设计系统，深色侧边栏 + 暖纸色内容区，以阅读体验为核心。

**配色方案：**

| 角色 | 变量 | 用途 |
|------|------|------|
| 深色背景 | `--ink-dark` (#0f0e11) | 侧边栏 |
| 深色浮层 | `--ink-dark-raised` | 侧边栏悬停/激活 |
| 暖纸色 | `--ink-paper` (#f6f2ec) | 文章列表背景 |
| 亮纸色 | `--ink-paper-bright` (#faf8f4) | 文章阅读器背景 |
| 铜色强调 | `--ink-accent` (#c07a4a) | 按钮、选中态、链接 |
| 正文色 | `--ink-text` (#1a1a1a) | 主要文字 |
| 辅助文字 | `--ink-text-secondary` | 次要信息 |
| 反色文字 | `--ink-text-inverse` | 侧边栏文字 |

**字体：**
- 标题：`'Playfair Display', Georgia, 'Noto Serif SC', serif`
- 正文：`-apple-system, 'PingFang SC', 'Helvetica Neue', sans-serif`

**组件类：**
- `.btn-ink` — 主按钮（铜色）
- `.btn-ghost` — 幽灵按钮（浅色背景）
- `.btn-ghost-dark` — 幽灵按钮（深色背景）
- `.input-ink` — 输入框
- `.modal-overlay` / `.modal-content` — 模态框
- `.prose-ink` — 文章正文排版
- `.badge-ink` — 未读数徽章

### UnoCSS

> `unocss.config.ts`

使用 `presetWind`（Tailwind CSS 兼容）预设，自定义 `ink-*` 色值。

### 关联文件

- `src/App.vue`
- `src/components/*.vue`
- `src/composables/*.ts`
- `src/types/*.ts`
- `src/utils/*.ts`
- `src/styles/theme.css`
- `unocss.config.ts`
