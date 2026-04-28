# 开发指南

最后更新：2026-04-29

## 目录

- [环境要求](#环境要求)
- [快速开始](#快速开始)
- [构建命令](#构建命令)
- [代码规范](#代码规范)
- [项目配置](#项目配置)
- [调试技巧](#调试技巧)
- [开发进度](#开发进度)

## 环境要求

| 工具 | 最低版本 | 用途 |
|------|----------|------|
| Node.js | 18+ | 前端运行时 |
| pnpm | 最新稳定版 | 包管理器 |
| Rust | 1.80+ | 后端编译 |
| macOS | 12+ | 开发平台 |

### 推荐 IDE 插件

- Vue - Official（Vue 语言支持）
- rust-analyzer（Rust 语言支持）
- UnoCSS（原子化 CSS 提示）

## 快速开始

```bash
# 1. 安装前端依赖
pnpm install

# 2. 启动开发模式（前端 + Rust 后端热重载）
pnpm tauri dev

# 3. 构建生产版本
pnpm tauri build
```

## 构建命令

| 命令 | 说明 |
|------|------|
| `pnpm install` | 安装前端依赖 |
| `pnpm dev` | 仅启动 Vite 开发服务器 |
| `pnpm build` | 仅构建前端 |
| `pnpm preview` | 预览前端构建结果 |
| `pnpm tauri:dev` | 启动 Tauri 开发模式 |
| `pnpm tauri:build` | 构建生产安装包 |

### Rust 侧命令

```bash
cd src-tauri

# 格式化
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
```

## 代码规范

### Git 提交

使用 Conventional Commits 规范：

```
feat: 添加分类管理功能
fix: 修复文章列表排序问题
refactor: 重构 Feed 拉取逻辑
docs: 更新 Wiki 文档
style: 代码格式调整
test: 添加解析器单元测试
chore: 更新依赖版本
```

### TypeScript/Vue

- 使用 ESLint + Prettier 统一格式
- 组件使用 `<script setup lang="ts">` 语法
- 优先使用 Composition API

### Rust

- 使用 `cargo fmt` 格式化
- 使用 `cargo clippy` 检查代码质量
- 错误处理使用 `anyhow`（应用层）和 `thiserror`（库层）

### 分支管理

| 分支 | 用途 |
|------|------|
| `main` | 主分支，稳定版本 |
| `develop` | 开发分支 |
| `feature/xxx` | 功能开发 |
| `fix/xxx` | Bug 修复 |

## 项目配置

### 路径别名

```typescript
// vite.config.ts
resolve: {
  alias: { '@': resolve(__dirname, './src') }
}
```

### TypeScript 配置

> `tsconfig.json`

- 目标：ESNext
- 模块解析：Bundler
- 严格模式启用

### UnoCSS 配置

> `unocss.config.ts`

使用 `presetWind`（Tailwind CSS 兼容预设）。

## 调试技巧

### 前端调试

- 使用 Tauri DevTools（开发模式自动启用）
- `console.log` 输出到 DevTools 控制台

### 后端调试

- 使用 `tracing` 输出结构化日志
- 设置 `RUST_LOG=debug` 环境变量查看详细日志
- 日志输出到控制台（开发模式）和文件

### 数据库调试

- 使用 [DB Browser for SQLite](https://sqlitebrowser.org/) 查看数据库内容
- 数据库路径：Tauri 应用数据目录

## 开发进度

> 详细任务清单见 [TODO.md](../TODO.md)

当前阶段：**阶段 3 — 数据管理**

| 阶段 | 名称 | 进度 |
|------|------|------|
| 0 | 环境准备 | 100% |
| 1 | 核心功能 (MVP) | ~95%（待集成测试） |
| 2 | 用户体验优化 | ~50%（筛选/排序/快捷键待开发） |
| 3 | 数据管理 | ~50%（备份恢复待开发） |
| 4 | 高级功能 | 0% |

### 已完成

- 数据库建表与 CRUD
- Feed 拉取与解析（RSS/Atom）
- 批量刷新与进度显示
- 文章阅读与状态管理
- 分类管理
- OPML 导入/导出
- Toast 通知系统
- 夕阳色主题

### 待开发

- 文章筛选/排序
- 键盘快捷键
- 数据备份/恢复
- 全文搜索（FTS5）
- 自动刷新调度
- 主题切换

### 关联文件

- `package.json`
- `src-tauri/Cargo.toml`
- `vite.config.ts`
- `tsconfig.json`
- `unocss.config.ts`
- `eslint.config.js`
- `.prettierrc`
- `TODO.md`
