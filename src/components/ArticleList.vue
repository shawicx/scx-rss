<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import type { Article } from '@/types'
import { useArticles } from '@/composables/useArticles'
import { useKeyboard } from '@/composables/useKeyboard'
import { formatDate } from '@/utils/formatters'

interface Props {
  feedId?: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'article-selected', article: Article): void
}>()

const { articles, loading, unreadCount, starredCount, loadArticlesByFeed, toggleRead, toggleStar } =
  useArticles()

const filterMode = ref<'all' | 'unread' | 'starred'>('all')
const selectedArticleId = ref<number | undefined>(undefined)

onMounted(async () => {
  if (props.feedId !== undefined) {
    await loadArticlesByFeed(props.feedId)
  }
})

watch(
  () => props.feedId,
  async newFeedId => {
    selectedArticleId.value = undefined
    if (newFeedId !== undefined) {
      await loadArticlesByFeed(newFeedId)
    } else {
      articles.value = []
    }
  }
)

const handleSelectArticle = (article: Article) => {
  selectedArticleId.value = article.id
  emit('article-selected', article)
  if (!article.is_read) {
    toggleRead(article.id)
  }
}

const handleToggleStar = async (articleId: number, event: Event) => {
  event.stopPropagation()
  await toggleStar(articleId)
}

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

const getArticleSummary = (article: Article): string => {
  if (article.description) {
    return article.description.replace(/<[^>]*>/g, '').substring(0, 140)
  }
  if (article.content) {
    return article.content.replace(/<[^>]*>/g, '').substring(0, 140)
  }
  return '暂无摘要'
}

const selectArticleByIndex = (index: number) => {
  const article = articles.value[index]
  if (article) {
    handleSelectArticle(article)
  }
}

useKeyboard({
  onNext: () => {
    if (articles.value.length === 0) return
    const idx = selectedArticleId.value
      ? articles.value.findIndex(a => a.id === selectedArticleId.value)
      : -1
    selectArticleByIndex(Math.min(idx + 1, articles.value.length - 1))
  },
  onPrev: () => {
    if (articles.value.length === 0) return
    const idx = selectedArticleId.value
      ? articles.value.findIndex(a => a.id === selectedArticleId.value)
      : 0
    selectArticleByIndex(Math.max(idx - 1, 0))
  },
  onToggleRead: () => {
    if (selectedArticleId.value) {
      toggleRead(selectedArticleId.value)
    }
  },
  onToggleStar: () => {
    if (selectedArticleId.value) {
      toggleStar(selectedArticleId.value)
    }
  }
})
</script>

<template>
  <div class="h-full flex flex-col" style="background: var(--ink-paper)">
    <!-- Filter Bar -->
    <div
      class="flex items-center gap-1 px-4 py-3"
      style="border-bottom: 1px solid var(--ink-border)"
    >
      <button
        v-for="mode in ['all', 'unread', 'starred'] as const"
        :key="mode"
        class="px-2.5 py-1 text-xs rounded transition-colors duration-150"
        :style="{
          background: filterMode === mode ? 'var(--ink-accent)' : 'transparent',
          color: filterMode === mode ? '#fff' : 'var(--ink-text-secondary)',
          fontWeight: filterMode === mode ? 500 : 400,
        }"
        @click="setFilterMode(mode)"
      >
        {{
          mode === 'all'
            ? '全部'
            : mode === 'unread'
              ? `未读 ${unreadCount}`
              : `收藏 ${starredCount}`
        }}
      </button>
    </div>

    <!-- Article List -->
    <div class="flex-1 overflow-y-auto">
      <!-- Empty State -->
      <div
        v-if="articles.length === 0 && !loading"
        class="flex flex-col items-center justify-center h-full px-6"
      >
        <svg
          class="w-12 h-12 mb-3"
          style="color: var(--ink-text-tertiary)"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="1.5"
            d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"
          />
        </svg>
        <p class="text-sm" style="color: var(--ink-text-tertiary)">暂无文章</p>
        <p class="text-xs mt-1" style="color: var(--ink-text-tertiary)">
          {{
            filterMode === 'unread'
              ? '没有未读文章'
              : filterMode === 'starred'
                ? '没有收藏文章'
                : '选择一个订阅源'
          }}
        </p>
      </div>

      <!-- Loading -->
      <div v-if="loading" class="flex items-center justify-center h-full">
        <div class="text-xs" style="color: var(--ink-text-tertiary)">加载中...</div>
      </div>

      <!-- Articles -->
      <div v-else>
        <article
          v-for="article in articles"
          :key="article.id"
          class="article-row group relative px-5 py-3.5 cursor-pointer transition-colors duration-150"
          :class="{ 'article-row--active': selectedArticleId === article.id }"
          @click="handleSelectArticle(article)"
        >
          <!-- Active indicator -->
          <div
            v-if="selectedArticleId === article.id"
            class="absolute left-0 top-2 bottom-2 w-[3px] rounded-r"
            style="background: var(--ink-accent)"
          ></div>

          <div class="flex gap-3">
            <!-- Unread dot -->
            <div class="flex-shrink-0 pt-1.5">
              <div
                class="w-1.5 h-1.5 rounded-full"
                :style="{ background: article.is_read ? 'transparent' : 'var(--ink-accent)' }"
              ></div>
            </div>

            <!-- Content -->
            <div class="flex-1 min-w-0">
              <!-- Title + Star -->
              <div class="flex items-start gap-1.5">
                <h3
                  class="flex-1 text-sm leading-snug truncate"
                  :style="{
                    fontWeight: article.is_read ? 400 : 500,
                    color: article.is_read ? 'var(--ink-text-secondary)' : 'var(--ink-text)',
                  }"
                >
                  {{ article.title }}
                </h3>
                <button
                  class="star-btn flex-shrink-0 mt-0.5 transition-opacity duration-100"
                  :class="{ 'star-btn--active': article.is_starred }"
                  title="收藏"
                  @click="handleToggleStar(article.id, $event)"
                >
                  <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 20 20">
                    <path
                      d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
                    />
                  </svg>
                </button>
              </div>

              <!-- Meta -->
              <div
                class="flex items-center gap-1.5 mt-1 text-[11px]"
                style="color: var(--ink-text-tertiary)"
              >
                <span>{{ formatDate(article.published_at || article.created_at) }}</span>
                <span v-if="article.author">· {{ article.author }}</span>
              </div>

              <!-- Summary -->
              <p
                class="mt-1.5 text-xs leading-relaxed line-clamp-2"
                style="color: var(--ink-text-secondary)"
              >
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

.article-row {
  background: transparent;
  border-bottom: 1px solid var(--ink-border-light);
}

.article-row:hover {
  background: var(--ink-white);
}

.article-row--active {
  background: var(--ink-white);
}

.star-btn {
  color: var(--ink-text-tertiary);
  opacity: 0;
}

.star-btn--active {
  color: var(--ink-accent);
  opacity: 1;
}

.group:hover .star-btn {
  opacity: 1;
}
</style>
