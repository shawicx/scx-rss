<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useToast } from '@/composables/useToast'
import { useI18n } from '@/composables/useI18n'

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
const { t } = useI18n()

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
    console.error('取消刷新失败:', error)
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
  <v-dialog :model-value="show" max-width="420">
    <v-card>
      <!-- Header -->
      <v-card-title class="d-flex align-center justify-between pa-5 pb-3">
        <span class="text-body-1 font-weight-semibold">{{ $t('refresh.refreshAll') }}</span>
        <v-btn icon variant="text" size="small" @click="emit('close')">
          <v-icon size="18">mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <v-card-text class="px-5">
        <!-- Progress bar -->
        <div class="mb-3">
          <div class="d-flex justify-between text-caption mb-2">
            <span>{{ current }} / {{ total }}</span>
            <span>{{ progressPercentage() }}%</span>
          </div>
          <v-progress-linear
            :model-value="progressPercentage()"
            color="primary"
            height="4"
            rounded
          />
        </div>

        <!-- Current feed -->
        <p v-if="currentFeedTitle" class="text-caption text-medium-emphasis text-truncate mb-3">
          {{ $t('refresh.refreshing') }}：{{ currentFeedTitle }}
        </p>

        <!-- Stats -->
        <div class="d-flex ga-3 text-center mt-3">
          <div class="flex-1-1">
            <div class="text-h6 font-weight-bold text-success">{{ successCount }}</div>
            <div class="text-caption text-medium-emphasis">{{ $t('refresh.refreshSuccess') }}</div>
          </div>
          <div class="flex-1-1">
            <div class="text-h6 font-weight-bold text-error">{{ failureCount }}</div>
            <div class="text-caption text-medium-emphasis">{{ $t('refresh.refreshFailed') }}</div>
          </div>
          <div class="flex-1-1">
            <div class="text-h6 font-weight-bold text-primary">{{ totalNewArticles }}</div>
            <div class="text-caption text-medium-emphasis">{{ $t('articles.all') }}</div>
          </div>
        </div>
      </v-card-text>

      <v-divider />
      <v-card-actions class="justify-end pa-3">
        <v-btn variant="text" size="small" :loading="isCancelling" @click="cancelRefresh">
          {{ isCancelling ? $t('common.loading') : $t('common.cancel') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.flex-1-1 {
  flex: 1 1 0;
}
.justify-between {
  justify-content: space-between;
}
</style>
