<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { save, open } from '@tauri-apps/plugin-dialog'
import { useOpml } from '@/composables/useOpml'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { useTheme, type ThemePreference } from '@/composables/useTheme'
import { useAutoRefresh } from '@/composables/useAutoRefresh'
import { useI18n } from '@/composables/useI18n'
import { useAutoUpdate } from '@/composables/useAutoUpdate'
import { AUTO_REFRESH_OPTIONS } from '@/utils/constants'

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
const { preference, setPreference } = useTheme()
const { locale, t, setLocale } = useI18n()
const {
  enabled: autoRefreshEnabled,
  intervalMinutes,
  toggleAutoRefresh,
  setRefreshInterval,
  formatLastRefreshed,
} = useAutoRefresh()
const { checkForUpdate, checking: isCheckingUpdate } = useAutoUpdate()

const isExporting = ref(false)
const isImporting = ref(false)
const isBackingUp = ref(false)
const isRestoring = ref(false)
const appVersion = ref('')

const languageOptions = [
  { title: t('settings.followSystem'), value: 'system' as const },
  { title: t('settings.simplifiedChinese'), value: 'zh-CN' as const },
  { title: t('settings.english'), value: 'en' as const },
]

const themeOptions = [
  { title: t('theme.light'), value: 'light' as ThemePreference },
  { title: t('theme.dark'), value: 'dark' as ThemePreference },
  { title: t('theme.system'), value: 'system' as ThemePreference },
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
      filters: [{ name: 'SQLite 数据库', extensions: ['db'] }],
    })
    if (!filePath) {
      showInfo(t('errors.backupCancelled'))
      return
    }
    await invoke('backup_database', { backupPath: filePath })
    showSuccess(t('errors.backupSuccess'))
  } catch (error) {
    showError(`${t('errors.backupFailed')}: ${error}`)
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
      filters: [{ name: 'SQLite 数据库', extensions: ['db'] }],
    })
    if (!selected || typeof selected !== 'string') {
      showInfo(t('errors.restoreCancelled'))
      return
    }
    if (!confirm(t('errors.restoreConfirm'))) {
      return
    }
    await invoke('restore_database', { backupPath: selected })
    showSuccess(t('errors.restoreSuccess'))
    emit('data-restored')
  } catch (error) {
    showError(`${t('errors.restoreFailed')}: ${error}`)
  } finally {
    isRestoring.value = false
  }
}

const handleCheckUpdate = async () => {
  try {
    const found = await checkForUpdate({ manual: true })
    if (!found) {
      showInfo(t('update.upToDate'))
    }
    // 若 found 为 true，UpdateDialog 已自动弹出（showDialog 已置 true）
  } catch (error) {
    showError(`${t('update.failed')}: ${error}`)
  }
}

const close = () => {
  emit('close')
}

// 读取应用版本号（用于关于区展示）
getVersion().then((v) => {
  appVersion.value = v
})
</script>

<template>
  <v-dialog :model-value="show" max-width="36rem" @click:outside="close">
    <v-card>
      <!-- Header -->
      <v-card-title class="d-flex align-center justify-between pa-5 pb-3">
        <span class="text-body-1 font-weight-semibold">{{ $t('settings.title') }}</span>
        <v-btn icon variant="text" size="small" @click="close">
          <v-icon size="18">mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <v-card-text class="px-5 pb-3">
        <!-- Language Selector (新增) -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('settings.language') }}</h3>
          <v-select
            :model-value="locale === 'zh-CN' || locale === 'en' ? locale : 'system'"
            :items="languageOptions"
            item-title="title"
            item-value="value"
            density="compact"
            variant="outlined"
            hide-details
            @update:model-value="setLocale"
          />
        </div>

        <!-- Theme Selector -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('settings.theme') }}</h3>
          <v-select
            :model-value="preference"
            :items="themeOptions"
            item-title="title"
            item-value="value"
            density="compact"
            variant="outlined"
            hide-details
            @update:model-value="setPreference"
          />
        </div>

        <!-- Auto-Refresh -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('settings.autoRefresh') }}</h3>
          <p class="text-caption text-medium-emphasis mb-3">
            {{ $t('settings.autoRefreshDesc') }}
          </p>
          <div class="d-flex align-center justify-between mb-2">
            <span class="text-body-2">{{ $t('settings.enableAutoRefresh') }}</span>
            <v-switch
              :model-value="autoRefreshEnabled"
              color="primary"
              density="compact"
              hide-details
              @update:model-value="toggleAutoRefresh"
            />
          </div>
          <template v-if="autoRefreshEnabled">
            <v-select
              :model-value="intervalMinutes"
              :items="[...AUTO_REFRESH_OPTIONS]"
              item-title="title"
              item-value="value"
              density="compact"
              variant="outlined"
              hide-details
              :label="$t('settings.refreshInterval')"
              @update:model-value="setRefreshInterval"
            />
            <p class="text-caption text-medium-emphasis mt-2">
              <v-icon size="14" class="mr-1">mdi-clock-outline</v-icon>
              {{ $t('settings.lastRefresh') }}：{{ formatLastRefreshed() }}
            </p>
          </template>
        </div>

        <!-- OPML Import/Export -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('settings.opml') }}</h3>
          <p class="text-caption text-medium-emphasis mb-3">
            {{ $t('settings.opmlDesc') }}
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
              {{ $t('settings.export') }}
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
              {{ $t('settings.import') }}
            </v-btn>
          </div>
        </div>

        <!-- Data Backup/Restore -->
        <div class="mb-5">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('settings.backup') }}</h3>
          <p class="text-caption text-medium-emphasis mb-3">
            {{ $t('settings.backupDesc') }}
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
              {{ $t('settings.backupData') }}
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
              {{ $t('settings.restoreData') }}
            </v-btn>
          </div>
        </div>

        <!-- About + Update Check -->
        <v-divider class="mb-4" />
        <div class="mb-2">
          <h3 class="text-caption font-weight-medium mb-2">{{ $t('update.checkForUpdates') }}</h3>
          <div class="d-flex align-center justify-between">
            <span class="text-body-2">
              SCX RSS Reader · {{ $t('about.version') }} {{ appVersion }}
            </span>
            <v-btn
              :loading="isCheckingUpdate"
              :disabled="isCheckingUpdate"
              color="primary"
              variant="outlined"
              size="small"
              prepend-icon="mdi-cellphone-link"
              @click="handleCheckUpdate"
            >
              {{ isCheckingUpdate ? $t('update.checking') : $t('update.checkForUpdates') }}
            </v-btn>
          </div>
        </div>
      </v-card-text>

      <v-divider />
      <v-card-actions class="justify-end pa-3">
        <v-btn variant="text" size="small" @click="close">{{ $t('common.close') }}</v-btn>
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
