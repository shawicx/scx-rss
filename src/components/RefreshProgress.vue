<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useToast } from '@/composables/useToast'

interface RefreshProgress {
  type: 'start' | 'progress' | 'feed-success' | 'feed-error' | 'complete' | 'cancelled'
  current?: number
  total?: number
  feed_title?: string
  new_articles?: number
  error?: string
  success?: number
  failed?: number
  total_new_articles?: number
}

interface Props {
  show: boolean
}

defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'complete', success: number, failed: number, newArticles: number): void
}>()

const { showError } = useToast()

const current = ref(0)
const total = ref(0)
const currentFeedTitle = ref('')
const successCount = ref(0)
const failureCount = ref(0)
const totalNewArticles = ref(0)
const isCancelling = ref(false)

const cancelRefresh = async () => {
  isCancelling.value = true
  try {
    await invoke('cancel_batch_refresh')
  } catch (error) {
    console.error('Failed to cancel refresh:', error)
    showError(`取消刷新失败: ${error}`)
    isCancelling.value = false
  }
}

let unlisten: (() => void) | null = null

onMounted(async () => {
  unlisten = await listen<RefreshProgress>('refresh-progress', event => {
    const progress = event.payload
    switch (progress.type) {
      case 'start':
        total.value = progress.total || 0
        current.value = 0
        successCount.value = 0
        failureCount.value = 0
        totalNewArticles.value = 0
        break
      case 'progress':
        current.value = progress.current || 0
        currentFeedTitle.value = progress.feed_title || ''
        break
      case 'feed-success':
        current.value = progress.current || 0
        currentFeedTitle.value = progress.feed_title || ''
        successCount.value++
        totalNewArticles.value += progress.new_articles || 0
        break
      case 'feed-error':
        current.value = progress.current || 0
        currentFeedTitle.value = progress.feed_title || ''
        failureCount.value++
        break
      case 'complete':
        current.value = total.value
        emit(
          'complete',
          progress.success || 0,
          progress.failed || 0,
          progress.total_new_articles || 0
        )
        break
      case 'cancelled':
        current.value = progress.current || 0
        emit('close')
        break
    }
  })
})

onUnmounted(() => {
  if (unlisten) unlisten()
})

const progressPercentage = () => {
  if (total.value === 0) return 0
  return Math.round((current.value / total.value) * 100)
}
</script>

<template>
  <div v-if="show" class="modal-overlay">
    <div class="modal-content">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 pt-5 pb-3">
        <h3 class="text-sm font-semibold" style="color: var(--ink-text)">批量刷新</h3>
        <button
          class="close-btn w-7 h-7 flex items-center justify-center rounded-md transition-colors duration-150"
          @click="emit('close')"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- Progress -->
      <div class="px-5 space-y-3">
        <!-- Progress bar -->
        <div>
          <div class="flex justify-between text-xs mb-1.5" style="color: var(--ink-text-secondary)">
            <span>{{ current }} / {{ total }}</span>
            <span>{{ progressPercentage() }}%</span>
          </div>
          <div class="w-full rounded-full h-1" style="background: var(--ink-border-light)">
            <div
              class="h-1 rounded-full transition-all duration-300"
              style="background: var(--ink-accent)"
              :style="{ width: progressPercentage() + '%' }"
            ></div>
          </div>
        </div>

        <!-- Current feed -->
        <div
          v-if="currentFeedTitle"
          class="text-xs truncate"
          style="color: var(--ink-text-tertiary)"
        >
          正在刷新：{{ currentFeedTitle }}
        </div>

        <!-- Stats -->
        <div class="grid grid-cols-3 gap-3 text-center">
          <div>
            <div class="text-lg font-semibold" style="color: var(--ink-success)">
              {{ successCount }}
            </div>
            <div class="text-[10px]" style="color: var(--ink-text-tertiary)">成功</div>
          </div>
          <div>
            <div class="text-lg font-semibold" style="color: var(--ink-error)">
              {{ failureCount }}
            </div>
            <div class="text-[10px]" style="color: var(--ink-text-tertiary)">失败</div>
          </div>
          <div>
            <div class="text-lg font-semibold" style="color: var(--ink-accent)">
              {{ totalNewArticles }}
            </div>
            <div class="text-[10px]" style="color: var(--ink-text-tertiary)">新文章</div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex justify-end px-5 py-3 mt-1"
        style="border-top: 1px solid var(--ink-border-light)"
      >
        <button :disabled="isCancelling" class="btn-ghost text-xs" @click="cancelRefresh">
          {{ isCancelling ? '取消中...' : '取消刷新' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.close-btn {
  color: var(--ink-text-tertiary);
}
.close-btn:hover {
  background: var(--ink-paper);
}
</style>
