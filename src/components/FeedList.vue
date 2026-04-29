<script setup lang="ts">
import { ref } from 'vue'
import type { Feed } from '@/types'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { validateFeedUrl } from '@/utils/validators'

/**
 * Feed 列表组件
 * 提供订阅源的显示、添加、删除和刷新功能
 */
interface Props {
  feeds: Feed[]
  loading: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { addFeed, deleteFeed, refreshFeed, refreshAllFeeds } = useFeeds()

const { showError } = useToast()

// 添加 Feed 的对话框状态
const showAddDialog = ref(false)
const newFeedUrl = ref('')
const newFeedCategory = ref('')
const adding = ref(false)

// 当前选中的 Feed ID
const selectedFeedId = ref<number | undefined>(undefined)

/**
 * 处理 Feed 选择
 */
const handleSelectFeed = (feedId: number) => {
  selectedFeedId.value = feedId
  emit('feed-selected', feedId)
}

/**
 * 显示添加 Feed 对话框
 */
const openAddDialog = () => {
  newFeedUrl.value = ''
  newFeedCategory.value = ''
  showAddDialog.value = true
}

/**
 * 添加新的 Feed
 */
const handleAddFeed = async () => {
  // 验证 URL
  const validation = validateFeedUrl(newFeedUrl.value)
  if (!validation.valid) {
    showError(validation.error)
    return
  }

  adding.value = true
  const success = await addFeed(newFeedUrl.value, newFeedCategory.value || undefined)

  adding.value = false

  if (success) {
    showAddDialog.value = false
  }
}

/**
 * 删除 Feed
 */
const handleDeleteFeed = async (feedId: number, feedTitle: string) => {
  const confirmed = confirm(`确定要删除订阅源 "${feedTitle}" 吗？`)
  if (!confirmed) return

  const success = await deleteFeed(feedId)

  if (success && selectedFeedId.value === feedId) {
    selectedFeedId.value = undefined
  }
}

/**
 * 刷新单个 Feed
 */
const handleRefreshFeed = async (feedId: number) => {
  await refreshFeed(feedId)
}

/**
 * 批量刷新所有 Feeds
 */
const handleRefreshAll = async () => {
  await refreshAllFeeds()
}
</script>

<template>
  <div class="p-2">
    <!-- 工具栏 -->
    <div class="flex gap-2 mb-3 px-2">
      <button
        class="btn-sunset flex-1 px-3 py-1.5 text-sm"
        :disabled="loading"
        @click="openAddDialog"
      >
        + 添加
      </button>
      <button
        class="flex-1 px-3 py-1.5 text-sm bg-gradient-to-r from-amber-500 to-orange-500 text-white rounded hover:from-amber-600 hover:to-orange-600 transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
        :disabled="loading || feeds.length === 0"
        title="刷新所有订阅源"
        @click="handleRefreshAll"
      >
        ↻ 全部刷新
      </button>
    </div>

    <!-- Feed 列表 -->
    <div v-if="feeds.length === 0 && !loading" class="px-2 py-8 text-center text-gray-500 text-sm">
      <p>还没有订阅任何源</p>
      <p class="mt-2 text-xs">点击"添加"按钮开始订阅</p>
    </div>

    <div v-else class="space-y-1">
      <div
        v-for="feed in feeds"
        :key="feed.id"
        class="group flex items-center gap-2 px-3 py-2 rounded cursor-pointer transition-all duration-300 hover:scale-[1.01]"
        :class="[
          selectedFeedId === feed.id
            ? 'bg-gradient-to-r from-orange-100 to-amber-100 text-orange-900 shadow-sm'
            : 'hover:bg-orange-50 text-gray-700',
        ]"
        @click="handleSelectFeed(feed.id)"
      >
        <!-- Feed 图标 -->
        <div
          class="flex-shrink-0 w-8 h-8 rounded bg-gradient-to-br from-orange-400 to-amber-500 flex items-center justify-center text-white text-xs font-bold shadow-sm"
        >
          {{ feed.title.charAt(0).toUpperCase() }}
        </div>

        <!-- Feed 信息 -->
        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium truncate">
            {{ feed.title }}
          </div>
          <div v-if="feed.category" class="text-xs text-orange-600 truncate">
            {{ feed.category }}
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            class="w-7 h-7 flex items-center justify-center text-orange-600 hover:bg-gradient-to-br hover:from-orange-400 hover:to-amber-500 hover:text-white rounded-full transition-all duration-300 hover:scale-110"
            title="刷新"
            @click.stop="handleRefreshFeed(feed.id)"
          >
            ↻
          </button>
          <button
            class="w-7 h-7 flex items-center justify-center text-orange-600 hover:text-white hover:bg-gradient-to-br hover:from-red-400 hover:to-red-500 rounded-full transition-all duration-300 hover:scale-110"
            title="删除"
            @click.stop="handleDeleteFeed(feed.id, feed.title)"
          >
            ✕
          </button>
        </div>
      </div>
    </div>

    <!-- 添加 Feed 对话框 -->
    <div
      v-if="showAddDialog"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      @click.self="showAddDialog = false"
    >
      <div class="bg-white rounded-lg shadow-xl p-6 w-full max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">添加订阅源</h3>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Feed URL <span class="text-red-500">*</span>
            </label>
            <input
              v-model="newFeedUrl"
              type="url"
              placeholder="https://example.com/feed.xml"
              class="input-sunset"
              @keypress.enter="handleAddFeed"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1"> 分类（可选） </label>
            <input
              v-model="newFeedCategory"
              type="text"
              placeholder="例如：技术、新闻"
              class="input-sunset"
              @keypress.enter="handleAddFeed"
            />
          </div>
        </div>

        <div class="flex gap-3 mt-6">
          <button
            class="flex-1 px-4 py-2 text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-all duration-300 hover:scale-105"
            @click="showAddDialog = false"
          >
            取消
          </button>
          <button
            :disabled="adding || !newFeedUrl"
            class="btn-sunset flex-1 px-4 py-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            @click="handleAddFeed"
          >
            {{ adding ? '添加中...' : '添加' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
