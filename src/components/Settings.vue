<script setup lang="ts">
import { ref } from 'vue'
import { useOpml } from '@/composables/useOpml'
import { useFeeds } from '@/composables/useFeeds'

/**
 * 设置对话框组件
 * 提供 OPML 导入/导出等功能
 */
defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const { exportOpml, importOpml } = useOpml()
const { refresh } = useFeeds()

// 导入/导出状态
const isExporting = ref(false)
const isImporting = ref(false)

/**
 * 处理 OPML 导出
 */
const handleExport = async () => {
  if (isExporting.value) return

  isExporting.value = true
  try {
    await exportOpml()
  } finally {
    isExporting.value = false
  }
}

/**
 * 处理 OPML 导入
 */
const handleImport = async () => {
  if (isImporting.value) return

  isImporting.value = true
  try {
    await importOpml()
    // 导入成功后刷新 Feed 列表
    await refresh()
  } finally {
    isImporting.value = false
  }
}

/**
 * 关闭对话框
 */
const close = () => {
  emit('close')
}
</script>

<template>
  <!-- 遮罩层 -->
  <div
    v-if="show"
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    @click.self="close"
  >
    <!-- 对话框 -->
    <div class="bg-white rounded-lg shadow-xl w-full max-w-md mx-4">
      <!-- 标题栏 -->
      <div class="flex items-center justify-between p-6 border-b border-orange-200">
        <h2 class="text-xl font-semibold bg-gradient-to-r from-orange-600 to-amber-600 bg-clip-text text-transparent">
          设置
        </h2>
        <button
          @click="close"
          class="text-gray-400 hover:text-orange-600 transition-all duration-300 hover:scale-110"
        >
          <svg
            class="w-6 h-6"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- 内容区域 -->
      <div class="p-6">
        <!-- OPML 导入/导出 -->
        <div class="space-y-4">
          <h3 class="text-lg font-medium text-gray-800">
            OPML 导入/导出
          </h3>
          <p class="text-sm text-orange-600">
            导出您的订阅源到 OPML 文件,或从 OPML 文件导入订阅源
          </p>

          <!-- 按钮组 -->
          <div class="flex gap-3">
            <!-- 导出按钮 -->
            <button
              @click="handleExport"
              :disabled="isExporting"
              class="btn-sunset flex-1 px-4 py-2 text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <span v-if="!isExporting" class="flex items-center justify-center gap-2">
                <svg
                  class="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
                导出 OPML
              </span>
              <span v-else class="flex items-center justify-center gap-2">
                <svg
                  class="w-4 h-4 animate-spin"
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
                导出中...
              </span>
            </button>

            <!-- 导入按钮 -->
            <button
              @click="handleImport"
              :disabled="isImporting"
              class="btn-sunset flex-1 px-4 py-2 text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <span v-if="!isImporting" class="flex items-center justify-center gap-2">
                <svg
                  class="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                  />
                </svg>
                导入 OPML
              </span>
              <span v-else class="flex items-center justify-center gap-2">
                <svg
                  class="w-4 h-4 animate-spin"
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
                导入中...
              </span>
            </button>
          </div>
        </div>

        <!-- 分隔线 -->
        <div class="my-6 border-t border-orange-200"></div>

        <!-- 关于信息 -->
        <div class="text-center text-sm text-orange-600">
          <p>SCX RSS Reader</p>
          <p class="mt-1">版本 1.0.0</p>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="flex justify-end p-6 border-t border-orange-200">
        <button
          @click="close"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-orange-300 hover:bg-orange-50 rounded transition-all duration-300 hover:scale-105"
        >
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 确保对话框在所有内容之上 */
.fixed {
  position: fixed;
}

.z-50 {
  z-index: 50;
}
</style>
