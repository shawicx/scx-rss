<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Sidebar from '@/components/Sidebar.vue'
import ArticleList from '@/components/ArticleList.vue'
import ArticleView from '@/components/ArticleView.vue'
import ToastContainer from '@/components/ToastContainer.vue'
import type { Article } from '@/types'
import { useToast } from '@/composables/useToast'
import { useTheme } from '@/composables/useTheme'

const { showError } = useToast()
useTheme()

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
  <v-app>
    <div style="display: flex; height: 100vh; overflow: hidden">
      <!-- Sidebar -->
      <aside class="sidebar-panel">
        <Sidebar v-if="isInitialized" @feed-selected="handleFeedSelected" />
      </aside>

      <!-- Article List -->
      <section class="list-panel">
        <ArticleList
          v-if="isInitialized"
          :feed-id="currentFeedId"
          @article-selected="handleArticleSelected"
        />
      </section>

      <!-- Article Reader -->
      <main class="reader-panel">
        <ArticleView :article="currentArticle" />
      </main>

      <ToastContainer />
    </div>
  </v-app>
</template>

<style scoped>
.sidebar-panel {
  width: 24rem;
  flex-shrink: 0;
  height: 100%;
}

.list-panel {
  width: 360px;
  flex-shrink: 0;
  height: 100%;
  border-left: 1px solid rgb(var(--v-theme-surface-variant));
}

.reader-panel {
  flex: 1;
  min-width: 0;
  height: 100%;
}
</style>
