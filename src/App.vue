<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Sidebar from '@/components/Sidebar.vue'
import ArticleList from '@/components/ArticleList.vue'
import ArticleView from '@/components/ArticleView.vue'
import ToastContainer from '@/components/ToastContainer.vue'
import ResizeHandle from '@/components/ResizeHandle.vue'
import type { Article } from '@/types'
import { useToast } from '@/composables/useToast'
import { useTheme } from '@/composables/useTheme'
import { useResizable } from '@/composables/useResizable'
import { useI18n } from '@/composables/useI18n'
import { useAutoUpdate } from '@/composables/useAutoUpdate'
import UpdateDialog from '@/components/UpdateDialog.vue'
import { STORAGE_KEYS } from '@/utils/constants'

const { showError } = useToast()
useTheme()
const { init } = useI18n()
const { startCheck } = useAutoUpdate()

const currentFeedId = ref<number | undefined>(undefined)
const currentArticle = ref<Article | undefined>(undefined)
const isInitialized = ref(false)

const { width: sidebarWidth, isResizing: isSidebarResizing, onDragStart: onSidebarDragStart } = useResizable({
  storageKey: STORAGE_KEYS.SIDEBAR_WIDTH,
  defaultWidth: 384,
  minWidth: 200,
  maxWidth: 480,
})

const { width: listWidth, isResizing: isListResizing, onDragStart: onListDragStart } = useResizable({
  storageKey: STORAGE_KEYS.ARTICLE_LIST_WIDTH,
  defaultWidth: 360,
  minWidth: 240,
  maxWidth: 560,
})

onMounted(async () => {
  try {
    await invoke('init_db')
    await init()
    startCheck()
    isInitialized.value = true
  } catch (error) {
    console.error('数据库初始化失败:', error)
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
      <aside class="sidebar-panel" :style="{ width: sidebarWidth + 'px' }">
        <Sidebar v-if="isInitialized" @feed-selected="handleFeedSelected" />
      </aside>

      <ResizeHandle
        :is-resizing="isSidebarResizing"
        @drag-start="onSidebarDragStart"
      />

      <!-- Article List -->
      <section class="list-panel" :style="{ width: listWidth + 'px' }">
        <ArticleList
          v-if="isInitialized"
          :feed-id="currentFeedId"
          @article-selected="handleArticleSelected"
        />
      </section>

      <ResizeHandle
        :is-resizing="isListResizing"
        @drag-start="onListDragStart"
      />

      <!-- Article Reader -->
      <main class="reader-panel">
        <ArticleView :article="currentArticle" />
      </main>

      <UpdateDialog />
      <ToastContainer />
    </div>
  </v-app>
</template>

<style scoped>
.sidebar-panel {
  flex-shrink: 0;
  height: 100%;
  overflow: hidden;
}

.list-panel {
  flex-shrink: 0;
  height: 100%;
  overflow: hidden;
  border-left: 1px solid rgb(var(--v-theme-surface-variant));
}

.reader-panel {
  flex: 1;
  min-width: 0;
  height: 100%;
}
</style>
