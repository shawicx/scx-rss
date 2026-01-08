<script setup lang="ts">
import { ref, onMounted } from 'vue'
import CategoryList from './CategoryList.vue'
import FeedList from './FeedList.vue'
import RefreshProgress from './RefreshProgress.vue'
import Settings from './Settings.vue'
import { useFeeds } from '@/composables/useFeeds'
import { useCategories } from '@/composables/useCategories'
import { useToast } from '@/composables/useToast'
import { invoke } from '@tauri-apps/api/core'

/**
 * 侧边栏组件
 * 包含分类列表（含 Feed）和设置入口
 */
const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { feeds, loading, init, refresh, deleteFeed, refreshFeed } = useFeeds()
const { categories, init: initCategories, refresh: refreshCategories } = useCategories()
const { showSuccess, showError } = useToast()

// 批量刷新状态
const showRefreshProgress = ref(false)
const isRefreshing = ref(false)

// 设置对话框状态
const showSettings = ref(false)

// 当前选中的 Feed ID
const selectedFeedId = ref<number | undefined>(undefined)

// 组件挂载时初始化
onMounted(async () => {
  await init()
  await initCategories()
})

// 开始批量刷新
const startBatchRefresh = async () => {
  if (isRefreshing.value || feeds.value.length === 0) {
    return
  }

  isRefreshing.value = true
  showRefreshProgress.value = true

  try {
    await invoke('batch_refresh_feeds')
  } catch (error) {
    showError(`批量刷新失败: ${error}`)
    showRefreshProgress.value = false
  }
}

// 刷新完成回调
const onRefreshComplete = (success: number, failed: number, newArticles: number) => {
  isRefreshing.value = false
  showRefreshProgress.value = false

  // 刷新 Feed 列表和分类列表
  refresh()
  refreshCategories()

  if (failed === 0) {
    showSuccess(`刷新完成！成功 ${success} 个，新增 ${newArticles} 篇文章`)
  } else {
    showError(`刷新完成：成功 ${success} 个，失败 ${failed} 个，新增 ${newArticles} 篇文章`)
  }
}

// 关闭进度对话框
const closeRefreshProgress = () => {
  showRefreshProgress.value = false
  isRefreshing.value = false
}

// 打开设置对话框
const openSettings = () => {
  showSettings.value = true
}

// 关闭设置对话框
const closeSettings = () => {
  showSettings.value = false
}

// 处理 Feed 选择
const handleFeedSelected = (feedId: number) => {
  selectedFeedId.value = feedId
  emit('feed-selected', feedId)
}

// 处理 Feed 删除
const handleFeedDelete = async (feedId: number) => {
  const success = await deleteFeed(feedId)
  if (success && selectedFeedId.value === feedId) {
    selectedFeedId.value = undefined
  }
}

// 处理 Feed 刷新
const handleFeedRefresh = async (feedId: number) => {
  await refreshFeed(feedId)
}
</script>

<template>
  <div class="h-full flex flex-col bg-gradient-to-br from-orange-50 to-amber-50 border-r border-orange-200">
    <!-- 侧边栏头部 -->
    <div class="p-4 border-b border-orange-200">
      <h2 class="text-lg font-semibold bg-gradient-to-r from-orange-600 to-amber-600 bg-clip-text text-transparent">
        SCX RSS Reader
      </h2>
      <p class="text-xs text-orange-600 mt-1">
        {{ feeds.length }} 个订阅源
      </p>

      <!-- 批量刷新按钮 -->
      <button
        @click="startBatchRefresh"
        :disabled="isRefreshing || feeds.length === 0"
        class="btn-sunset mt-3 w-full px-4 py-2 text-sm flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
      >
        <svg
          v-if="!isRefreshing"
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
        <svg
          v-else
          class="w-4 h-4 animate-spin"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
        {{ isRefreshing ? '刷新中...' : '批量刷新' }}
      </button>
    </div>

    <!-- 分类和 Feed 列表容器 -->
    <div class="flex-1 overflow-y-auto">
      <!-- 分类列表（包含 Feed） -->
      <CategoryList
        :categories="categories"
        :feeds="feeds"
        :loading="loading"
        :selected-feed-id="selectedFeedId"
        @feed-selected="handleFeedSelected"
        @feed-delete="handleFeedDelete"
        @feed-refresh="handleFeedRefresh"
      />
    </div>

    <!-- 侧边栏底部：设置入口 -->
    <div class="p-4 border-t border-orange-200">
      <button
        @click="openSettings"
        class="w-full px-4 py-2 text-sm text-orange-700 hover:text-orange-900 hover:bg-orange-100 rounded transition-all duration-300 hover:scale-105"
      >
        ⚙️ 设置
      </button>
    </div>

    <!-- 刷新进度对话框 -->
    <RefreshProgress
      :show="showRefreshProgress"
      @complete="onRefreshComplete"
      @close="closeRefreshProgress"
    />

    <!-- 设置对话框 -->
    <Settings :show="showSettings" @close="closeSettings" />
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 - 夕阳色主题 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: rgba(255, 142, 83, 0.3);
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background-color: rgba(255, 142, 83, 0.5);
}
</style>
