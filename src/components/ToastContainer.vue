<script setup lang="ts">
import { useToast } from '@/composables/useToast'

const { toasts, removeToast } = useToast()

const getTypeStyle = (type: string) => {
  const styles: Record<string, { bg: string; border: string; color: string; icon: string }> = {
    success: {
      bg: 'var(--ink-success-bg)',
      border: 'var(--ink-success)',
      color: 'var(--ink-success)',
      icon: '✓',
    },
    error: {
      bg: 'var(--ink-error-bg)',
      border: 'var(--ink-error)',
      color: 'var(--ink-error)',
      icon: '✕',
    },
    info: {
      bg: 'var(--ink-accent-light)',
      border: 'var(--ink-accent)',
      color: 'var(--ink-accent)',
      icon: 'i',
    },
    warning: {
      bg: 'var(--ink-warning-bg)',
      border: 'var(--ink-warning)',
      color: 'var(--ink-warning)',
      icon: '!',
    },
  }
  return styles[type] || styles.info
}
</script>

<template>
  <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <TransitionGroup name="toast" tag="div" class="flex flex-col gap-2">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="pointer-events-auto w-full max-w-xs overflow-hidden rounded-lg shadow-lg cursor-pointer"
        style="background: var(--ink-white)"
        @click="removeToast(toast.id)"
      >
        <div
          class="flex items-start gap-2.5 px-3.5 py-3"
          style="border-left: 3px solid"
          :style="{ borderLeftColor: getTypeStyle(toast.type).border }"
        >
          <!-- Icon -->
          <span
            class="flex-shrink-0 w-5 h-5 flex items-center justify-center rounded-full text-[10px] font-bold"
            :style="{
              background: getTypeStyle(toast.type).bg,
              color: getTypeStyle(toast.type).color,
            }"
          >
            {{ getTypeStyle(toast.type).icon }}
          </span>

          <!-- Message -->
          <p class="flex-1 text-xs leading-relaxed" style="color: var(--ink-text)">
            {{ toast.message }}
          </p>

          <!-- Close -->
          <button
            class="toast-close flex-shrink-0 w-4 h-4 flex items-center justify-center rounded"
            @click.stop="removeToast(toast.id)"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-close {
  color: var(--ink-text-tertiary);
  opacity: 0.4;
  transition: opacity 100ms;
}
.toast-close:hover {
  opacity: 1;
}
</style>
