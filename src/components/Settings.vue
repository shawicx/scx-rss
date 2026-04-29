<script setup lang="ts">
import { ref } from 'vue'
import { useOpml } from '@/composables/useOpml'
import { useFeeds } from '@/composables/useFeeds'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { exportOpml, importOpml } = useOpml()
const { refresh } = useFeeds()

const isExporting = ref(false)
const isImporting = ref(false)

const handleExport = async () => {
  if (isExporting.value) return
  isExporting.value = true
  try {
    await exportOpml()
  } finally {
    isExporting.value = false
  }
}

const handleImport = async () => {
  if (isImporting.value) return
  isImporting.value = true
  try {
    await importOpml()
    await refresh()
  } finally {
    isImporting.value = false
  }
}

const close = () => {
  emit('close')
}
</script>

<template>
  <div v-if="show" class="modal-overlay" @click.self="close">
    <div class="modal-content">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 pt-5 pb-3">
        <h2 class="text-sm font-semibold" style="color: var(--ink-text)">设置</h2>
        <button
          class="close-btn w-7 h-7 flex items-center justify-center rounded-md transition-colors duration-150"
          @click="close"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Content -->
      <div class="px-5 pb-3">
        <div class="space-y-3">
          <h3 class="text-xs font-medium" style="color: var(--ink-text-secondary)">OPML 导入 / 导出</h3>
          <p class="text-xs leading-relaxed" style="color: var(--ink-text-tertiary)">
            将订阅源导出为 OPML 文件以备份，或从 OPML 文件导入订阅源。
          </p>

          <div class="flex gap-2">
            <button
              :disabled="isExporting"
              class="btn-ink flex-1 text-xs gap-1.5"
              @click="handleExport"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              {{ isExporting ? '导出中...' : '导出' }}
            </button>
            <button
              :disabled="isImporting"
              class="btn-ink flex-1 text-xs gap-1.5"
              @click="handleImport"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
              </svg>
              {{ isImporting ? '导入中...' : '导入' }}
            </button>
          </div>
        </div>

        <!-- About -->
        <div class="mt-6 pt-4 text-center" style="border-top: 1px solid var(--ink-border-light)">
          <p class="text-xs" style="color: var(--ink-text-tertiary)">SCX RSS Reader · v1.0.0</p>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex justify-end px-5 py-3" style="border-top: 1px solid var(--ink-border-light)">
        <button class="btn-ghost text-xs" @click="close">关闭</button>
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
