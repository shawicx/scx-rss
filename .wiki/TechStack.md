# 技术栈

最后更新：2026-05-03

## 目录

- [框架与运行时](#框架与运行时)
- [前端依赖](#前端依赖)
- [后端依赖 (Rust)](#后端依赖-rust)
- [构建工具链](#构建工具链)

## 框架与运行时

| 层级 | 技术 | 版本 | 说明 |
|------|------|------|------|
| 桌面框架 | Tauri | v2 | Rust 后端 + WebView 前端 |
| 前端框架 | Vue | 3.5+ | Composition API + `<script setup>` |
| 语言（前端） | TypeScript | 5.6+ | 类型安全 |
| 语言（后端） | Rust | 2021 edition | 高性能系统语言 |
| 数据库 | SQLite | via rusqlite (bundled) | 本地嵌入式数据库 |
| 样式方案 | Vuetify | 3.x | Material Design 组件库 |

## 前端依赖

> 参考：`package.json`

### 生产依赖

| 包名 | 版本 | 用途 |
|------|------|------|
| `vue` | ^3.5.0 | 响应式 UI 框架 |
| `vuetify` | ^3.7.0 | Material Design 组件库 |
| `@tauri-apps/api` | ^2.1.0 | Tauri IPC 调用（invoke、events） |
| `@tauri-apps/plugin-dialog` | ^2.4.2 | 文件选择对话框 |
| `@tauri-apps/plugin-fs` | ^2.4.4 | 文件系统读写 |
| `@tauri-apps/plugin-shell` | ^2.0.0 | 系统 Shell 操作 |

### 开发依赖

| 包名 | 版本 | 用途 |
|------|------|------|
| `vite` | ^6.0.0 | 构建工具 |
| `@vitejs/plugin-vue` | ^5.2.0 | Vue SFC 编译 |
| `vite-plugin-vuetify` | ^2.0.0 | Vuetify 按需加载 |
| `typescript` | ~5.6.0 | 类型检查 |
| `vue-tsc` | ^2.1.0 | Vue 模板类型检查 |
| `eslint` | ^9.15.0 | 代码检查 |
| `eslint-plugin-vue` | ^9.30.0 | Vue 规则 |
| `eslint-config-prettier` | ^9.1.0 | Prettier 兼容 |
| `prettier` | ^3.4.0 | 代码格式化 |

## 后端依赖 (Rust)

> 参考：`src-tauri/Cargo.toml`

| Crate | 版本 | 用途 |
|-------|------|------|
| `tauri` | 2 | 桌面应用框架核心 |
| `tauri-plugin-shell` | 2 | Shell 集成插件 |
| `tauri-plugin-dialog` | 2 | 原生对话框插件 |
| `tauri-plugin-fs` | 2 | 文件系统插件 |
| `serde` + `serde_json` | 1 | 序列化/反序列化 |
| `tokio` | 1 (full) | 异步运行时 |
| `futures` | 0.3 | 异步工具集 |
| `reqwest` | 0.12 (json, cookies) | HTTP 客户端 |
| `feed` / `feed-rs` | 2 | RSS/Atom 解析 |
| `rusqlite` | 0.32 (bundled) | SQLite 绑定 |
| `opml` | 1.1 | OPML 格式处理 |
| `tracing` + `tracing-subscriber` | 0.1 / 0.3 | 结构化日志 |
| `tracing-appender` | 0.2 | 日志文件输出 |
| `anyhow` | 1 | 通用错误处理 |
| `thiserror` | 1 | 派生错误类型 |
| `url` | 2 | URL 解析与验证 |
| `chrono` | 0.4 | 时间处理 |
| `encoding_rs` | 0.8 | 字符编码检测（GBK 等） |
| `tokio-util` | 0.7 | Tokio 异步工具 |
| `once_cell` | 1.19 | 全局静态变量 |

## 构建工具链

| 工具 | 配置文件 | 说明 |
|------|----------|------|
| Vite | `vite.config.ts` | 前端构建，路径别名 `@/`，Vuetify 插件 |
| Vuetify | `src/plugins/vuetify.ts` | 主题与组件配置 |
| TypeScript | `tsconfig.json` | 类型检查配置 |
| ESLint | `eslint.config.js` | 代码质量检查 |
| Prettier | `.prettierrc` | 代码格式化 |
| Cargo | `src-tauri/Cargo.toml` | Rust 构建配置 |
| Tauri | `src-tauri/tauri.conf.json` | 应用配置（窗口、权限） |

### 关联文件

- `package.json`
- `src-tauri/Cargo.toml`
- `vite.config.ts`
- `src/plugins/vuetify.ts`
- `tsconfig.json`
- `eslint.config.js`
