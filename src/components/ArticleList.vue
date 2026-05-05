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

const {
  articles,
  loading,
  unreadCount,
  starredCount,
  loadArticlesByFeed,
  toggleRead,
  toggleStar,
  showUnreadOnly,
  showStarredOnly,
  showAll,
} = useArticles()

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
    await showUnreadOnly()
  } else if (mode === 'starred') {
    await showStarredOnly()
  } else {
    await showAll()
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
  },
})
</script>

<template>
  <div class="d-flex flex-column h-100 bg-surface">
    <!-- Filter Bar -->
    <div
      class="d-flex align-center px-4 py-3"
      style="border-bottom: 1px solid rgb(var(--v-theme-surface-variant))"
    >
      <v-btn-toggle
        :model-value="filterMode"
        @update:model-value="setFilterMode"
        mandatory
        divided
        density="compact"
        variant="outlined"
        color="primary"
        rounded="sm"
      >
        <v-btn size="small" value="all">全部</v-btn>
        <v-btn size="small" value="unread">未读 {{ unreadCount }}</v-btn>
        <v-btn size="small" value="starred">收藏 {{ starredCount }}</v-btn>
      </v-btn-toggle>
    </div>

    <!-- Article List -->
    <div class="flex-1-1 overflow-y-auto">
      <!-- Empty State -->
      <div
        v-if="articles.length === 0 && !loading"
        class="d-flex flex-column align-center justify-center h-100 px-6"
      >
        <v-icon size="48" class="mb-3 text-medium-emphasis">mdi-newspaper-variant-outline</v-icon>
        <p class="text-body-2 text-medium-emphasis">暂无文章</p>
        <p class="text-caption text-medium-emphasis mt-1">
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
      <div v-if="loading" class="d-flex align-center justify-center h-100">
        <v-progress-circular indeterminate size="24" class="text-medium-emphasis" />
      </div>

      <!-- Articles -->
      <v-list v-else density="compact" class="py-0" :lines="false">
        <v-list-item
          v-for="article in articles"
          :key="article.id"
          :active="selectedArticleId === article.id"
          color="primary"
          class="article-row px-2 py-3"
          @click="handleSelectArticle(article)"
        >
          <template v-slot:prepend>
            <div class="d-flex align-start pt-1 mr-3">
              <v-badge dot :color="article.is_read ? 'transparent' : 'primary'" inline />
            </div>
          </template>

          <v-list-item-title
            class="text-body-2"
            :class="{
              'font-weight-medium': !article.is_read,
              'text-medium-emphasis': article.is_read,
            }"
          >
            {{ article.title }}
          </v-list-item-title>

          <v-list-item-subtitle class="mt-1">
            <div class="d-flex align-center text-caption text-medium-emphasis">
              <span>{{ formatDate(article.published_at || article.created_at) }}</span>
              <span v-if="article.author" class="ml-1">· {{ article.author }}</span>
            </div>
            <p class="text-caption text-medium-emphasis mt-1 line-clamp-2">
              {{ getArticleSummary(article) }}
            </p>
          </v-list-item-subtitle>

          <template v-slot:append>
            <v-btn
              icon
              variant="text"
              size="small"
              @click.stop="handleToggleStar(article.id, $event)"
            >
              <v-icon size="16" :color="article.is_starred ? 'warning' : undefined">
                {{ article.is_starred ? 'mdi-star' : 'mdi-star-outline' }}
              </v-icon>
            </v-btn>
          </template>
        </v-list-item>
      </v-list>
    </div>
  </div>
</template>

<style scoped>
.flex-1-1 {
  flex: 1 1 0;
  min-height: 0;
}
.article-row {
  position: relative;
}
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
