# 状态管理

## 状态来源

### Frontend State (Vue 3)
**无全局状态库**，使用 `ref()` 本地状态

```typescript
// useFeeds.ts
const feeds = ref<Feed[]>([])           // Feed 列表
const loading = ref(false)              // 加载状态

// useArticles.ts
const articles = ref<Article[]>([])     // 文章列表
const currentArticle = ref<Article>()   // 当前文章
```

**状态位置**: 各个 Composable 内部

## 状态更新方式

### 直接更新
```typescript
// 添加 Feed
feeds.value.push(newFeed)

// 删除 Feed
feeds.value = feeds.value.filter(f => f.id !== feedId)

// 更新文章
articles.value = articles.value.map(a =>
    a.id === articleId ? { ...a, is_read: true } : a
)
```

### IPC 触发更新
```typescript
// 刷新 Feed
const refreshFeed = async (id: number) => {
    await invoke('fetch_and_update_feed', { feedId: id })
    await getAllFeeds()  // 重新获取
}
```

## 最关键状态

### 1. Feeds 列表
**位置**: `useFeeds.ts`

**更新时机**:
- 添加/删除 Feed 后
- 刷新 Feed 后（更新 `last_fetched_at`）

**缓存**: 是（Feed 列表很少变化）

### 2. Articles 列表
**位置**: `useArticles.ts`

**更新时机**:
- 切换 Feed 时
- 筛选条件变化时
- 滚动加载时

**缓存**: 否（频繁变化）

### 3. 刷新进度
**位置**: `RefreshProgress.vue`

**更新方式**: Tauri Events 实时推送

```typescript
listen('refresh-progress', (event) => {
    progress.value = event.payload.current / event.payload.total
})
```

### 4. 语言设置
**位置**: `useI18n.ts`

**更新时机**:
- 应用启动时（从数据库或系统语言初始化）
- 用户在设置中切换语言时

**持久化**: `user_settings` 表（key='language'，value='zh-CN' | 'en' | 'system'）

```typescript
const { locale, setLocale } = useI18n()
locale.value // 当前实际语言 ('zh-CN' | 'en')
await setLocale('en') // 切换并持久化
```

## 无 Rust 全局状态

**原因**: Tauri Commands 无状态，每次调用独立

**例外**: 全局取消令牌（`CancellationToken`）

```rust
static CANCEL_TOKEN: Lazy<Mutex<Option<CancellationToken>>> = Lazy::new(|| Mutex::new(None));
```

## 本地缓存

### Feed 列表缓存
```typescript
// useFeeds.ts
const feedsCache = ref<Feed[]>([])

const getAllFeeds = async () => {
    if (feedsCache.value.length > 0) {
        return feedsCache.value  // 返回缓存
    }
    feedsCache.value = await invoke<Feed[]>('get_all_feeds')
    return feedsCache.value
}
```

**缓存失效**: 添加/删除 Feed 后清除

### 无文章缓存
文章列表频繁变化，不缓存

## 状态流转

### 用户操作 → 状态更新 → UI 刷新
```
用户点击 Feed
  ↓
ArticleList.vue: emit('select', feed)
  ↓
useArticles.getArticles(feedId)
  ↓
invoke('get_articles', { filter: { feed_id } })
  ↓
articles.value = newArticles
  ↓
ArticleList.vue 自动刷新（响应式）
```

### 后端事件 → 状态更新
```
batch_refresh_feeds 执行
  ↓
app.emit('refresh-progress', data)
  ↓
RefreshProgress.vue 监听
  ↓
progress.value = data.current / data.total
  ↓
进度条更新
```

## 风险点

1. **状态不同步**: 多个组件同时修改同一状态
2. **内存泄漏**: 事件监听未清理（`unlisten()`）
3. **缓存失效**: 缓存未及时清除导致显示旧数据
