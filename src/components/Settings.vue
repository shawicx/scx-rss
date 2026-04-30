<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import { useOpml } from '@/composables/useOpml'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { useTheme, type ThemeName } from '@/composables/useTheme'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'data-restored'): void
  (e: 'feeds-changed'): void
}>()

const { exportOpml, importOpml } = useOpml()
const { refresh } = useFeeds()
const { showSuccess, showError, showInfo } = useToast()
const { currentTheme, setTheme } = useTheme()

const isExporting = ref(false)
const isImporting = ref(false)
const isBackingUp = ref(false)
const isRestoring = ref(false)

const themeOptions = [
  { title: 'Material Light', value: 'materialLight' as ThemeName },
  { title: 'Material Dark', value: 'materialDark' as ThemeName },
  { title: 'Warm Ink', value: 'warmInk' as ThemeName },
]

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
    emit('feeds-changed')
  } finally {
    isImporting.value = false
  }
}

const handleBackup = async () => {
  if (isBackingUp.value) return
  isBackingUp.value = true
  try {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)
    const defaultName = `scx-rss-backup-${timestamp}.db`
    const filePath = await save({
      defaultPath: defaultName,
      filters: [{ name: 'SQLite Database', extensions: ['db'] }],
    })
    if (!filePath) {
      showInfo('备份已取消')
      return
    }
    await invoke('backup_database', { backupPath: filePath })
    showSuccess('数据库备份成功')
  } catch (error) {
    showError(`备份失败: ${error}`)
  } finally {
    isBackingUp.value = false
  }
}

const handleRestore = async () => {
  if (isRestoring.value) return
  isRestoring.value = true
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'SQLite Database', extensions: ['db'] }],
    })
    if (!selected || typeof selected !== 'string') {
      showInfo('恢复已取消')
      return
    }
    if (!confirm('恢复操作将覆盖当前所有数据，是否继续？')) {
      return
    }
    await invoke('restore_database', { backupPath: selected })
    showSuccess('数据恢复成功，正在重新加载...')
    emit('data-restored')
  } catch (error) {
    showError(`恢复失败: ${error}`)
  } finally {
    isRestoring.value = false
  }
}

const close = () => {
  emit('close')
}
</script>

<template>
  <v-dialog :model-value="show" max-width="420" @click:outside="close">
    <v-card>
      <!-- Header -->
      <v-card-title class="d-flex align-center justify-between pa-5 pb-3">
        <span class="text-body-1 font-weight-semibold">设置</span>
        <v-btn icon variant="text" size="small" @click="close">
          <v-icon size="18">mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <v-card-text class="px-5 pb-3">
        <!-- Theme Selector -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">主题</h3>
          <v-select
            :model-value="currentTheme"
            :items="themeOptions"
            item-title="title"
            item-value="value"
            density="compact"
            variant="outlined"
            hide-details
            @update:model-value="setTheme"
          />
        </div>

        <!-- OPML Import/Export -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">OPML 导入 / 导出</h3>
          <p class="text-caption text-medium-emphasis mb-3">
            将订阅源导出为 OPML 文件以备份，或从 OPML 文件导入订阅源。
          </p>
          <div class="d-flex ga-2">
            <v-btn
              :loading="isExporting"
              color="primary"
              variant="outlined"
              size="medium"
              class="flex-1-1"
              prepend-icon="mdi-download"
              @click="handleExport"
            >
              导出
            </v-btn>
            <v-btn
              :loading="isImporting"
              color="primary"
              variant="outlined"
              size="medium"
              class="flex-1-1"
              prepend-icon="mdi-upload"
              @click="handleImport"
            >
              导入
            </v-btn>
          </div>
        </div>

        <!-- Data Backup/Restore -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">数据备份 / 恢复</h3>
          <p class="text-caption text-medium-emphasis mb-3">
            备份完整数据库（含文章、订阅、阅读状态），或从备份文件恢复。
          </p>
          <div class="d-flex ga-2">
            <v-btn
              :loading="isBackingUp"
              color="primary"
              variant="outlined"
              size="medium"
              class="flex-1-1"
              prepend-icon="mdi-download"
              @click="handleBackup"
            >
              备份数据
            </v-btn>
            <v-btn
              :loading="isRestoring"
              color="primary"
              variant="outlined"
              size="medium"
              class="flex-1-1"
              prepend-icon="mdi-upload"
              @click="handleRestore"
            >
              恢复数据
            </v-btn>
          </div>
        </div>

        <!-- About -->
        <v-divider class="mb-4" />
        <p class="text-caption text-center text-medium-emphasis">SCX RSS Reader · v1.0.0</p>
      </v-card-text>

      <v-divider />
      <v-card-actions class="justify-end pa-3">
        <v-btn variant="text" size="small" @click="close">关闭</v-btn>
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
