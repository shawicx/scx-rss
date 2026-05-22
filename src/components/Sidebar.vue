<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import CategoryList from './CategoryList.vue'
import RefreshProgress from './RefreshProgress.vue'
import Settings from './Settings.vue'
import { useFeeds } from '@/composables/useFeeds'
import { useCategories } from '@/composables/useCategories'
import { useToast } from '@/composables/useToast'
import { useI18n } from '@/composables/useI18n'
import { invoke } from '@tauri-apps/api/core'
const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { feeds, loading, init, refresh, deleteFeed, refreshFeed } = useFeeds()
const { categories, init: initCategories, refresh: refreshCategories } = useCategories()
const { showSuccess, showError } = useToast()
const { t } = useI18n()

const showRefreshProgress = ref(false)
const isRefreshing = ref(false)
const showSettings = ref(false)
const selectedFeedId = ref<number | undefined>(undefined)

onMounted(async () => {
  await init()
  await initCategories()
  window.addEventListener('feeds-refreshed', onAutoRefreshComplete)
})

onUnmounted(() => {
  window.removeEventListener('feeds-refreshed', onAutoRefreshComplete)
})

const onAutoRefreshComplete = () => {
  refresh()
  refreshCategories()
}

const startBatchRefresh = async () => {
  if (isRefreshing.value || feeds.value.length === 0) return
  isRefreshing.value = true
  showRefreshProgress.value = true
  try {
    await invoke('batch_refresh_feeds')
  } catch (error) {
    showError(`批量刷新失败: ${error}`)
    showRefreshProgress.value = false
  }
}

const onRefreshComplete = (success: number, failed: number, newArticles: number) => {
  isRefreshing.value = false
  showRefreshProgress.value = false
  refresh()
  refreshCategories()
  if (failed === 0) {
    showSuccess(`刷新完成！${success} 个源，${newArticles} 篇新文章`)
  } else {
    showError(`刷新完成：${success} 成功，${failed} 失败，${newArticles} 篇新文章`)
  }
}

const closeRefreshProgress = () => {
  showRefreshProgress.value = false
  isRefreshing.value = false
}

const handleFeedSelected = (feedId: number) => {
  selectedFeedId.value = feedId
  emit('feed-selected', feedId)
}

const handleFeedDelete = async (feedId: number) => {
  const success = await deleteFeed(feedId)
  if (success && selectedFeedId.value === feedId) {
    selectedFeedId.value = undefined
  }
  refreshCategories()
}

const handleFeedRefresh = async (feedId: number) => {
  await refreshFeed(feedId)
}
</script>

<template>
  <v-sheet
    color="surface-variant"
    class="d-flex flex-column h-100 overflow-hidden"
  >
    <!-- Header -->
    <div class="pa-5 pb-4">
      <h1 class="text-h6 font-weight-bold tracking-wide">SCX RSS</h1>
      <p class="text-body-2 mt-1 text-medium-emphasis">{{ feeds.length }} {{ $t('feeds.allFeeds') }}</p>

      <!-- Actions -->
      <v-btn
        :loading="isRefreshing"
        :disabled="feeds.length === 0"
        color="primary"
        block
        class="mt-3"
        size="large"
        prepend-icon="mdi-refresh"
        @click="startBatchRefresh"
      >
        {{ isRefreshing ? $t('refresh.refreshing') : $t('refresh.refreshAll') }}
      </v-btn>
    </div>

    <v-divider />

    <!-- Feed / Category List -->
    <div class="flex-1-1 overflow-y-auto py-2">
      <CategoryList
        :categories="categories"
        :feeds="feeds"
        :loading="loading"
        :selected-feed-id="selectedFeedId"
        @feed-selected="handleFeedSelected"
        @feed-delete="handleFeedDelete"
        @feed-refresh="handleFeedRefresh"
        @feed-updated="
          () => {
            refresh()
            refreshCategories()
          }
        "
      />
    </div>

    <!-- Bottom bar -->
    <v-divider />
    <div class="pa-3">
      <v-btn variant="text" block prepend-icon="mdi-cog" @click="showSettings = true" size="large">
        {{ $t('settings.title') }}
      </v-btn>
    </div>

    <RefreshProgress
      :show="showRefreshProgress"
      @complete="onRefreshComplete"
      @close="closeRefreshProgress"
    />
    <Settings
      :show="showSettings"
      @close="showSettings = false"
      @data-restored="
        () => {
          init()
          initCategories()
        }
      "
      @feeds-changed="
        () => {
          refresh()
          refreshCategories()
        }
      "
    />
  </v-sheet>
</template>

<style scoped>
.flex-1-1 {
  flex: 1 1 0;
  min-height: 0;
}
</style>
