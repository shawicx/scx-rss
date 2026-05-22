export default {
  // Common
  common: {
    loading: 'Loading...',
    save: 'Save',
    cancel: 'Cancel',
    delete: 'Delete',
    edit: 'Edit',
    close: 'Close',
    confirm: 'Confirm',
  },

  // Settings
  settings: {
    title: 'Settings',
    language: 'Language',
    theme: 'Theme',
    autoRefresh: 'Auto Refresh',
    autoRefreshDesc: 'Automatically fetch latest articles from all feeds at regular intervals. Notify only when new content is found.',
    enableAutoRefresh: 'Enable Auto Refresh',
    refreshInterval: 'Refresh Interval',
    lastRefresh: 'Last Refresh',
    opml: 'OPML Import / Export',
    opmlDesc: 'Export feeds to OPML file for backup, or import feeds from OPML file.',
    export: 'Export',
    import: 'Import',
    backup: 'Data Backup / Restore',
    backupDesc: 'Backup complete database (including articles, feeds, read status), or restore from backup file.',
    backupData: 'Backup Data',
    restoreData: 'Restore Data',
    followSystem: 'Follow System',
    simplifiedChinese: '简体中文',
    english: 'English',
  },

  // Articles
  articles: {
    all: 'All',
    unread: 'Unread',
    starred: 'Starred',
    noArticles: 'No Articles',
    noArticlesDesc: 'Articles will appear here after adding feeds',
  },

  // Feeds
  feeds: {
    allFeeds: 'All Feeds',
    uncategorized: 'Uncategorized',
    addFeed: 'Add Feed',
    editFeed: 'Edit Feed',
    deleteFeed: 'Delete Feed',
    feedUrl: 'Feed URL',
    feedTitle: 'Title',
    feedCategory: 'Category',
    noFeeds: 'No Feeds',
    noFeedsDesc: 'Click the button above to add feeds',
  },

  // Categories
  categories: {
    uncategorized: 'Uncategorized',
    allCategories: 'All Categories',
  },

  // Refresh
  refresh: {
    refreshing: 'Refreshing...',
    refreshAll: 'Refresh All',
    refreshSuccess: 'Refresh successful',
    refreshFailed: 'Refresh failed',
    completed: 'Completed',
  },

  // Errors
  errors: {
    network: 'Network connection failed. Please check your network settings.',
    parse: 'RSS parsing failed. The format may be incorrect.',
    database: 'Database operation failed.',
    validation: 'Data validation failed.',
    json: 'JSON parsing failed.',
    io: 'File operation failed.',
    invalidUrl: 'Invalid URL.',
    feedNotFound: 'Feed not found.',
    backupCancelled: 'Backup cancelled',
    backupSuccess: 'Database backed up successfully',
    backupFailed: 'Backup failed',
    restoreCancelled: 'Restore cancelled',
    restoreConfirm: 'This will overwrite all current data. Continue?',
    restoreSuccess: 'Data restored successfully, reloading...',
    restoreFailed: 'Restore failed',
    urlAlreadyUsed: 'This URL is already used by another feed',
  },

  // Theme
  theme: {
    light: 'Light',
    dark: 'Dark',
    system: 'Follow System',
  },

  // Toast
  toast: {
    success: 'Success',
    error: 'Error',
    info: 'Info',
  },

  // About
  about: {
    version: 'Version',
  },
}
