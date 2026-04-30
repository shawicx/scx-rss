<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Article } from '@/types'
import { useArticles } from '@/composables/useArticles'
import { formatDate } from '@/utils/formatters'

interface Props {
  article?: Article
}

const props = defineProps<Props>()

const { toggleRead, toggleStar } = useArticles()
const currentArticle = ref<Article | undefined>(props.article)

watch(
  () => props.article,
  newArticle => {
    currentArticle.value = newArticle
  }
)

const handleMarkAsUnread = async () => {
  if (currentArticle.value) {
    await toggleRead(currentArticle.value.id)
    currentArticle.value.is_read = false
  }
}

const handleToggleStar = async () => {
  if (currentArticle.value) {
    await toggleStar(currentArticle.value.id)
  }
}

const openInBrowser = () => {
  if (currentArticle.value) {
    window.open(currentArticle.value.link, '_blank')
  }
}

const renderContent = (): string => {
  if (!currentArticle.value) return ''
  const html = currentArticle.value.content || currentArticle.value.description || ''
  return html.replace(/<a\s/g, '<a target="_blank" rel="noopener noreferrer" ')
}
</script>

<template>
  <div class="d-flex flex-column h-100">
    <!-- Empty State -->
    <div v-if="!currentArticle" class="d-flex flex-column align-center justify-center h-100">
      <v-sheet
        rounded="circle"
        class="d-flex align-center justify-center mb-4 pa-6 bg-surface-variant"
      >
        <v-icon size="28" color="primary">mdi-book-open-variant</v-icon>
      </v-sheet>
      <p class="text-body-1 font-weight-medium text-medium-emphasis">选择一篇文章开始阅读</p>
    </div>

    <!-- Article Content -->
    <div v-else class="flex-1-1 overflow-y-auto">
      <!-- Toolbar -->
      <v-toolbar density="compact" class="sticky-toolbar" color="background">
        <v-btn icon variant="text" size="small" @click="handleToggleStar">
          <v-icon :color="currentArticle.is_starred ? 'warning' : undefined">
            {{ currentArticle.is_starred ? 'mdi-star' : 'mdi-star-outline' }}
          </v-icon>
        </v-btn>

        <v-btn
          v-if="currentArticle.is_read"
          icon
          variant="text"
          size="small"
          @click="handleMarkAsUnread"
        >
          <v-icon>mdi-email-mark-as-unread</v-icon>
        </v-btn>

        <v-btn icon variant="text" size="small" @click="openInBrowser">
          <v-icon>mdi-open-in-new</v-icon>
        </v-btn>

        <v-spacer />

        <v-chip
          v-if="currentArticle.is_starred"
          size="small"
          color="warning"
          variant="tonal"
          class="mr-1"
        >
          已收藏
        </v-chip>
        <v-chip v-if="currentArticle.is_read" size="small" variant="tonal" class="mr-2">
          已读
        </v-chip>
      </v-toolbar>

      <!-- Article Body -->
      <article class="px-8 py-8 mx-auto">
        <!-- Title -->
        <h1 class="text-h5 font-weight-bold mb-5" style="line-height: 1.3">
          {{ currentArticle.title }}
        </h1>

        <!-- Meta -->
        <div
          class="d-flex flex-wrap align-center ga-3 text-caption text-medium-emphasis pb-6 mb-6 article-meta"
        >
          <span v-if="currentArticle.author" class="d-flex align-center ga-1">
            <v-icon size="14">mdi-account</v-icon>
            {{ currentArticle.author }}
          </span>
          <span class="d-flex align-center ga-1">
            <v-icon size="14">mdi-calendar</v-icon>
            {{ formatDate(currentArticle.published_at || currentArticle.created_at) }}
          </span>
        </div>

        <!-- Content -->
        <div class="prose-ink" v-html="renderContent()" />

        <!-- Footer -->
        <v-divider class="mt-10 mb-6" />
        <a
          :href="currentArticle.link"
          target="_blank"
          rel="noopener noreferrer"
          class="d-inline-flex align-center ga-2 text-caption font-weight-medium text-primary text-decoration-none"
        >
          阅读原文
          <v-icon size="12">mdi-open-in-new</v-icon>
        </a>
      </article>
    </div>
  </div>
</template>

<style scoped>
.flex-1-1 {
  flex: 1 1 0;
  min-height: 0;
}
.sticky-toolbar {
  position: sticky;
  top: 0;
  z-index: 10;
  backdrop-filter: blur(8px);
}
.article-meta {
  border-bottom: 1px solid rgb(var(--v-theme-surface-variant));
}
</style>
