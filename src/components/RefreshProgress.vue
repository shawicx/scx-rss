<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useToast } from '@/composables/useToast'

/**
 * 刷新进度类型定义
 */
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

/**
 * 组件属性
 */
interface Props {
  show: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'complete', success: number, failed: number, newArticles: number): void
}>()

const { showError } = useToast()

// 状态管理
const current = ref(0)
const total = ref(0)
const currentFeedTitle = ref('')
const successCount = ref(0)
const failureCount = ref(0)
const totalNewArticles = ref(0)
const isCancelling = ref(false)

// 取消刷新
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

// 监听刷新进度事件
let unlisten: (() => void) | null = null

onMounted(async () => {
  unlisten = await listen<RefreshProgress>('refresh-progress', (event) => {
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
        emit('complete', progress.success || 0, progress.failed || 0, progress.total_new_articles || 0)
        break

      case 'cancelled':
        current.value = progress.current || 0
        emit('close')
        break
    }
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})

// 计算进度百分比
const progressPercentage = () => {
  if (total.value === 0) return 0
  return Math.round((current.value / total.value) * 100)
}
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
  >
    <div class="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
      <!-- 标题 -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold bg-gradient-to-r from-orange-600 to-amber-600 bg-clip-text text-transparent">
          批量刷新订阅源
        </h3>
        <button
          @click="emit('close')"
          class="text-gray-400 hover:text-orange-600 transition-all duration-300 hover:scale-110"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 进度条 -->
      <div class="mb-4">
        <div class="flex justify-between text-sm text-orange-600 mb-2">
          <span>进度</span>
          <span>{{ current }} / {{ total }} ({{ progressPercentage() }}%)</span>
        </div>
        <div class="w-full bg-orange-200 rounded-full h-2">
          <div
            class="bg-gradient-to-r from-orange-500 to-amber-500 h-2 rounded-full transition-all duration-300"
            :style="{ width: progressPercentage() + '%' }"
          ></div>
        </div>
      </div>

      <!-- 当前刷新的 Feed -->
      <div v-if="currentFeedTitle" class="mb-4 text-sm text-orange-600">
        <span class="font-medium">当前刷新：</span>{{ currentFeedTitle }}
      </div>

      <!-- 统计信息 -->
      <div class="grid grid-cols-3 gap-4 mb-4">
        <div class="text-center">
          <div class="text-2xl font-semibold text-green-600">{{ successCount }}</div>
          <div class="text-xs text-orange-600">成功</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-semibold text-red-600">{{ failureCount }}</div>
          <div class="text-xs text-orange-600">失败</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-semibold text-orange-600">{{ totalNewArticles }}</div>
          <div class="text-xs text-orange-600">新文章</div>
        </div>
      </div>

      <!-- 取消按钮 -->
      <div class="flex justify-end">
        <button
          @click="cancelRefresh"
          :disabled="isCancelling"
          class="px-4 py-2 text-sm text-gray-700 bg-gray-200 hover:bg-orange-100 rounded transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
        >
          {{ isCancelling ? '取消中...' : '取消刷新' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
