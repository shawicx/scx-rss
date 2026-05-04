<script setup lang="ts">
defineProps<{
  isResizing: boolean
}>()

const emit = defineEmits<{
  dragStart: [event: MouseEvent]
}>()

const onMousedown = (e: MouseEvent) => {
  emit('dragStart', e)
}
</script>

<template>
  <div
    class="resize-handle"
    :class="{ 'resize-handle--active': isResizing }"
    @mousedown="onMousedown"
  />
</template>

<style scoped>
.resize-handle {
  width: 4px;
  cursor: col-resize;
  flex-shrink: 0;
  position: relative;
  transition: background-color 0.15s;
}

.resize-handle::before {
  content: '';
  position: absolute;
  top: 0;
  left: -2px;
  right: -2px;
  bottom: 0;
  cursor: col-resize;
}

.resize-handle:hover,
.resize-handle--active {
  background-color: rgba(var(--v-theme-primary), 0.3);
}
</style>
