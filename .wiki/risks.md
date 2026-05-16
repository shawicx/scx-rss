# 风险分析

## 高频 IPC

### 风险点
```typescript
// ❌ 错误：循环调用
for (const feed of feeds) {
    await invoke('fetch_and_update_feed', { feedId: feed.id })
}
```

**影响**: 10 次 IPC = ~5ms 开销

### 解决方案
```typescript
// ✅ 正确：批量调用
await invoke('batch_refresh_feeds')
```

## 阻塞 IO

### 数据库操作
**位置**: `database.rs`

**风险**:
- 大量数据插入可能阻塞
- 复杂查询可能慢

**现状**: Tauri 自动异步化，无需担心

### 文件读写
**位置**: 备份/恢复功能

**风险**:
- 大文件（100MB+）可能阻塞
- 无进度提示

**解决方案**: 显示加载状态

## Shell 风险

### 当前状态
**已禁用**: 不使用 `shell:allow-execute`

**原因**: 防止命令注入

### 仅允许
```json
"shell:allow-open"  // 只打开 URL
```

**安全**: 不接受用户输入的命令

## Capability 风险

### 危险权限（未使用）
```json
// ❌ 永远不要添加
"shell:allow-execute"        // 可执行任意命令
"fs:allow-remove"            // 可删除文件
"fs:allow-copy-file"         // 可复制文件（数据窃取）
```

### 当前权限
```json
{
  "permissions": [
    "core:event:allow-emit",      // 事件推送
    "fs:allow-read-file",         // 文件读取
    "fs:allow-write-file",        // 文件写入
    "dialog:allow-open",          // 打开对话框
    "shell:allow-open"            // 打开链接
  ]
}
```

**风险**: 低（只授予必需权限）

## 大文件风险

### OPML 导入
**位置**: `feed.rs::import_opml`

**风险**:
- 大型 OPML 文件（1000+ Feeds）
- 可能插入大量数据

**现状**: 无进度提示

**建议**: 显示导入进度

### 数据库备份/恢复
**位置**: `db.rs::backup_database/restore_database`

**风险**:
- 大型数据库文件（100MB+）
- 可能阻塞 UI

**现状**: 无进度提示

## 内存风险

### 文章列表
**位置**: `ArticleList.vue`

**风险**:
- 一次性加载所有文章（10 万+）
- 内存占用：~500MB

**解决方案**: 分页加载（已实现）

### Feed 刷新
**位置**: `batch_refresh_feeds`

**风险**:
- 并发拉取多个 Feed
- 内存占用：~100MB

**现状**: 顺序执行，无并发风险

## 网络风险

### 恶意 URL
**位置**: `network.rs::fetch_feed`

**风险**:
- SSRF（服务端请求伪造）
- 内网扫描

**防护**: 无（信任用户输入）

**建议**: 添加 URL 白名单

### 超时风险
**位置**: `network.rs`

**当前**: 15 秒超时

**风险**: 慢速 Feed 可能超时

**现状**: 已实现 3 次重试

## 并发风险

### 取消机制
**位置**: `feed.rs::batch_refresh_feeds`

**风险**:
- 全局取消令牌可能冲突
- 多个批量刷新同时运行

**现状**: 单一全局令牌，安全

### 数据库并发
**位置**: `database.rs`

**风险**: 无（SQLite 单连接）

## 安全边界

### 输入验证
**前端**: TypeScript 类型系统
**后端**: Rust 类型系统

**风险**: 无类型攻击

### SQL 注入
**位置**: `database.rs`

**防护**: 参数化查询

```rust
conn.execute(
    "INSERT INTO articles (feed_id, title) VALUES (?, ?)",
    params![feed_id, title]
)
```

**风险**: 无

### XSS 攻击
**位置**: Vue 组件

**防护**: Vue 自动转义

**风险**: 无

## 性能风险

### 长时间运行
**位置**: `batch_refresh_feeds`

**风险**:
- 100 个 Feeds = ~5 分钟
- 可能阻塞其他操作

**现状**: 支持取消

### 内存泄漏
**位置**: 事件监听

**风险**:
- 未清理 `unlisten()`

**防护**: 组件卸载时清理

```typescript
onUnmounted(() => {
    unlisten?.()
})
```

## 关键风险总结

| 风险 | 等级 | 状态 |
|------|------|------|
| 循环 IPC 调用 | 🟡 中 | 已避免 |
| Shell 注入 | 🟢 低 | 已禁用 |
| 大文件处理 | 🟡 中 | 无进度 |
| 内存泄漏 | 🟡 中 | 需注意 |
| SQL 注入 | 🟢 低 | 已防护 |
| XSS 攻击 | 🟢 低 | 已防护 |
| 长时间运行 | 🟡 中 | 可取消 |
