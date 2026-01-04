<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import type { Article } from '@/types'
import { useArticles } from '@/composables/useArticles'
import { formatDate } from '@/utils/formatters'

/**
 * 文章列表组件
 * 显示文章列表，支持筛选和状态显示
 */
interface Props {
  feedId?: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'article-selected', article: Article): void
}>()

const {
  articles,
  loading,
  currentFeedId,
  unreadCount,
  starredCount,
  loadArticlesByFeed,
  toggleRead,
  toggleStar
} = useArticles()

// 筛选模式：'all' | 'unread' | 'starred'
const filterMode = ref<'all' | 'unread' | 'starred'>('all')

// 当前选中的文章 ID
const selectedArticleId = ref<number | undefined>(undefined)

/**
 * 组件挂载时加载文章
 */
onMounted(async () => {
  if (props.feedId !== undefined) {
    await loadArticlesByFeed(props.feedId)
  }
})

/**
 * 监听 feedId 变化
 */
watch(() => props.feedId, async (newFeedId) => {
  selectedArticleId.value = undefined
  if (newFeedId !== undefined) {
    await loadArticlesByFeed(newFeedId)
  } else {
    articles.value = []
  }
})

/**
 * 选择文章
 */
const handleSelectArticle = (article: Article) => {
  selectedArticleId.value = article.id
  emit('article-selected', article)

  // 自动标记为已读
  if (!article.is_read) {
    toggleRead(article.id)
  }
}

/**
 * 切换收藏状态
 */
const handleToggleStar = async (articleId: number, event: Event) => {
  event.stopPropagation()
  await toggleStar(articleId)
}

/**
 * 切换筛选模式
 */
const setFilterMode = async (mode: 'all' | 'unread' | 'starred') => {
  filterMode.value = mode
  if (mode === 'unread') {
    await useArticles().showUnreadOnly()
  } else if (mode === 'starred') {
    await useArticles().showStarredOnly()
  } else {
    await useArticles().showAll()
  }
}

/**
 * 获取文章摘要
 */
const getArticleSummary = (article: Article): string => {
  if (article.description) {
    return article.description.replace(/<[^>]*>/g, '').substring(0, 150)
  }
  if (article.content) {
    return article.content.replace(/<[^>]*>/g, '').substring(0, 150)
  }
  return '暂无摘要'
}
</script>

<template>
  <div class="h-full flex flex-col bg-white">
    <!-- 筛选工具栏 -->
    <div class="flex items-center gap-2 p-4 border-b border-gray-200">
      <button
        @click="setFilterMode('all')"
        class="px-3 py-1.5 text-sm rounded transition-colors"
        :class="filterMode === 'all' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'"
      >
        全部
      </button>
      <button
        @click="setFilterMode('unread')"
        class="px-3 py-1.5 text-sm rounded transition-colors"
        :class="filterMode === 'unread' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'"
      >
        未读 ({{ unreadCount }})
      </button>
      <button
        @click="setFilterMode('starred')"
        class="px-3 py-1.5 text-sm rounded transition-colors"
        :class="filterMode === 'starred' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'"
      >
        收藏 ({{ starredCount }})
      </button>
    </div>

    <!-- 文章列表 -->
    <div class="flex-1 overflow-y-auto">
      <!-- 空状态 -->
      <div
        v-if="articles.length === 0 && !loading"
        class="flex flex-col items-center justify-center h-full text-gray-500"
      >
        <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-lg">暂无文章</p>
        <p class="text-sm mt-2">
          {{ filterMode === 'unread' ? '没有未读文章' : filterMode === 'starred' ? '没有收藏的文章' : '选择一个订阅源开始阅读' }}
        </p>
      </div>

      <!-- 加载状态 -->
      <div
        v-if="loading"
        class="flex items-center justify-center h-full"
      >
        <div class="text-gray-500">加载中...</div>
      </div>

      <!-- 文章列表 -->
      <div v-else class="divide-y divide-gray-100">
        <article
          v-for="article in articles"
          :key="article.id"
          @click="handleSelectArticle(article)"
          class="p-4 cursor-pointer transition-colors hover:bg-gray-50"
          :class="{ 'bg-blue-50': selectedArticleId === article.id }"
        >
          <div class="flex gap-3">
            <!-- 已读/未读指示器 -->
            <div class="flex-shrink-0 mt-1">
              <div
                class="w-2 h-2 rounded-full transition-colors"
                :class="article.is_read ? 'bg-gray-300' : 'bg-blue-500'"
              />
            </div>

            <!-- 文章内容 -->
            <div class="flex-1 min-w-0">
              <!-- 标题和收藏按钮 -->
              <div class="flex items-start gap-2">
                <h3
                  class="flex-1 text-base font-medium text-gray-900 truncate"
                  :class="{ 'font-normal': article.is_read }"
                >
                  {{ article.title }}
                </h3>
                <button
                  @click="handleToggleStar(article.id, $event)"
                  class="flex-shrink-0 transition-colors"
                  :class="article.is_starred ? 'text-yellow-500' : 'text-gray-300 hover:text-yellow-500'"
                  title="收藏"
                >
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                  </svg>
                </button>
              </div>

              <!-- 元信息 -->
              <div class="flex items-center gap-2 mt-1 text-xs text-gray-500">
                <span>{{ formatDate(article.published_at || article.created_at) }}</span>
                <span v-if="article.author">· {{ article.author }}</span>
              </div>

              <!-- 摘要 -->
              <p class="mt-2 text-sm text-gray-600 line-clamp-2">
                {{ getArticleSummary(article) }}
              </p>
            </div>
          </div>
        </article>
      </div>
    </div>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 8px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.3);
}
</style>
