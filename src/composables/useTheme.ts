import { ref, watch, onScopeDispose } from 'vue'
import { useTheme as useVuetifyTheme } from 'vuetify'

export type ThemePreference = 'light' | 'dark' | 'system'
type ResolvedTheme = 'materialLight' | 'materialDark'

const THEME_KEY = 'scx-rss-theme'

function migrateLegacy(value: string | null): ThemePreference {
  if (value === 'dark' || value === 'light' || value === 'system') return value
  if (value === 'materialDark') return 'dark'
  return 'light'
}

const preference = ref<ThemePreference>(
  migrateLegacy(localStorage.getItem(THEME_KEY))
)

const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

function resolve(preference: ThemePreference): ResolvedTheme {
  if (preference === 'dark') return 'materialDark'
  if (preference === 'light') return 'materialLight'
  return mediaQuery.matches ? 'materialDark' : 'materialLight'
}

export function useTheme() {
  const theme = useVuetifyTheme()

  function setPreference(pref: ThemePreference) {
    preference.value = pref
    localStorage.setItem(THEME_KEY, pref)
  }

  function applyTheme(resolved: ResolvedTheme) {
    theme.global.name.value = resolved
  }

  function onMediaChange() {
    if (preference.value === 'system') {
      applyTheme(resolve('system'))
    }
  }

  mediaQuery.addEventListener('change', onMediaChange)
  onScopeDispose(() => mediaQuery.removeEventListener('change', onMediaChange))

  watch(preference, (pref) => {
    applyTheme(resolve(pref))
  })

  // Apply on init
  applyTheme(resolve(preference.value))

  return { preference, setPreference }
}
