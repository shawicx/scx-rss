<script setup lang="ts">
import { useToast } from '@/composables/useToast'

/**
 * Toast 通知容器组件
 * 显示所有 Toast 通知消息
 */
const { toasts, removeToast } = useToast()

/**
 * 获取 Toast 样式类
 */
const getToastClass = (type: string): string => {
  const baseClasses = 'pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg shadow-lg ring-1 ring-black ring-opacity-5'

  const typeClasses: Record<string, string> = {
    success: 'bg-green-50 text-green-800 border-l-4 border-green-500',
    error: 'bg-red-50 text-red-800 border-l-4 border-red-500',
    info: 'bg-blue-50 text-blue-800 border-l-4 border-blue-500',
    warning: 'bg-yellow-50 text-yellow-800 border-l-4 border-yellow-500'
  }

  return `${baseClasses} ${typeClasses[type] || typeClasses.info}`
}

/**
 * 获取 Toast 图标
 */
const getToastIcon = (type: string): string => {
  const icons: Record<string, string> = {
    success: '✓',
    error: '✕',
    info: 'ℹ',
    warning: '⚠'
  }
  return icons[type] || icons.info
}
</script>

<template>
  <!-- Toast 容器：固定在右上角 -->
  <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <TransitionGroup
      name="toast"
      tag="div"
      class="flex flex-col gap-2"
    >
      <div
        v-for="toast in toasts"
        :key="toast.id"
        :class="getToastClass(toast.type)"
        class="transform transition-all duration-300 ease-out"
      >
        <div class="p-4">
          <div class="flex items-start">
            <!-- 图标 -->
            <div class="flex-shrink-0">
              <span class="text-xl">{{ getToastIcon(toast.type) }}</span>
            </div>

            <!-- 消息内容 -->
            <div class="ml-3 flex-1">
              <p class="text-sm font-medium break-words">
                {{ toast.message }}
              </p>
            </div>

            <!-- 关闭按钮 -->
            <div class="ml-4 flex-shrink-0 flex">
              <button
                @click="removeToast(toast.id)"
                class="inline-flex rounded-md p-1.5 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-50 focus:ring-gray-600"
                :class="{
                  'hover:bg-green-100 text-green-500': toast.type === 'success',
                  'hover:bg-red-100 text-red-500': toast.type === 'error',
                  'hover:bg-blue-100 text-blue-500': toast.type === 'info',
                  'hover:bg-yellow-100 text-yellow-500': toast.type === 'warning'
                }"
              >
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
/* Toast 进入动画 */
.toast-enter-from {
  opacity: 0;
  transform: translateX(30px) scale(0.95);
}

.toast-enter-to {
  opacity: 1;
  transform: translateX(0) scale(1);
}

/* Toast 离开动画 */
.toast-leave-from {
  opacity: 1;
  transform: translateX(0) scale(1);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(30px) scale(0.95);
}

/* Toast 列表项移动动画 */
.toast-move {
  transition: transform 300ms ease-out;
}
</style>
