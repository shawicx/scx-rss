# SCX RSS Reader Wiki

最后更新：2026-05-03

> 基于 Tauri v2 + Vue 3 的本地优先 RSS 阅读器

## 文档索引

- [架构设计](Architecture.md) — 整体架构、数据流、模块关系
- [技术栈](TechStack.md) — 前后端技术选型与依赖清单
- [前端模块](Frontend.md) — Vue 组件、Composables、类型定义
- [后端模块](Backend.md) — Rust 命令、核心模块（数据库/网络/解析）
- [数据模型](DataModel.md) — SQLite 表结构、索引、数据结构
- [开发指南](Development.md) — 环境搭建、构建命令、代码规范

## 项目概览

SCX RSS Reader 是一个单用户、本地优先的桌面 RSS 阅读器，数据完全存储在本地 SQLite 数据库中，支持 OPML 导入/导出。

### 核心特性

- 本地数据存储，无需服务端
- RSS/Atom 订阅源拉取与解析
- 文章阅读、已读/收藏管理
- 分类管理
- OPML 导入/导出
- 自动刷新（定时静默拉取）
- Toast 通知系统
- 夕阳色主题 UI
