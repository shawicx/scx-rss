<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Sidebar from '@/components/Sidebar.vue'
import ArticleList from '@/components/ArticleList.vue'
import ArticleView from '@/components/ArticleView.vue'
import ToastContainer from '@/components/ToastContainer.vue'
import type { Article } from '@/types'
import { useToast } from '@/composables/useToast'

const { showError } = useToast()

const currentFeedId = ref<number | undefined>(undefined)
const currentArticle = ref<Article | undefined>(undefined)
const isInitialized = ref(false)

onMounted(async () => {
  try {
    await invoke('init_db')
    isInitialized.value = true
  } catch (error) {
    console.error('Failed to initialize database:', error)
    showError(`数据库初始化失败: ${error}`)
  }
})

const handleFeedSelected = (feedId: number) => {
  currentFeedId.value = feedId
  currentArticle.value = undefined
}

const handleArticleSelected = (article: Article) => {
  currentArticle.value = article
}
</script>

<template>
  <div class="h-screen w-screen flex overflow-hidden">
    <!-- Sidebar -->
    <aside class="flex-shrink-0 h-full" style="width: var(--ink-sidebar-w)">
      <Sidebar v-if="isInitialized" @feed-selected="handleFeedSelected" />
    </aside>

    <!-- Article List -->
    <section class="flex-shrink-0 h-full border-l" style="width: var(--ink-list-w); border-color: var(--ink-border)">
      <ArticleList
        v-if="isInitialized"
        :feed-id="currentFeedId"
        @article-selected="handleArticleSelected"
      />
    </section>

    <!-- Article Reader -->
    <main class="flex-1 h-full min-w-0" style="background: var(--ink-paper-bright)">
      <ArticleView :article="currentArticle" />
    </main>

    <ToastContainer />
  </div>
</template>
