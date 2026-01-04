<script setup lang="ts">
import { ref } from 'vue'
import type { Category, Feed } from '@/types'
import { useFeeds } from '@/composables/useFeeds'
import { validateFeedUrl } from '@/utils/validators'

/**
 * 分类列表组件
 * 显示所有分类及其未读计数，支持折叠/展开
 * 每个 分类下方显示该分类的 Feeds
 */
interface Props {
  categories: Category[]
  feeds: Feed[]
  loading: boolean
  selectedFeedId?: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
  (e: 'feed-delete', feedId: number): void
  (e: 'feed-refresh', feedId: number): void
}>()

const { addFeed } = useFeeds()

// 折叠状态（分类名 -> 是否折叠）
const collapsedState = ref<Record<string, boolean>>({})

// 添加 Feed 的对话框状态
const showAddDialog = ref(false)
const newFeedUrl = ref('')
const newFeedCategory = ref('')
const adding = ref(false)

/**
 * 切换分类折叠状态
 */
const toggleCollapse = (categoryName: string) => {
  collapsedState.value[categoryName] = !collapsedState.value[categoryName]
}

/**
 * 判断分类是否折叠
 */
const isCollapsed = (categoryName: string): boolean => {
  return collapsedState.value[categoryName] || false
}

/**
 * 获取指定分类下的 Feeds
 */
const getFeedsByCategory = (categoryName: string | null): Feed[] => {
  if (categoryName === null) {
    // "全部文章"分类：显示没有分类的 Feeds
    return props.feeds.filter(feed => !feed.category)
  }
  // 其他分类：显示匹配该分类的 Feeds
  return props.feeds.filter(feed => feed.category === categoryName)
}

/**
 * 选择 Feed
 */
const handleSelectFeed = (feedId: number) => {
  emit('feed-selected', feedId)
}

/**
 * 删除 Feed
 */
const handleDeleteFeed = (feedId: number, feedTitle: string) => {
  const confirmed = confirm(`确定要删除订阅源 "${feedTitle}" 吗？`)
  if (confirmed) {
    emit('feed-delete', feedId)
  }
}

/**
 * 刷新 Feed
 */
const handleRefreshFeed = (feedId: number) => {
  emit('feed-refresh', feedId)
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
    alert(validation.error)
    return
  }

  adding.value = true
  const success = await addFeed(
    newFeedUrl.value,
    newFeedCategory.value || undefined
  )

  adding.value = false

  if (success) {
    showAddDialog.value = false
  }
}
</script>

<template>
  <div class="p-2">
    <!-- 工具栏：添加 Feed 按钮 -->
    <div class="mb-3 px-2">
      <button
        @click="openAddDialog"
        class="w-full px-3 py-2 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
        :disabled="loading"
      >
        + 添加订阅源
      </button>
    </div>

    <!-- 分类列表 -->
    <div v-if="categories.length === 0 && !loading" class="px-3 py-4 text-center text-gray-500 text-xs">
      <p>暂无分类</p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="category in categories"
        :key="category.name"
        class="flex flex-col"
      >
        <!-- 分类头部 -->
        <div
          @click="toggleCollapse(category.name)"
          class="group flex items-center gap-2 px-3 py-2 rounded cursor-pointer transition-colors hover:bg-gray-200"
        >
          <button
            class="flex-shrink-0 p-0.5 hover:bg-gray-300 rounded transition-colors"
            :class="{ 'rotate-0': !isCollapsed(category.name), 'rotate-[-90deg]': isCollapsed(category.name) }"
          >
            <svg
              class="w-4 h-4 transition-transform"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              />
            </svg>
          </button>

          <svg
            class="w-5 h-5 flex-shrink-0"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
            />
          </svg>

          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium truncate">
              {{ category.name }}
            </div>
            <div class="text-xs text-gray-500">
              {{ category.feed_count }} 个订阅源
            </div>
          </div>

          <div
            v-if="category.unread_count > 0"
            class="flex-shrink-0 px-2 py-0.5 text-xs font-medium bg-red-500 text-white rounded-full"
          >
            {{ category.unread_count }}
          </div>
        </div>

        <!-- 该分类下的 Feeds -->
        <div v-if="!isCollapsed(category.name)" class="ml-4 mt-1 space-y-1">
          <div
            v-for="feed in getFeedsByCategory(category.name)"
            :key="feed.id"
            @click="handleSelectFeed(feed.id)"
            class="group flex items-center gap-2 px-3 py-2 rounded cursor-pointer transition-colors"
            :class="[
              selectedFeedId === feed.id
                ? 'bg-blue-100 text-blue-800'
                : 'hover:bg-gray-200 text-gray-700'
            ]"
          >
            <div class="flex-shrink-0 w-6 h-6 rounded bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-xs font-bold">
              {{ feed.title.charAt(0).toUpperCase() }}
            </div>

            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate">
                {{ feed.title }}
              </div>
            </div>

            <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
              <button
                @click.stop="handleRefreshFeed(feed.id)"
                class="p-1 text-gray-500 hover:text-blue-600 hover:bg-blue-50 rounded"
                title="刷新"
              >
                ↻
              </button>
              <button
                @click.stop="handleDeleteFeed(feed.id, feed.title)"
                class="p-1 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded"
                title="删除"
              >
                ✕
              </button>
            </div>
          </div>

          <div v-if="getFeedsByCategory(category.name).length === 0" class="px-3 py-2 text-xs text-gray-500">
            该分类下暂无订阅源
          </div>
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
              class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
              @keypress.enter="handleAddFeed"
            >
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              分类（可选）
            </label>
            <input
              v-model="newFeedCategory"
              type="text"
              placeholder="例如：技术、新闻"
              class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
              @keypress.enter="handleAddFeed"
            >
          </div>
        </div>

        <div class="flex gap-3 mt-6">
          <button
            @click="showAddDialog = false"
            class="flex-1 px-4 py-2 text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-colors"
          >
            取消
          </button>
          <button
            @click="handleAddFeed"
            :disabled="adding || !newFeedUrl"
            class="flex-1 px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
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
.rotate-0 {
  transform: rotate(0deg);
}

.rotate-\[-90deg\] {
  transform: rotate(-90deg);
}
</style>
