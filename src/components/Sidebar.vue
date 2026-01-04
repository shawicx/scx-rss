<script setup lang="ts">
import { onMounted } from 'vue'
import FeedList from './FeedList.vue'
import { useFeeds } from '@/composables/useFeeds'

/**
 * 侧边栏组件
 * 包含 Feed 列表和设置入口
 */
const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { feeds, loading, init } = useFeeds()

// 组件挂载时初始化 Feeds
onMounted(async () => {
  await init()
})
</script>

<template>
  <div class="h-full flex flex-col bg-gray-100 border-r border-gray-200">
    <!-- 侧边栏头部 -->
    <div class="p-4 border-b border-gray-200">
      <h2 class="text-lg font-semibold text-gray-800">
        SCX RSS Reader
      </h2>
      <p class="text-xs text-gray-500 mt-1">
        {{ feeds.length }} 个订阅源
      </p>
    </div>

    <!-- Feed 列表容器 -->
    <div class="flex-1 overflow-y-auto">
      <FeedList :feeds="feeds" :loading="loading" @feed-selected="emit('feed-selected', $event)" />
    </div>

    <!-- 侧边栏底部：设置入口 -->
    <div class="p-4 border-t border-gray-200">
      <button
        class="w-full px-4 py-2 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-200 rounded transition-colors"
      >
        ⚙️ 设置
      </button>
    </div>
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.3);
}
</style>
