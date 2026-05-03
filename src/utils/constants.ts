/**
 * 应用常量定义
 */

/** 应用名称 */
export const APP_NAME = 'SCX-RSS'

/** 应用版本 */
export const APP_VERSION = '0.1.0'

/** 应用描述 */
export const APP_DESCRIPTION = '基于 Tauri + Vue 3 的现代 RSS 阅读器'

/** GitHub 仓库地址 */
export const GITHUB_REPO = 'https://github.com/scx/scx-rss'

/** 分页配置 */
export const PAGINATION = {
  /** 默认每页文章数量 */
  DEFAULT_PAGE_SIZE: 50,
  /** 最大每页文章数量 */
  MAX_PAGE_SIZE: 200,
  /** 文章列表加载更多时的增量 */
  LOAD_MORE_INCREMENT: 20
} as const

/** 网络请求配置 */
export const NETWORK = {
  /** Feed 请求超时时间（毫秒） */
  FEED_TIMEOUT: 15000,
  /** 最大重试次数 */
  MAX_RETRIES: 3,
  /** 批量刷新时的并发数 */
  BATCH_CONCURRENCY: 3,
  /** 批量刷新时的请求间隔（毫秒） */
  BATCH_REQUEST_INTERVAL: 100
} as const

/** Toast 持续时间配置 */
export const TOAST_DURATION = {
  /** 成功提示持续时间（毫秒） */
  SUCCESS: 3000,
  /** 错误提示持续时间（毫秒） */
  ERROR: 5000,
  /** 信息提示持续时间（毫秒） */
  INFO: 3000
} as const

/** 本地存储键名 */
export const STORAGE_KEYS = {
  /** 侧边栏宽度 */
  SIDEBAR_WIDTH: 'sidebar-width',
  /** 主题偏好 */
  THEME: 'theme-preference',
  /** 自动刷新开关 */
  AUTO_REFRESH_ENABLED: 'auto-refresh-enabled',
  /** 自动刷新间隔 */
  AUTO_REFRESH_INTERVAL: 'auto-refresh-interval',
  /** 最后刷新时间 */
  LAST_REFRESH_TIME: 'last-refresh-time'
} as const

/** 主题类型 */
export const THEMES = {
  /** 浅色主题 */
  LIGHT: 'light',
  /** 深色主题 */
  DARK: 'dark',
  /** 跟随系统 */
  SYSTEM: 'system'
} as const

/** 自动刷新间隔选项（分钟） */
export const AUTO_REFRESH_OPTIONS = [
  { title: '每 15 分钟', value: 15 },
  { title: '每 30 分钟', value: 30 },
  { title: '每 1 小时', value: 60 },
  { title: '每 2 小时', value: 120 },
  { title: '每 4 小时', value: 240 }
] as const

/** 文章筛选类型 */
export const ARTICLE_FILTERS = {
  /** 所有文章 */
  ALL: 'all',
  /** 未读文章 */
  UNREAD: 'unread',
  /** 收藏文章 */
  STARRED: 'starred'
} as const

/** 键盘快捷键 */
export const KEYBOARD_SHORTCUTS = {
  /** 下一篇文章 */
  NEXT_ARTICLE: 'j',
  /** 上一篇文章 */
  PREV_ARTICLE: 'k',
  /** 标记已读 */
  MARK_READ: 'r',
  /** 收藏/取消收藏 */
  TOGGLE_STAR: 's',
  /** 刷新 Feed */
  REFRESH: 'R',
  /** 打开文章链接 */
  OPEN_LINK: 'o',
  /** 关闭文章视图 */
  CLOSE_VIEW: 'Escape'
} as const
