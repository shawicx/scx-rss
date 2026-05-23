import { ref, computed } from 'vue'
import type { Ref } from 'vue'
import type { Article, ArticleFilter } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'
import { useI18n } from './useI18n'

/**
 * 文章管理组合式函数
 * 提供文章的查询和状态更新功能
 */
export function useArticles() {
  const { showSuccess, showError } = useToast()
  const { t } = useI18n()

  // 文章列表状态
  const articles: Ref<Article[]> = ref([])
  const allArticles: Ref<Article[]> = ref([]) // 用于统计，不受筛选影响
  const loading = ref(false)

  // 当前选中的 Feed ID
  const currentFeedId = ref<number | undefined>(undefined)

  // 筛选条件
  const filter = ref<ArticleFilter>({
    feed_id: undefined,
    unread_only: false,
    starred_only: false,
    limit: 50 // 默认加载 50 篇文章
  })

  /**
   * 查询文章列表
   * @param customFilter - 自定义筛选条件（可选）
   */
  const listArticles = async (customFilter?: ArticleFilter) => {
    try {
      loading.value = true
      const queryFilter = customFilter || filter.value

      // 如果有筛选条件（只看未读或收藏），需要同时获取全部文章用于统计
      if (queryFilter.unread_only || queryFilter.starred_only) {
        const allFilter = { ...queryFilter, unread_only: false, starred_only: false }
        const [filtered, all] = await Promise.all([
          invoke<Article[]>('get_articles', { filter: queryFilter }),
          invoke<Article[]>('get_articles', { filter: allFilter })
        ])
        articles.value = filtered
        allArticles.value = all
        return filtered
      } else {
        const result = await invoke<Article[]>('get_articles', { filter: queryFilter })
        articles.value = result
        allArticles.value = result
        return result
      }
    } catch (error) {
      showError(`${t('toast.error')}: ${error}`)
      return []
    } finally {
      loading.value = false
    }
  }

  /**
   * 按 Feed ID 查询文章
   * @param feedId - Feed ID
   */
  const loadArticlesByFeed = async (feedId?: number) => {
    currentFeedId.value = feedId
    filter.value.feed_id = feedId
    return await listArticles()
  }

  /**
   * 标记文章为已读/未读
   * @param id - 文章 ID
   * @param isRead - 是否已读
   */
  const markAsRead = async (id: number, isRead: boolean = true) => {
    try {
      await invoke('update_article', {
        articleId: id,
        isRead: isRead,
        isStarred: undefined
      })

      // 更新本地状态
      const article = articles.value.find((a) => a.id === id)
      const allArticle = allArticles.value.find((a) => a.id === id)
      if (article) {
        article.is_read = isRead
      }
      if (allArticle) {
        allArticle.is_read = isRead
      }

      return true
    } catch (error) {
      showError(`更新文章状态失败: ${error}`)
      return false
    }
  }

  /**
   * 切换文章的已读状态
   * @param id - 文章 ID
   */
  const toggleRead = async (id: number) => {
    const article = articles.value.find((a) => a.id === id)
    if (article) {
      return await markAsRead(id, !article.is_read)
    }
    return false
  }

  /**
   * 切换文章的收藏状态
   * @param id - 文章 ID
   */
  const toggleStar = async (id: number) => {
    const article = articles.value.find((a) => a.id === id)
    if (!article) return false

    try {
      await invoke('update_article', {
        articleId: id,
        isRead: undefined,
        isStarred: !article.is_starred
      })

      // 更新本地状态
      article.is_starred = !article.is_starred
      const allArticle = allArticles.value.find((a) => a.id === id)
      if (allArticle) {
        allArticle.is_starred = !allArticle.is_starred
      }

      const message = article.is_starred ? '已收藏' : '已取消收藏'
      showSuccess(message)

      return true
    } catch (error) {
      showError(`更新收藏状态失败: ${error}`)
      return false
    }
  }

  /**
   * 筛选未读文章
   */
  const showUnreadOnly = async () => {
    filter.value.unread_only = true
    filter.value.starred_only = false
    return await listArticles()
  }

  /**
   * 筛选收藏文章
   */
  const showStarredOnly = async () => {
    filter.value.starred_only = true
    filter.value.unread_only = false
    return await listArticles()
  }

  /**
   * 显示所有文章
   */
  const showAll = async () => {
    filter.value.unread_only = false
    filter.value.starred_only = false
    return await listArticles()
  }

  /**
   * 获取单篇文章
   * @param id - 文章 ID
   */
  const getArticleById = (id: number): Article | undefined => {
    return articles.value.find((a) => a.id === id)
  }

  // 计算属性：未读文章数（基于全部文章）
  const unreadCount = computed(() => {
    return allArticles.value.filter((a) => !a.is_read).length
  })

  // 计算属性：收藏文章数（基于全部文章）
  const starredCount = computed(() => {
    return allArticles.value.filter((a) => a.is_starred).length
  })

  return {
    articles,
    loading,
    currentFeedId,
    filter,
    unreadCount,
    starredCount,
    listArticles,
    loadArticlesByFeed,
    markAsRead,
    toggleRead,
    toggleStar,
    showUnreadOnly,
    showStarredOnly,
    showAll,
    getArticleById
  }
}
