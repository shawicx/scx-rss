<script setup lang="ts">
import { ref, onMounted } from 'vue'
import FeedList from './FeedList.vue'
import RefreshProgress from './RefreshProgress.vue'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { invoke } from '@tauri-apps/api/core'

/**
 * 侧边栏组件
 * 包含 Feed 列表和设置入口
 */
const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { feeds, loading, init, refresh } = useFeeds()
const { showSuccess, showError } = useToast()

// 批量刷新状态
const showRefreshProgress = ref(false)
const isRefreshing = ref(false)

// 组件挂载时初始化 Feeds
onMounted(async () => {
  await init()
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

  // 刷新 Feed 列表
  refresh()

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
</script>

<template>
  <div class="h-full flex flex-col bg-gray-100 border-r border-gray-200">
    <!-- 侧边栏头部 -->
    <div class="p-4 border-b border-gray-200">
      <h2 class="text-lg font-semibold text-gray-800">
        SCX RSS Reader
      </h2>
      <p class="text-xs text-gray-500 mt-1">
        {{ feeds.length }} 个订阅源
      </p>

      <!-- 批量刷新按钮 -->
      <button
        @click="startBatchRefresh"
        :disabled="isRefreshing || feeds.length === 0"
        class="mt-3 w-full px-4 py-2 text-sm text-white bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed rounded transition-colors flex items-center justify-center gap-2"
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

    <!-- Feed 列表容器 -->
    <div class="flex-1 overflow-y-auto">
      <FeedList :feeds="feeds" :loading="loading" @feed-selected="emit('feed-selected', $event)" />
    </div>

    <!-- 侧边栏底部：设置入口 -->
    <div class="p-4 border-t border-gray-200">
      <button
        class="w-full px-4 py-2 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded transition-colors"
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
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.3);
}
</style>
