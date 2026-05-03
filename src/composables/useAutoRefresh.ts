import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './useToast'
import { STORAGE_KEYS } from '@/utils/constants'

const { showInfo, showError } = useToast()

const enabled = ref(localStorage.getItem(STORAGE_KEYS.AUTO_REFRESH_ENABLED) === 'true')
const intervalMinutes = ref(Number(localStorage.getItem(STORAGE_KEYS.AUTO_REFRESH_INTERVAL)) || 30)
const lastRefreshTime = ref<string | null>(localStorage.getItem(STORAGE_KEYS.LAST_REFRESH_TIME))

let timerId: ReturnType<typeof setInterval> | null = null
let isRefreshing = false

function startTimer() {
  stopTimer()
  const ms = intervalMinutes.value * 60 * 1000
  timerId = setInterval(silentRefresh, ms)
}

function stopTimer() {
  if (timerId !== null) {
    clearInterval(timerId)
    timerId = null
  }
}

async function silentRefresh() {
  if (isRefreshing) return
  isRefreshing = true

  try {
    const message = await invoke<string>('batch_refresh_feeds')

    const match = message.match(/New articles: (\d+)/)
    const newCount = match ? parseInt(match[1], 10) : 0

    if (newCount > 0) {
      showInfo(`自动刷新发现 ${newCount} 篇新文章`)
    }

    window.dispatchEvent(new CustomEvent('feeds-refreshed'))
  } catch (error) {
    showError(`自动刷新失败: ${error}`)
  } finally {
    isRefreshing = false
    const now = new Date().toISOString()
    lastRefreshTime.value = now
    localStorage.setItem(STORAGE_KEYS.LAST_REFRESH_TIME, now)
  }
}

export function useAutoRefresh() {
  function toggleAutoRefresh(value: boolean | null) {
    const enabled_val = value ?? false
    enabled.value = enabled_val
    localStorage.setItem(STORAGE_KEYS.AUTO_REFRESH_ENABLED, String(enabled_val))
    if (enabled_val) {
      startTimer()
    } else {
      stopTimer()
    }
  }

  function setRefreshInterval(minutes: number) {
    intervalMinutes.value = minutes
    localStorage.setItem(STORAGE_KEYS.AUTO_REFRESH_INTERVAL, String(minutes))
    if (enabled.value) {
      startTimer()
    }
  }

  function formatLastRefreshed(): string {
    if (!lastRefreshTime.value) return '从未刷新'
    const diff = Date.now() - new Date(lastRefreshTime.value).getTime()
    const minutes = Math.floor(diff / 60000)
    if (minutes < 1) return '刚刚'
    if (minutes < 60) return `${minutes} 分钟前`
    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours} 小时前`
    return `${Math.floor(hours / 24)} 天前`
  }

  function handleVisibility() {
    if (document.hidden) {
      stopTimer()
    } else if (enabled.value) {
      if (lastRefreshTime.value) {
        const elapsed = Date.now() - new Date(lastRefreshTime.value).getTime()
        if (elapsed >= intervalMinutes.value * 60 * 1000) {
          silentRefresh()
        }
      }
      startTimer()
    }
  }

  onMounted(() => {
    document.addEventListener('visibilitychange', handleVisibility)
    if (enabled.value) {
      startTimer()
    }
  })

  onUnmounted(() => {
    document.removeEventListener('visibilitychange', handleVisibility)
    stopTimer()
  })

  return {
    enabled,
    intervalMinutes,
    lastRefreshTime,
    toggleAutoRefresh,
    setRefreshInterval,
    formatLastRefreshed,
  }
}
