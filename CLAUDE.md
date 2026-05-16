# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SCX RSS Reader — a local-first desktop RSS reader built with **Tauri v2** (Rust backend) + **Vue 3** (TypeScript frontend) + **SQLite** (local storage). Single-user, no server required. Supports RSS/Atom feed fetching, article reading (read/star), category management, and OPML import/export.

## Wiki-Driven Development

This project maintains a `.wiki/` directory as the knowledge base. Two rules apply to every feature task:

1. **Before developing a feature**: read the relevant `.wiki/` pages to understand current architecture and design decisions.
2. **After completing a feature**: update the affected `.wiki/` pages to keep documentation in sync with code.

### AI 助手必读

**Claude Code / Cursor / Copilot**: 在开始任何工作前，请先阅读 [.wiki/ai-context.md](.wiki/ai-context.md) 以最快速度理解项目核心架构。

### Wiki 索引

- [.wiki/README.md](.wiki/README.md) - Wiki 导航和项目概览
- [.wiki/ai-context.md](.wiki/ai-context.md) - AI 5分钟快速理解（**优先阅读**）
- [.wiki/architecture.md](.wiki/architecture.md) - 系统架构和数据流
- [.wiki/ipc.md](.wiki/ipc.md) - IPC 调用链和 command 映射
- [.wiki/risks.md](.wiki/risks.md) - 风险分析和注意事项

## Build & Run Commands

```bash
pnpm install            # Install frontend dependencies
pnpm tauri dev          # Dev mode (hot reload for both frontend & Rust)
pnpm tauri build        # Production build (creates installers)

# Rust-only
cd src-tauri && cargo fmt       # Format Rust code
cd src-tauri && cargo clippy    # Lint Rust code
cd src-tauri && cargo test      # Run Rust unit tests
```

Dev server runs at `http://localhost:1420`. Tauri rebuilds Rust automatically on changes.

## Architecture

Three-layer desktop app: **Vue frontend → Tauri IPC (invoke) → Rust backend → SQLite**.

### Frontend (`src/`)

- **Components** (`components/`): Three-pane layout — Sidebar (FeedList + CategoryList + Settings) | ArticleList | ArticleView. Plus ToastContainer and RefreshProgress.
- **Composables** (`composables/`): All business logic lives here — `useFeeds`, `useArticles`, `useOpml`, `useCategories`, `useToast`. Components call composables, composables call `invoke()`.
- **Types** (`types/`): `Feed`, `Article`, `Category` interfaces mirroring Rust structs.
- **Styling**: UnoCSS (presetWind, Tailwind-compatible) + custom sunset theme in `src/styles/sunset-theme.css`. Warm orange/amber gradient palette.
- No router/state library — state managed via `ref()` in composables, passed through props/events.

### Backend (`src-tauri/`)

- **Entry** (`main.rs`): Initializes DB on startup, registers all Tauri commands.
- **Commands** (`commands/db.rs`, `commands/feed.rs`): IPC command handlers. `db.rs` for CRUD, `feed.rs` for feed fetching + OPML.
- **Core modules** (`core/`): `database.rs` (SQLite via rusqlite), `network.rs` (HTTP via reqwest, 15s timeout, 3 retries with exponential backoff), `parser.rs` (RSS/Atom via feed-rs, handles GBK encoding), `error.rs` (thiserror-based AppError enum).
- **IPC bridge**: Frontend calls `invoke('command_name', { params })`, Rust handlers receive `app_handle` for DB access.

### Data Flow

```
User action → Vue component → composable → invoke() → Rust command → core module → SQLite
                                                                       ↓
Response ← composable ← invoke() result ← Rust command ← core module ←
```

Batch refresh uses Tauri Events to push real-time progress to `RefreshProgress.vue`.

## Mandatory Rules

- **禁止擅自提交代码**: 未经用户明确指示，不得执行 `git commit`。即使计划中包含提交步骤，也必须先获得用户确认。

## Key Conventions

- **Git commits**: Conventional Commits (`feat:`, `fix:`, `refactor:`, etc.)
- **Vue components**: `<script setup lang="ts">` with Composition API
- **Rust**: `cargo fmt` + `cargo clippy` before committing
- **TypeScript**: ESLint + Prettier configured
- **Path alias**: `@/` maps to `./src/` (configured in vite.config.ts)
- **Database**: All DB access goes through Rust core/database.rs. Frontend never touches SQLite directly. Articles use `(feed_id, guid)` unique constraint for dedup.
