<script setup lang="ts">
import { ref, onMounted } from 'vue'
import CategoryList from './CategoryList.vue'
import RefreshProgress from './RefreshProgress.vue'
import Settings from './Settings.vue'
import { useFeeds } from '@/composables/useFeeds'
import { useCategories } from '@/composables/useCategories'
import { useToast } from '@/composables/useToast'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { feeds, loading, init, refresh, deleteFeed, refreshFeed } = useFeeds()
const { categories, init: initCategories, refresh: refreshCategories } = useCategories()
const { showSuccess, showError } = useToast()

const showRefreshProgress = ref(false)
const isRefreshing = ref(false)
const showSettings = ref(false)
const selectedFeedId = ref<number | undefined>(undefined)

onMounted(async () => {
  await init()
  await initCategories()
})

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
}

const handleFeedRefresh = async (feedId: number) => {
  await refreshFeed(feedId)
}
</script>

<template>
  <div
    class="h-full flex flex-col ink-dark-scroll overflow-y-auto"
    style="background: var(--ink-dark)"
  >
    <!-- Header -->
    <div class="px-5 pt-5 pb-4">
      <h1
        class="text-lg font-semibold tracking-wide"
        style="color: var(--ink-text-inverse); font-family: 'Playfair Display', Georgia, serif"
      >
        SCX RSS
      </h1>
      <p class="text-sm mt-0.5" style="color: var(--ink-text-inverse-muted)">
        {{ feeds.length }} 个订阅源
      </p>

      <!-- Actions -->
      <div class="flex gap-2 mt-3">
        <button
          :disabled="isRefreshing || feeds.length === 0"
          class="btn-ink flex-1 text-sm py-2 gap-1.5"
          @click="startBatchRefresh"
        >
          <svg
            class="w-3.5 h-3.5"
            :class="{ 'animate-spin': isRefreshing }"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
            />
          </svg>
          {{ isRefreshing ? '刷新中' : '全部刷新' }}
        </button>
      </div>
    </div>

    <!-- Divider -->
    <div style="height: 1px; background: var(--ink-dark-hover)"></div>

    <!-- Feed / Category List -->
    <div class="flex-1 overflow-y-auto py-2 ink-dark-scroll">
      <CategoryList
        :categories="categories"
        :feeds="feeds"
        :loading="loading"
        :selected-feed-id="selectedFeedId"
        @feed-selected="handleFeedSelected"
        @feed-delete="handleFeedDelete"
        @feed-refresh="handleFeedRefresh"
      />
    </div>

    <!-- Bottom bar -->
    <div style="border-top: 1px solid var(--ink-dark-hover)" class="p-3">
      <button class="btn-ghost-dark text-sm" @click="showSettings = true">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="1.5"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="1.5"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        设置
      </button>
    </div>

    <RefreshProgress
      :show="showRefreshProgress"
      @complete="onRefreshComplete"
      @close="closeRefreshProgress"
    />
    <Settings :show="showSettings" @close="showSettings = false" />
  </div>
</template>
