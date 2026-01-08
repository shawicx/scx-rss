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
  const baseClasses = 'pointer-events-auto w-full max-w-sm overflow-hidden rounded-xl shadow-lg transform transition-all duration-300'

  const typeClasses: Record<string, string> = {
    success: 'bg-gradient-to-r from-amber-50 to-orange-50 text-amber-900 border-l-4 border-amber-500',
    error: 'bg-gradient-to-r from-red-50 to-pink-50 text-red-900 border-l-4 border-red-500',
    info: 'bg-gradient-to-r from-orange-50 to-yellow-50 text-orange-900 border-l-4 border-orange-500',
    warning: 'bg-gradient-to-r from-yellow-50 to-amber-50 text-yellow-900 border-l-4 border-yellow-600'
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
        class="toast-item transform transition-all duration-300 ease-out cursor-pointer"
      >
        <div class="p-4">
          <div class="flex items-start">
            <!-- 图标 -->
            <div class="flex-shrink-0">
              <span class="toast-icon text-xl">{{ getToastIcon(toast.type) }}</span>
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
                class="inline-flex rounded-lg p-1.5 focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all duration-200 hover:scale-110"
                :class="{
                  'hover:bg-amber-200 text-amber-600 focus:ring-amber-500': toast.type === 'success',
                  'hover:bg-red-200 text-red-600 focus:ring-red-500': toast.type === 'error',
                  'hover:bg-orange-200 text-orange-600 focus:ring-orange-500': toast.type === 'info',
                  'hover:bg-yellow-200 text-yellow-700 focus:ring-yellow-500': toast.type === 'warning'
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
/* Toast 进入动画 - 弹跳效果 */
.toast-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.3) rotate(-5deg);
}

.toast-enter-to {
  opacity: 1;
  transform: translateX(0) scale(1) rotate(0deg);
}

.toast-enter-active {
  animation: bounceIn 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

/* Toast 离开动画 - 滑出淡出 */
.toast-leave-from {
  opacity: 1;
  transform: translateX(0) scale(1);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.toast-leave-active {
  transition: all 0.3s ease-in;
}

/* Toast 列表项移动动画 */
.toast-move {
  transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 悬停效果 */
.toast-item:hover {
  transform: translateX(-5px) scale(1.02);
  box-shadow: 0 10px 40px rgba(255, 142, 83, 0.2);
}

/* 图标动画 */
@keyframes iconPop {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.2);
  }
}

.toast-icon {
  animation: iconPop 0.5s ease-in-out;
}
</style>
