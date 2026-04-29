import { ref, watch } from 'vue'
import { useTheme as useVuetifyTheme } from 'vuetify'

export type ThemeName = 'materialLight' | 'materialDark' | 'warmInk'

const THEME_KEY = 'scx-rss-theme'

const currentTheme = ref<ThemeName>(
  (localStorage.getItem(THEME_KEY) as ThemeName) || 'materialLight'
)

export function useTheme() {
  const theme = useVuetifyTheme()

  function setTheme(name: ThemeName) {
    currentTheme.value = name
    theme.global.name.value = name
    localStorage.setItem(THEME_KEY, name)
  }

  function initTheme() {
    theme.global.name.value = currentTheme.value
  }

  watch(currentTheme, (val) => {
    if (theme.global.name.value !== val) {
      theme.global.name.value = val
    }
  })

  return { currentTheme, setTheme, initTheme }
}
