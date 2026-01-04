import { ref } from 'vue'
import type { Ref } from 'vue'
import type { Feed, NewFeed } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'

/**
 * Feed 管理组合式函数
 * 提供 Feed 的增删改查功能
 */
export function useFeeds() {
  const { showSuccess, showError } = useToast()

  // Feed 列表状态
  const feeds: Ref<Feed[]> = ref([])
  const loading = ref(false)

  /**
   * 获取所有 Feeds
   */
  const listFeeds = async () => {
    try {
      loading.value = true
      const result = await invoke<Feed[]>('get_all_feeds')
      feeds.value = result
      return result
    } catch (error) {
      showError(`获取 Feed 列表失败: ${error}`)
      return []
    } finally {
      loading.value = false
    }
  }

  /**
   * 添加新的 Feed
   * @param url - Feed URL
   * @param category - 分类（可选）
   */
  const addFeed = async (url: string, category?: string) => {
    try {
      loading.value = true
      const newFeed = await invoke<Feed>('add_feed', { url, category })
      feeds.value.push(newFeed)
      showSuccess(`成功添加 Feed: ${newFeed.title}`)
      return newFeed
    } catch (error) {
      showError(`添加 Feed 失败: ${error}`)
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * 删除 Feed
   * @param id - Feed ID
   */
  const deleteFeed = async (id: number) => {
    try {
      loading.value = true
      await invoke('delete_feed', { feedId: id })
      feeds.value = feeds.value.filter((feed) => feed.id !== id)
      showSuccess('Feed 删除成功')
      return true
    } catch (error) {
      showError(`删除 Feed 失败: ${error}`)
      return false
    } finally {
      loading.value = false
    }
  }

  /**
   * 刷新单个 Feed
   * @param id - Feed ID
   */
  const refreshFeed = async (id: number) => {
    try {
      loading.value = true
      const message = await invoke<string>('fetch_and_update_feed', { feedId: id })
      showSuccess(message)
      return true
    } catch (error) {
      showError(`刷新 Feed 失败: ${error}`)
      return false
    } finally {
      loading.value = false
    }
  }

  /**
   * 批量刷新所有 Feeds
   */
  const refreshAllFeeds = async () => {
    try {
      loading.value = true
      const message = await invoke<string>('batch_refresh_feeds')
      showSuccess(message)
      return true
    } catch (error) {
      showError(`批量刷新失败: ${error}`)
      return false
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取单个 Feed（从列表中查找）
   * @param id - Feed ID
   */
  const getFeedById = (id: number): Feed | undefined => {
    return feeds.value.find((feed) => feed.id === id)
  }

  /**
   * 初始化：加载所有 Feeds
   */
  const init = async () => {
    await listFeeds()
  }

  return {
    feeds,
    loading,
    listFeeds,
    refresh: listFeeds, // 别名，用于刷新 Feed 列表
    addFeed,
    deleteFeed,
    refreshFeed,
    refreshAllFeeds,
    getFeedById,
    init
  }
}
