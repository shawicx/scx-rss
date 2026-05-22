export default {
  // 通用
  common: {
    loading: '加载中...',
    save: '保存',
    cancel: '取消',
    delete: '删除',
    edit: '编辑',
    close: '关闭',
    confirm: '确认',
  },

  // 设置页面
  settings: {
    title: '设置',
    language: '语言',
    theme: '主题',
    autoRefresh: '自动刷新',
    autoRefreshDesc: '定时自动拉取所有订阅源的最新文章，仅在发现新内容时通知。',
    enableAutoRefresh: '启用自动刷新',
    refreshInterval: '刷新间隔',
    lastRefresh: '上次刷新',
    opml: 'OPML 导入 / 导出',
    opmlDesc: '将订阅源导出为 OPML 文件以备份，或从 OPML 文件导入订阅源。',
    export: '导出',
    import: '导入',
    backup: '数据备份 / 恢复',
    backupDesc: '备份完整数据库（含文章、订阅、阅读状态），或从备份文件恢复。',
    backupData: '备份数据',
    restoreData: '恢复数据',
    followSystem: '跟随系统',
    simplifiedChinese: '简体中文',
    english: 'English',
  },

  // 文章列表
  articles: {
    all: '全部',
    unread: '未读',
    starred: '收藏',
    noArticles: '暂无文章',
    noArticlesDesc: '订阅源后，文章将显示在这里',
  },

  // Feed 列表
  feeds: {
    allFeeds: '所有订阅源',
    uncategorized: '未分类',
    addFeed: '添加订阅源',
    editFeed: '编辑订阅源',
    deleteFeed: '删除订阅源',
    feedUrl: '订阅源 URL',
    feedTitle: '标题',
    feedCategory: '分类',
    noFeeds: '暂无订阅源',
    noFeedsDesc: '点击上方按钮添加订阅源',
  },

  // 分类
  categories: {
    uncategorized: '未分类',
    allCategories: '所有分类',
  },

  // 刷新进度
  refresh: {
    refreshing: '正在刷新...',
    refreshAll: '全部刷新',
    refreshSuccess: '刷新成功',
    refreshFailed: '刷新失败',
    completed: '完成',
  },

  // 错误消息
  errors: {
    network: '网络连接失败，请检查网络设置',
    parse: 'RSS 解析失败，格式可能不正确',
    database: '数据库操作失败',
    validation: '数据验证失败',
    json: 'JSON 解析失败',
    io: '文件操作失败',
    invalidUrl: '无效的 URL 地址',
    feedNotFound: '未找到该订阅源',
    backupCancelled: '备份已取消',
    backupSuccess: '数据库备份成功',
    backupFailed: '备份失败',
    restoreCancelled: '恢复已取消',
    restoreConfirm: '恢复操作将覆盖当前所有数据，是否继续？',
    restoreSuccess: '数据恢复成功，正在重新加载...',
    restoreFailed: '恢复失败',
    urlAlreadyUsed: '该 URL 已被其他订阅源使用',
  },

  // 主题
  theme: {
    light: '浅色',
    dark: '深色',
    system: '跟随系统',
  },

  // Toast 消息
  toast: {
    success: '操作成功',
    error: '操作失败',
    info: '提示',
  },

  // 版本信息
  about: {
    version: '版本',
  },
}
