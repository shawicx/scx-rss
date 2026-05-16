# SCX RSS Reader

本地优先的桌面 RSS 阅读器

## AI 快速入口

**重要**: AI 助手（Claude Code、Cursor、Copilot）请先阅读 [.wiki/ai-context.md](.wiki/ai-context.md) 以最快速度理解项目。

## 技术栈

- **Desktop**: Tauri v2
- **Frontend**: Vue 3 + Vuetify + TypeScript
- **Backend**: Rust + SQLite (rusqlite)
- **Network**: reqwest + feed-rs

## 启动方式

```bash
pnpm install
pnpm tauri:dev    # 开发模式 (http://localhost:1420)
```

## 打包方式

```bash
pnpm tauri:build  # 生产构建
```

输出: `src-tauri/target/release/bundle/`

## 核心目录

```
src/                    # Vue 3 前端
├── components/          # 9 个 Vuetify 组件
├── composables/         # 9 个业务逻辑 hooks
└── types/               # TypeScript 类型定义

src-tauri/              # Rust 后端
├── src/
│   ├── commands/        # IPC handlers (db.rs, feed.rs)
│   └── core/            # 核心模块 (database, network, parser)
└── capabilities/        # 权限配置
```

## 关键文件

- `src-tauri/src/main.rs` - 应用入口，插件初始化
- `src-tauri/src/commands/feed.rs` - Feed 拉取、刷新、批量操作
- `src-tauri/src/commands/db.rs` - 数据库 CRUD
- `src/composables/useFeeds.ts` - Feed 业务逻辑
- `src/composables/useArticles.ts` - 文章业务逻辑
