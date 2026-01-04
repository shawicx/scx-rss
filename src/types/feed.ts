/**
 * RSS Feed 订阅源类型定义
 */

export interface Feed {
  /** 数据库主键 ID */
  id: number
  /** Feed URL */
  url: string
  /** Feed 标题 */
  title: string
  /** Feed 描述 */
  description?: string
  /** Feed 图标 URL */
  icon_url?: string
  /** 分类（可选） */
  category?: string
  /** 创建时间 (ISO 8601 格式) */
  created_at: string
  /** 最后更新时间 (ISO 8601 格式) */
  updated_at: string
  /** 最后一次成功拉取的时间 (ISO 8601 格式) */
  last_fetched_at?: string
}

/** 用于创建新 Feed 的结构体 */
export interface NewFeed {
  /** Feed URL */
  url: string
  /** Feed 标题 */
  title: string
  /** Feed 描述 */
  description?: string
  /** Feed 图标 URL */
  icon_url?: string
  /** 分类（可选） */
  category?: string
}
