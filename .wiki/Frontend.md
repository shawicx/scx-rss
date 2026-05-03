# 前端模块

最后更新：2026-05-03

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

- 主题切换（Material Light / Material Dark / Warm Ink）
- 自动刷新设置（开关、间隔选择、上次刷新时间）
- OPML 导入/导出
- 数据库备份/恢复（带确认对话框）
- 恢复后触发 `data-restored` 事件，由 Sidebar 重新加载数据

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

### useAutoRefresh.ts

> `src/composables/useAutoRefresh.ts`

自动刷新定时器管理：

- `enabled` — 启用状态（ref，synced to localStorage）
- `intervalMinutes` — 刷新间隔（ref，单位分钟）
- `toggleAutoRefresh(value)` — 开关自动刷新
- `setRefreshInterval(minutes)` — 设置刷新间隔
- `formatLastRefreshed()` — 格式化上次刷新时间
- 页面隐藏时暂停计时器，恢复时自动补刷
- 刷新完成后通过 `window.dispatchEvent('feeds-refreshed')` 通知 Sidebar 更新列表

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

### UI 框架：Vuetify 3

> `src/plugins/vuetify.ts`

使用 Vuetify 3 组件库，提供 Material Design 组件和三套主题：

- **Material Light** — 标准浅色主题
- **Material Dark** — 标准深色主题
- **Warm Ink** — 自定义暖色阅读主题

### 基础样式

> `src/styles/theme.css`

最小化的全局样式，Vuetify 处理所有组件样式。保留：
- 全局 reset
- 滚动条样式
- 工具类（`.line-clamp-2`）
- 文章正文排版（`.prose-ink`）

### 关联文件

- `src/App.vue`
- `src/components/*.vue`
- `src/composables/*.ts`
- `src/types/*.ts`
- `src/utils/*.ts`
- `src/styles/theme.css`
- `src/plugins/vuetify.ts`
