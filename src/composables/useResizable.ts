import { ref, onMounted, onUnmounted } from 'vue'

interface ResizableOptions {
  storageKey: string
  defaultWidth: number
  minWidth: number
  maxWidth: number
}

export function useResizable(options: ResizableOptions) {
  const { storageKey, defaultWidth, minWidth, maxWidth } = options

  const width = ref(defaultWidth)
  const isResizing = ref(false)

  let startX = 0
  let startWidth = 0

  const onMouseMove = (e: MouseEvent) => {
    const delta = e.clientX - startX
    width.value = Math.min(maxWidth, Math.max(minWidth, startWidth + delta))
  }

  const onMouseUp = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
    localStorage.setItem(storageKey, String(width.value))
  }

  const onDragStart = (e: MouseEvent) => {
    e.preventDefault()
    isResizing.value = true
    startX = e.clientX
    startWidth = width.value
    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
    document.body.style.cursor = 'col-resize'
    document.body.style.userSelect = 'none'
  }

  onMounted(() => {
    const saved = localStorage.getItem(storageKey)
    if (saved) {
      const parsed = Number(saved)
      if (!isNaN(parsed) && parsed >= minWidth && parsed <= maxWidth) {
        width.value = parsed
      }
    }
  })

  onUnmounted(() => {
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
  })

  return { width, isResizing, onDragStart }
}
