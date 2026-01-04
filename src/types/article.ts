/**
 * 文章/条目类型定义
 */

export interface Article {
  /** 数据库主键 ID */
  id: number
  /** 所属 Feed 的 ID */
  feed_id: number
  /** 文章标题 */
  title: string
  /** 文章链接 */
  link: string
  /** 文章内容/摘要 */
  content?: string
  /** 文章描述/摘要 */
  description?: string
  /** 作者（可选） */
  author?: string
  /** 发布时间 (ISO 8601 格式) */
  published_at?: string
  /** 是否已读 */
  is_read: boolean
  /** 是否收藏 */
  is_starred: boolean
  /** 创建时间（抓取时间，ISO 8601 格式） */
  created_at: string
}

/** 文章查询过滤器 */
export interface ArticleFilter {
  /** 按 Feed ID 过滤 */
  feed_id?: number
  /** 只显示未读 */
  unread_only?: boolean
  /** 只显示收藏 */
  starred_only?: boolean
  /** 分页偏移量 */
  offset?: number
  /** 分页限制 */
  limit?: number
}

/** 文章状态更新参数 */
export interface ArticleUpdate {
  /** 是否已读 */
  is_read?: boolean
  /** 是否收藏 */
  is_starred?: boolean
}
