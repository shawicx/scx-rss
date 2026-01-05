<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Sidebar from '@/components/Sidebar.vue'
import ArticleList from '@/components/ArticleList.vue'
import ArticleView from '@/components/ArticleView.vue'
import ToastContainer from '@/components/ToastContainer.vue'
import type { Article, Feed } from '@/types'
import { useToast } from '@/composables/useToast'

/**
 * SCX RSS Reader 主应用
 * 包含侧边栏、文章列表和文章阅读器的三栏布局
 */
const { showSuccess, showError } = useToast()

// 当前选中的 Feed ID
const currentFeedId = ref<number | undefined>(undefined)

// 当前选中的文章
const currentArticle = ref<Article | undefined>(undefined)

// 数据库初始化状态
const isInitialized = ref(false)

/**
 * 初始化应用
 */
onMounted(async () => {
  try {
    // 初始化数据库
    await invoke('init_db')
    isInitialized.value = true
    console.log('Database initialized successfully')
  } catch (error) {
    console.error('Failed to initialize database:', error)
    showError(`数据库初始化失败: ${error}`)
  }
})

/**
 * 处理 Feed 选择
 */
const handleFeedSelected = (feedId: number) => {
  currentFeedId.value = feedId
  currentArticle.value = undefined // 清除当前选中的文章
}

/**
 * 处理文章选择
 */
const handleArticleSelected = (article: Article) => {
  currentArticle.value = article
}
</script>

<template>
  <div class="h-screen flex overflow-hidden bg-gray-50">
    <!-- 侧边栏 -->
    <aside class="w-80 flex-shrink-0">
      <Sidebar v-if="isInitialized" @feed-selected="handleFeedSelected" />
    </aside>

    <!-- 文章列表 -->
    <section class="w-96 flex-shrink-0 border-r border-gray-200">
      <ArticleList
        v-if="isInitialized"
        :feed-id="currentFeedId"
        @article-selected="handleArticleSelected"
      />
    </section>

    <!-- 文章阅读器 -->
    <main class="flex-1 overflow-hidden">
      <ArticleView :article="currentArticle" />
    </main>

    <!-- Toast 通知容器 -->
    <ToastContainer />
  </div>
</template>

<style scoped>
/* 全局样式 */
</style>
