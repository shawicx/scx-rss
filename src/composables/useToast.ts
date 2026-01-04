/**
 * Toast 通知组合式函数
 *
 * 提供统一的消息通知接口
 */

import { ref, type Ref } from 'vue'
import { TOAST_DURATION } from '@/utils/constants'

/** Toast 消息类型 */
export type ToastType = 'success' | 'error' | 'info' | 'warning'

/** Toast 消息接口 */
export interface ToastMessage {
  /** 消息唯一 ID */
  id: number
  /** 消息类型 */
  type: ToastType
  /** 消息内容 */
  message: string
  /** 持续时间（毫秒），0 表示不自动关闭 */
  duration: number
}

/** Toast 消息队列 */
const toasts: Ref<ToastMessage[]> = ref([])

/** 消息 ID 计数器 */
let toastId = 0

/**
 * Toast 通知组合式函数
 */
export function useToast() {
  /**
   * 添加 Toast 消息
   * @param type 消息类型
   * @param message 消息内容
   * @param duration 持续时间（毫秒）
   */
  function addToast(type: ToastType, message: string, duration: number = TOAST_DURATION.INFO): void {
    const id = ++toastId

    const toast: ToastMessage = {
      id,
      type,
      message,
      duration
    }

    toasts.value.push(toast)

    // 如果设置了自动关闭，则倒计时后移除
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id)
      }, duration)
    }
  }

  /**
   * 移除 Toast 消息
   * @param id 消息 ID
   */
  function removeToast(id: number): void {
    const index = toasts.value.findIndex((toast) => toast.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  /**
   * 显示成功消息
   * @param message 消息内容
   * @param duration 持续时间（毫秒）
   */
  function showSuccess(message: string, duration: number = TOAST_DURATION.SUCCESS): void {
    addToast('success', message, duration)
  }

  /**
   * 显示错误消息
   * @param message 消息内容
   * @param duration 持续时间（毫秒）
   */
  function showError(message: string, duration: number = TOAST_DURATION.ERROR): void {
    addToast('error', message, duration)
  }

  /**
   * 显示信息消息
   * @param message 消息内容
   * @param duration 持续时间（毫秒）
   */
  function showInfo(message: string, duration: number = TOAST_DURATION.INFO): void {
    addToast('info', message, duration)
  }

  /**
   * 显示警告消息
   * @param message 消息内容
   * @param duration 持续时间（毫秒）
   */
  function showWarning(message: string, duration: number = TOAST_DURATION.INFO): void {
    addToast('warning', message, duration)
  }

  /**
   * 清空所有消息
   */
  function clearAll(): void {
    toasts.value = []
  }

  return {
    toasts,
    showSuccess,
    showError,
    showInfo,
    showWarning,
    removeToast,
    clearAll
  }
}
