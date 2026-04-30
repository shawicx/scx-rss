<script setup lang="ts">
import { useToast } from '@/composables/useToast'

const { toasts, removeToast } = useToast()

const iconMap: Record<string, string> = {
  success: 'mdi-check',
  error: 'mdi-close',
  info: 'mdi-information',
  warning: 'mdi-alert',
}

const colorMap: Record<string, string> = {
  success: 'success',
  error: 'error',
  info: 'primary',
  warning: 'warning',
}
</script>

<template>
  <div class="toast-container">
    <TransitionGroup name="toast" tag="div" class="d-flex flex-column ga-2">
      <v-sheet
        v-for="toast in toasts"
        :key="toast.id"
        rounded="lg"
        class="elevation-4 toast-item"
        :style="{
          borderLeft: `3px solid rgb(var(--v-theme-${colorMap[toast.type] || 'primary'}))`,
        }"
        @click="removeToast(toast.id)"
      >
        <div class="d-flex align-start ga-3 pa-3">
          <v-icon
            :color="colorMap[toast.type] || 'primary'"
            :icon="iconMap[toast.type] || 'mdi-information'"
            size="18"
          />
          <span class="text-body-2 flex-fill">{{ toast.message }}</span>
          <v-btn icon variant="text" size="x-small" @click.stop="removeToast(toast.id)">
            <v-icon size="14">mdi-close</v-icon>
          </v-btn>
        </div>
      </v-sheet>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 9999;
  pointer-events: none;
}
.toast-item {
  pointer-events: auto;
  cursor: pointer;
  max-width: 340px;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-enter-to {
  opacity: 1;
  transform: translateX(0);
}
.toast-enter-active {
  transition: all 0.25s ease-out;
}
.toast-leave-from {
  opacity: 1;
  transform: translateX(0);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-active {
  transition: all 0.2s ease-in;
}
.toast-move {
  transition: transform 0.25s ease;
}

.flex-fill {
  flex: 1;
}
</style>
