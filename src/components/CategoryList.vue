<script setup lang="ts">
import { ref } from 'vue'
import type { Category, Feed } from '@/types'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { validateFeedUrl } from '@/utils/validators'

interface Props {
  categories: Category[]
  feeds: Feed[]
  loading: boolean
  selectedFeedId?: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
  (e: 'feed-delete', feedId: number): void
  (e: 'feed-refresh', feedId: number): void
}>()

const { addFeed } = useFeeds()
const { showError } = useToast()

const collapsedState = ref<Record<string, boolean>>({})
const showAddDialog = ref(false)
const newFeedUrl = ref('')
const newFeedCategory = ref('')
const adding = ref(false)

const toggleCollapse = (categoryName: string) => {
  collapsedState.value[categoryName] = !collapsedState.value[categoryName]
}

const isCollapsed = (categoryName: string): boolean => {
  return collapsedState.value[categoryName] || false
}

const getFeedsByCategory = (categoryName: string | null): Feed[] => {
  if (categoryName === null) {
    return props.feeds.filter(feed => !feed.category)
  }
  return props.feeds.filter(feed => feed.category === categoryName)
}

const handleSelectFeed = (feedId: number) => {
  emit('feed-selected', feedId)
}

const handleDeleteFeed = (feedId: number, feedTitle: string) => {
  const confirmed = confirm(`确定要删除 "${feedTitle}" 吗？`)
  if (confirmed) emit('feed-delete', feedId)
}

const handleRefreshFeed = (feedId: number) => {
  emit('feed-refresh', feedId)
}

const openAddDialog = () => {
  newFeedUrl.value = ''
  newFeedCategory.value = ''
  showAddDialog.value = true
}

const handleAddFeed = async () => {
  const validation = validateFeedUrl(newFeedUrl.value)
  if (!validation.valid) {
    showError(validation?.error)
    return
  }
  adding.value = true
  const success = await addFeed(newFeedUrl.value, newFeedCategory.value || undefined)
  adding.value = false
  if (success) showAddDialog.value = false
}
</script>

<template>
  <div class="px-3">
    <!-- Add Feed Button -->
    <button :disabled="loading" class="btn-ghost-dark text-sm py-2 mb-1" @click="openAddDialog">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      添加订阅源
    </button>

    <!-- Empty state -->
    <div
      v-if="categories.length === 0 && !loading"
      class="py-8 text-center"
      style="color: var(--ink-text-inverse-muted)"
    >
      <p class="text-sm">暂无分类</p>
    </div>

    <!-- Categories -->
    <div v-else>
      <div v-for="category in categories" :key="category.name">
        <!-- Category Header -->
        <button
          class="cat-header w-full flex items-center gap-2 px-2 py-2 rounded text-left transition-colors duration-150"
          @click="toggleCollapse(category.name)"
        >
          <svg
            class="w-3 h-3 flex-shrink-0 transition-transform duration-150"
            :class="{ '-rotate-90': isCollapsed(category.name) }"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 9l-7 7-7-7"
            />
          </svg>
          <span class="text-sm font-medium tracking-wide uppercase flex-1 truncate">{{
            category.name
          }}</span>
          <span
            v-if="category.unread_count > 0"
            class="badge-count text-xs font-semibold px-2 py-0.5 rounded-full"
          >
            {{ category.unread_count }}
          </span>
        </button>

        <!-- Feeds under category -->
        <div v-if="!isCollapsed(category.name)" class="ml-1">
          <div
            v-for="feed in getFeedsByCategory(category.name)"
            :key="feed.id"
            class="feed-item group flex items-center gap-2.5 px-2.5 py-2 rounded cursor-pointer transition-colors duration-150"
            :class="{ 'feed-item--active': selectedFeedId === feed.id }"
            @click="handleSelectFeed(feed.id)"
          >
            <!-- Feed initial -->
            <div
              class="feed-avatar w-6 h-6 rounded flex-shrink-0 flex items-center justify-center text-xs font-bold"
            >
              {{ feed.title.charAt(0).toUpperCase() }}
            </div>

            <span class="flex-1 text-sm truncate">{{ feed.title }}</span>

            <!-- Actions -->
            <div class="feed-actions flex items-center gap-0.5">
              <button
                class="action-btn w-6 h-6 flex items-center justify-center rounded"
                title="刷新"
                @click.stop="handleRefreshFeed(feed.id)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                  />
                </svg>
              </button>
              <button
                class="action-btn action-btn--danger w-6 h-6 flex items-center justify-center rounded"
                title="删除"
                @click.stop="handleDeleteFeed(feed.id, feed.title)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
          </div>

          <div
            v-if="getFeedsByCategory(category.name).length === 0"
            class="px-3 py-2 text-sm"
            style="color: var(--ink-text-inverse-muted)"
          >
            暂无订阅源
          </div>
        </div>
      </div>
    </div>

    <!-- Add Feed Modal -->
    <div v-if="showAddDialog" class="modal-overlay" @click.self="showAddDialog = false">
      <div class="modal-content">
        <div class="px-5 pt-5 pb-3">
          <h3 class="text-sm font-semibold" style="color: var(--ink-text)">添加订阅源</h3>
        </div>

        <div class="px-5 space-y-3">
          <div>
            <label class="block text-xs font-medium mb-1" style="color: var(--ink-text-secondary)">
              Feed URL <span style="color: var(--ink-error)">*</span>
            </label>
            <input
              v-model="newFeedUrl"
              type="url"
              placeholder="https://example.com/feed.xml"
              class="input-ink text-xs"
              @keypress.enter="handleAddFeed"
            />
          </div>
          <div>
            <label class="block text-xs font-medium mb-1" style="color: var(--ink-text-secondary)">
              分类（可选）
            </label>
            <input
              v-model="newFeedCategory"
              type="text"
              placeholder="例如：技术、新闻"
              class="input-ink text-xs"
              @keypress.enter="handleAddFeed"
            />
          </div>
        </div>

        <div
          class="flex gap-2 px-5 py-4 mt-2"
          style="border-top: 1px solid var(--ink-border-light)"
        >
          <button class="btn-ghost flex-1 text-xs" @click="showAddDialog = false">取消</button>
          <button
            :disabled="adding || !newFeedUrl"
            class="btn-ink flex-1 text-xs"
            @click="handleAddFeed"
          >
            {{ adding ? '添加中...' : '添加' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cat-header {
  color: var(--ink-text-inverse-muted);
  background: transparent;
}
.cat-header:hover {
  background: var(--ink-dark-hover);
  color: var(--ink-text-inverse);
}

.badge-count {
  background: var(--ink-accent-subtle);
  color: var(--ink-accent);
}

.feed-item {
  color: var(--ink-text-inverse-muted);
  background: transparent;
}
.feed-item:hover {
  background: var(--ink-dark-hover);
}
.feed-item--active {
  background: var(--ink-dark-active);
  color: var(--ink-accent);
}
.feed-item--active:hover {
  background: var(--ink-dark-active);
}

.feed-avatar {
  background: var(--ink-dark-hover);
  color: var(--ink-text-inverse-muted);
}
.feed-item--active .feed-avatar {
  background: var(--ink-accent);
  color: #fff;
}

.feed-actions {
  opacity: 0;
  transition: opacity 100ms;
}
.group:hover .feed-actions {
  opacity: 1;
}

.action-btn {
  color: var(--ink-text-inverse-muted);
  transition: all 100ms;
}
.action-btn:hover {
  background: var(--ink-dark-hover);
  color: var(--ink-accent);
}
.action-btn--danger:hover {
  background: var(--ink-error);
  color: #fff;
}
</style>
