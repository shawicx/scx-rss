/**
 * 自动更新组合式函数
 *
 * 单例模式：模块级 ref 在所有调用方之间共享状态。
 * 启动后 3s 自动检查；Settings 可手动触发。
 */

import { ref } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

export type UpdateState = 'idle' | 'available' | 'downloading' | 'ready' | 'error'

/** 是否显示更新弹窗 */
const showDialog = ref(false)
/** 当前更新状态 */
const updateState = ref<UpdateState>('idle')
/** 是否正在检查更新 */
const checking = ref(false)
/** 新版本号 */
const newVersion = ref('')
/** 下载进度百分比 (0-100) */
const downloadProgress = ref(0)
/** 错误信息 */
const errorMessage = ref('')

/** 本次会话是否已忽略更新（仅影响自动检查，不影响手动检查） */
let dismissed = false
let totalBytes = 0
let downloadedBytes = 0

export function useAutoUpdate() {
  /**
   * 检查更新
   * @param opts.manual 是否手动触发（手动触发在无更新/出错时由调用方提示）
   * @returns 是否发现新版本
   */
  async function checkForUpdate(opts?: { manual?: boolean }): Promise<boolean> {
    const manual = opts?.manual ?? false

    // dev 模式不检查（手动也不检查，避免开发期误触发）
    if (!import.meta.env.PROD) {
      return false
    }
    // 自动检查时，若用户已忽略本次会话，则跳过
    if (!manual && dismissed) {
      return false
    }

    checking.value = true
    try {
      const update = await check()
      if (update?.available) {
        newVersion.value = update.version
        updateState.value = 'available'
        showDialog.value = true
        return true
      }
      // 无更新
      return false
    } catch (e) {
      // 手动检查出错时抛出，由调用方提示；自动检查静默忽略
      if (manual) {
        throw e
      }
      return false
    } finally {
      checking.value = false
    }
  }

  /**
   * 下载并安装更新
   */
  async function downloadAndInstall(): Promise<void> {
    try {
      const update = await check()
      if (!update?.available) return

      updateState.value = 'downloading'
      downloadProgress.value = 0
      errorMessage.value = ''
      totalBytes = 0
      downloadedBytes = 0

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            totalBytes = event.data.contentLength ?? 0
            break
          case 'Progress':
            downloadedBytes += event.data.chunkLength
            if (totalBytes > 0) {
              downloadProgress.value = Math.round((downloadedBytes / totalBytes) * 100)
            }
            break
          case 'Finished':
            updateState.value = 'ready'
            break
        }
      })
    } catch (e) {
      updateState.value = 'error'
      errorMessage.value = e instanceof Error ? e.message : String(e)
    }
  }

  /**
   * 忽略本次会话的更新提示
   */
  function dismiss(): void {
    dismissed = true
    showDialog.value = false
    updateState.value = 'idle'
  }

  /**
   * 关闭弹窗（不设置 dismissed，下次检查仍会弹）
   */
  function closeDialog(): void {
    showDialog.value = false
    updateState.value = 'idle'
  }

  /**
   * 重启应用以完成安装
   */
  async function restart(): Promise<void> {
    await relaunch()
  }

  /**
   * 启动后延迟自动检查
   */
  function startCheck(): void {
    setTimeout(() => {
      checkForUpdate().catch(() => {
        // 自动检查失败静默忽略
      })
    }, 3000)
  }

  return {
    showDialog,
    updateState,
    checking,
    newVersion,
    downloadProgress,
    errorMessage,
    checkForUpdate,
    downloadAndInstall,
    dismiss,
    closeDialog,
    restart,
    startCheck,
  }
}
