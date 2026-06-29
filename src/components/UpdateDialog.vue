<script setup lang="ts">
import { useAutoUpdate } from '@/composables/useAutoUpdate'
import { useI18n } from '@/composables/useI18n'

const { t } = useI18n()
const {
  showDialog,
  updateState,
  newVersion,
  downloadProgress,
  errorMessage,
  dismiss,
  closeDialog,
  downloadAndInstall,
  restart,
} = useAutoUpdate()
</script>

<template>
  <v-dialog :model-value="showDialog" max-width="400" persistent>
    <v-card>
      <v-card-title class="d-flex align-center ga-2">
        <v-icon icon="mdi-cellphone-link" />
        {{ t('update.available') }}
      </v-card-title>

      <v-card-text>
        <!-- available -->
        <template v-if="updateState === 'available'">
          <div class="text-body-2 mb-2">{{ t('update.newVersion', { version: newVersion }) }}</div>
        </template>

        <!-- downloading -->
        <template v-else-if="updateState === 'downloading'">
          <div class="text-body-2 mb-3">{{ t('update.downloading') }}</div>
          <v-progress-linear
            :model-value="downloadProgress"
            color="primary"
            height="6"
            rounded
          />
          <div class="text-caption text-center mt-1">{{ downloadProgress }}%</div>
        </template>

        <!-- ready -->
        <template v-else-if="updateState === 'ready'">
          <div class="text-body-2">{{ t('update.readyDescription') }}</div>
        </template>

        <!-- error -->
        <template v-else-if="updateState === 'error'">
          <div class="text-error text-body-2">{{ errorMessage }}</div>
        </template>
      </v-card-text>

      <v-card-actions>
        <v-spacer />

        <!-- available: 以后再说 / 立即更新 -->
        <v-btn v-if="updateState === 'available'" variant="text" @click="dismiss">
          {{ t('update.later') }}
        </v-btn>
        <v-btn
          v-if="updateState === 'available'"
          variant="flat"
          color="primary"
          @click="downloadAndInstall"
        >
          {{ t('update.updateNow') }}
        </v-btn>

        <!-- downloading: 关闭（隐藏弹窗，下载仍继续） -->
        <v-btn v-if="updateState === 'downloading'" variant="text" @click="closeDialog">
          {{ t('update.close') }}
        </v-btn>

        <!-- ready: 立即重启 -->
        <v-btn v-if="updateState === 'ready'" variant="flat" color="primary" @click="restart">
          {{ t('update.restart') }}
        </v-btn>

        <!-- error: 关闭 / 重试 -->
        <v-btn v-if="updateState === 'error'" variant="text" @click="closeDialog">
          {{ t('update.close') }}
        </v-btn>
        <v-btn
          v-if="updateState === 'error'"
          variant="flat"
          color="primary"
          @click="downloadAndInstall"
        >
          {{ t('update.retry') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
