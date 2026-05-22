import { ref } from 'vue'
import type { Ref } from 'vue'
import type { Category } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'
import { useI18n } from './useI18n'

/**
 * 分类管理组合式函数
 * 提供分类的查询功能
 */
export function useCategories() {
  const { showError } = useToast()
  const { t } = useI18n()

  // 分类列表状态
  const categories: Ref<Category[]> = ref([])
  const loading = ref(false)

  /**
   * 获取所有分类
   */
  const listCategories = async () => {
    try {
      loading.value = true
      const result = await invoke<Category[]>('get_categories')
      categories.value = result
      return result
    } catch (error) {
      showError(`${t('toast.error')}: ${error}`)
      return []
    } finally {
      loading.value = false
    }
  }

  /**
   * 刷新分类列表
   */
  const refresh = () => {
    return listCategories()
  }

  /**
   * 初始化：加载所有分类
   */
  const init = async () => {
    await listCategories()
  }

  return {
    categories,
    loading,
    listCategories,
    refresh,
    init
  }
}
