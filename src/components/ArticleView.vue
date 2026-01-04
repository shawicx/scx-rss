<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Article } from '@/types'
import { useArticles } from '@/composables/useArticles'
import { formatDate } from '@/utils/formatters'

/**
 * 文章阅读器组件
 * 渲染文章完整内容，提供状态标记和跳转功能
 */
interface Props {
  article?: Article
}

const props = defineProps<Props>()

const { toggleRead, toggleStar } = useArticles()

// 本地文章状态
const currentArticle = ref<Article | undefined>(props.article)

/**
 * 监听 article 变化
 */
watch(() => props.article, (newArticle) => {
  currentArticle.value = newArticle
})

/**
 * 标记为未读
 */
const handleMarkAsUnread = async () => {
  if (currentArticle.value) {
    await toggleRead(currentArticle.value.id, false)
    currentArticle.value.is_read = false
  }
}

/**
 * 切换收藏状态
 */
const handleToggleStar = async () => {
  if (currentArticle.value) {
    await toggleStar(currentArticle.value.id)
  }
}

/**
 * 在浏览器中打开原文链接
 */
const openInBrowser = () => {
  if (currentArticle.value) {
    window.open(currentArticle.value.link, '_blank')
  }
}

/**
 * 渲染文章内容（处理 HTML）
 */
const renderContent = (): string => {
  if (!currentArticle.value) return ''

  // 优先使用 content，如果没有则使用 description
  const html = currentArticle.value.content || currentArticle.value.description || ''

  // 简单的 HTML 清理和增强
  return html
    .replace(/<img/g, '<img class="max-w-full h-auto rounded"')
    .replace(/<a /g, '<a target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:text-blue-800 underline" ')
    .replace(/<h1>/g, '<h1 class="text-2xl font-bold mb-4 mt-6">')
    .replace(/<h2>/g, '<h2 class="text-xl font-bold mb-3 mt-5">')
    .replace(/<h3>/g, '<h3 class="text-lg font-bold mb-2 mt-4">')
    .replace(/<p>/g, '<p class="mb-4 leading-relaxed">')
    .replace(/<pre>/g, '<pre class="bg-gray-100 p-4 rounded overflow-x-auto">')
    .replace(/<code>/g, '<code class="bg-gray-100 px-1 py-0.5 rounded text-sm">')
    .replace(/<blockquote>/g, '<blockquote class="border-l-4 border-gray-300 pl-4 italic">')
    .replace(/<ul>/g, '<ul class="list-disc list-inside mb-4">')
    .replace(/<ol>/g, '<ol class="list-decimal list-inside mb-4">')
}
</script>

<template>
  <div class="h-full flex flex-col bg-white">
    <!-- 空状态 -->
    <div
      v-if="!currentArticle"
      class="flex flex-col items-center justify-center h-full text-gray-500"
    >
      <svg class="w-24 h-24 mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
      </svg>
      <p class="text-lg">选择一篇文章开始阅读</p>
    </div>

    <!-- 文章内容 -->
    <div v-else class="flex-1 overflow-y-auto">
      <!-- 工具栏 -->
      <div class="sticky top-0 z-10 bg-white border-b border-gray-200 px-8 py-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <!-- 收藏按钮 -->
            <button
              @click="handleToggleStar"
              class="p-2 rounded transition-colors"
              :class="currentArticle.is_starred ? 'text-yellow-500 bg-yellow-50' : 'text-gray-500 hover:bg-gray-100'"
              title="收藏"
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
              </svg>
            </button>

            <!-- 标记未读按钮 -->
            <button
              v-if="currentArticle.is_read"
              @click="handleMarkAsUnread"
              class="p-2 text-gray-500 rounded hover:bg-gray-100 transition-colors"
              title="标记为未读"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </button>

            <!-- 打开原文按钮 -->
            <button
              @click="openInBrowser"
              class="p-2 text-gray-500 rounded hover:bg-gray-100 transition-colors"
              title="在浏览器中打开"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
              </svg>
            </button>
          </div>

          <!-- 状态标签 -->
          <div class="flex items-center gap-2 text-sm">
            <span
              v-if="currentArticle.is_starred"
              class="px-2 py-1 bg-yellow-100 text-yellow-800 rounded"
            >
              已收藏
            </span>
            <span
              v-if="currentArticle.is_read"
              class="px-2 py-1 bg-gray-100 text-gray-600 rounded"
            >
              已读
            </span>
          </div>
        </div>
      </div>

      <!-- 文章内容区 -->
      <article class="px-8 py-6 max-w-4xl">
        <!-- 标题 -->
        <h1 class="text-3xl font-bold text-gray-900 mb-4">
          {{ currentArticle.title }}
        </h1>

        <!-- 元信息 -->
        <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500 mb-8 pb-6 border-b border-gray-200">
          <div class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            <span v-if="currentArticle.author">{{ currentArticle.author }}</span>
            <span v-else>未知作者</span>
          </div>
          <div class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            <span>{{ formatDate(currentArticle.published_at || currentArticle.created_at) }}</span>
          </div>
        </div>

        <!-- 文章正文 -->
        <div
          class="prose prose-lg max-w-none"
          v-html="renderContent()"
        />

        <!-- 原文链接 -->
        <div class="mt-8 pt-6 border-t border-gray-200">
          <a
            :href="currentArticle.link"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-2 text-blue-600 hover:text-blue-800 transition-colors"
          >
            <span>阅读原文</span>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
          </a>
        </div>
      </article>
    </div>
  </div>
</template>

<style scoped>
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

/* 文章内容样式 */
.prose :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
  margin: 1.5rem 0;
}

.prose :deep(p) {
  margin-bottom: 1rem;
  line-height: 1.75;
}

.prose :deep(a) {
  color: #2563eb;
  text-decoration: underline;
}

.prose :deep(a:hover) {
  color: #1d4ed8;
}

.prose :deep(pre) {
  background-color: #f3f4f6;
  padding: 1rem;
  border-radius: 8px;
  overflow-x: auto;
}

.prose :deep(code) {
  background-color: #f3f4f6;
  padding: 0.125rem 0.25rem;
  border-radius: 4px;
  font-size: 0.875rem;
}

.prose :deep(blockquote) {
  border-left: 4px solid #d1d5db;
  padding-left: 1rem;
  font-style: italic;
  color: #6b7280;
}

.prose :deep(ul), .prose :deep(ol) {
  margin-bottom: 1rem;
  padding-left: 1.5rem;
}

.prose :deep(h1), .prose :deep(h2), .prose :deep(h3) {
  font-weight: bold;
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
}

.prose :deep(h1) {
  font-size: 2rem;
}

.prose :deep(h2) {
  font-size: 1.5rem;
}

.prose :deep(h3) {
  font-size: 1.25rem;
}
</style>
