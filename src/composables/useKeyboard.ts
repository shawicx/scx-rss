import { onMounted, onUnmounted } from 'vue'

interface KeyboardCallbacks {
  onNext: () => void
  onPrev: () => void
  onToggleRead: () => void
  onToggleStar: () => void
}

export function useKeyboard(callbacks: KeyboardCallbacks) {
  const handleKeydown = (e: KeyboardEvent) => {
    const tag = (e.target as HTMLElement).tagName
    if (tag === 'INPUT' || tag === 'TEXTAREA') return

    switch (e.key) {
      case 'j':
        e.preventDefault()
        callbacks.onNext()
        break
      case 'k':
        e.preventDefault()
        callbacks.onPrev()
        break
      case 'r':
        e.preventDefault()
        callbacks.onToggleRead()
        break
      case 's':
        e.preventDefault()
        callbacks.onToggleStar()
        break
    }
  }

  onMounted(() => document.addEventListener('keydown', handleKeydown))
  onUnmounted(() => document.removeEventListener('keydown', handleKeydown))
}
