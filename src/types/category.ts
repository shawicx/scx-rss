/**
 * Feed 分类信息
 */
export interface Category {
  /** 分类名称 */
  name: string
  /** 该分类下的未读文章数 */
  unread_count: number
  /** 该分类下的 Feed 数量 */
  feed_count: number
}
