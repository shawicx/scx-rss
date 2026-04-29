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
  return html
}
</script>

<template>
  <div class="h-full flex flex-col" style="background: var(--ink-paper-bright)">
    <!-- Empty State -->
    <div v-if="!currentArticle" class="flex flex-col items-center justify-center h-full">
      <div
        class="w-16 h-16 rounded-full flex items-center justify-center mb-4"
        style="background: var(--ink-accent-subtle)"
      >
        <svg
          class="w-7 h-7"
          style="color: var(--ink-accent)"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="1.5"
            d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
          />
        </svg>
      </div>
      <p class="text-sm font-medium" style="color: var(--ink-text-secondary)">
        选择一篇文章开始阅读
      </p>
    </div>

    <!-- Article Content -->
    <div v-else class="flex-1 overflow-y-auto">
      <!-- Toolbar -->
      <div class="toolbar-sticky sticky top-0 z-10 px-8 py-3 flex items-center justify-between">
        <div class="flex items-center gap-1">
          <!-- Star -->
          <button
            class="toolbar-btn w-8 h-8 flex items-center justify-center rounded-md"
            :class="{ 'toolbar-btn--starred': currentArticle.is_starred }"
            title="收藏"
            @click="handleToggleStar"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path
                d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"
              />
            </svg>
          </button>

          <!-- Mark Unread -->
          <button
            v-if="currentArticle.is_read"
            class="toolbar-btn w-8 h-8 flex items-center justify-center rounded-md"
            title="标记未读"
            @click="handleMarkAsUnread"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M5 13l4 4L19 7"
              />
            </svg>
          </button>

          <!-- Open in browser -->
          <button
            class="toolbar-btn w-8 h-8 flex items-center justify-center rounded-md"
            title="在浏览器中打开"
            @click="openInBrowser"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
              />
            </svg>
          </button>
        </div>

        <!-- Status Tags -->
        <div class="flex items-center gap-1.5">
          <span
            v-if="currentArticle.is_starred"
            class="px-2 py-0.5 rounded text-[10px] font-medium"
            style="background: var(--ink-accent-subtle); color: var(--ink-accent)"
            >已收藏</span
          >
          <span
            v-if="currentArticle.is_read"
            class="px-2 py-0.5 rounded text-[10px] font-medium"
            style="background: var(--ink-paper); color: var(--ink-text-tertiary)"
            >已读</span
          >
        </div>
      </div>

      <!-- Article Body -->
      <article class="px-8 py-8 mx-auto">
        <!-- Title -->
        <h1 class="article-title text-2xl font-bold leading-tight mb-5">
          {{ currentArticle.title }}
        </h1>

        <!-- Meta -->
        <div class="article-meta flex flex-wrap items-center gap-3 text-xs pb-6 mb-6">
          <span v-if="currentArticle.author" class="flex items-center gap-1">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>
            {{ currentArticle.author }}
          </span>
          <span class="flex items-center gap-1">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
            {{ formatDate(currentArticle.published_at || currentArticle.created_at) }}
          </span>
        </div>

        <!-- Content -->
        <div class="prose-ink" v-html="renderContent()" />

        <!-- Footer -->
        <div class="mt-10 pt-6" style="border-top: 1px solid var(--ink-border-light)">
          <a
            :href="currentArticle.link"
            target="_blank"
            rel="noopener noreferrer"
            class="article-link inline-flex items-center gap-1.5 text-xs font-medium"
          >
            阅读原文
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
              />
            </svg>
          </a>
        </div>
      </article>
    </div>
  </div>
</template>

<style scoped>
.toolbar-sticky {
  background: rgba(250, 248, 244, 0.92);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--ink-border-light);
}

.toolbar-btn {
  color: var(--ink-text-tertiary);
  background: transparent;
  transition: all 150ms ease;
}
.toolbar-btn:hover {
  background: var(--ink-accent-subtle);
  color: var(--ink-accent);
}
.toolbar-btn--starred {
  color: var(--ink-accent);
  background: var(--ink-accent-subtle);
}

.article-title {
  color: var(--ink-text);
  font-family: 'Playfair Display', 'Georgia', 'Noto Serif SC', serif;
}

.article-meta {
  color: var(--ink-text-tertiary);
  border-bottom: 1px solid var(--ink-border-light);
}

.article-link {
  color: var(--ink-accent);
  transition: color 150ms ease;
}
.article-link:hover {
  color: var(--ink-accent-hover);
}
</style>
