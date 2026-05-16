# 前端模块

## 页面结构

```
App.vue
├── Sidebar (左侧边栏)
│   ├── FeedList.vue         # Feed 列表
│   ├── CategoryList.vue     # 分类列表
│   └── Settings.vue         # 设置按钮
├── ArticleList.vue          # 文章列表
├── ArticleView.vue          # 文章阅读器
├── ToastContainer.vue       # Toast 通知
└── RefreshProgress.vue      # 刷新进度条
```

**无 Router**: 使用组件切换，非 SPA 路由

## 关键 Composables

### useFeeds.ts
**作用**: Feed 管理业务逻辑

**核心方法**:
- `getAllFeeds()` - 获取所有 Feeds
- `addFeed(url, category)` - 添加 Feed
- `refreshFeed(id)` - 单个刷新
- `refreshAllFeeds()` - 批量刷新
- `onRefreshProgress(callback)` - 监听进度事件

**风险**:
- `refreshAllFeeds()` 可能长时间运行（30s - 5min）
- 需要支持取消操作

### useArticles.ts
**作用**: 文章管理业务逻辑

**核心方法**:
- `getArticles(filter)` - 查询文章（支持分页）
- `markAsRead(id)` - 标记已读
- `toggleStar(id)` - 切换收藏

**风险**:
- `getArticles()` 可能返回大量数据，必须使用 `limit` 参数

### useOpml.ts
**作用**: OPML 导入/导出

**核心方法**:
- `exportOpml()` - 导出 OPML
- `importOpml(content)` - 导入 OPML

**风险**:
- 导入可能插入大量数据，需要进度提示

## IPC 封装位置

所有 IPC 调用封装在 `src/composables/` 中：

```typescript
import { invoke } from '@tauri-apps/api/core'

const feeds = await invoke<Feed[]>('get_all_feeds')
```

**不在组件中直接调用 `invoke()`**，统一通过 Composables

## 关键业务逻辑

### 文章去重
通过 `(feed_id, link)` 唯一约束自动去重，前端无需处理

### 自动刷新
`useAutoRefresh.ts` 每 30 分钟自动调用 `refreshAllFeeds()`

### 响应式布局
`useResizable.ts` 实现三栏布局拖拽调整宽度

## 容易改崩的地方

1. **ArticleList.vue**: 滚动加载逻辑（offset/limit 计算）
2. **RefreshProgress.vue**: 事件监听清理（`unlisten()`）
3. **useFeeds.ts**: 缓存失效时机（添加/删除 Feed 后）
