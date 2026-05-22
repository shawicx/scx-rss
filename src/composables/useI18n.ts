import { useI18n as useVueI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { ref, type Ref } from 'vue'

export type SupportedLocale = 'zh-CN' | 'en' | 'system'

export function useI18n() {
  const { locale, t } = useVueI18n<SupportLocale>()
  const initialized = ref(false)

  /**
   * 从 Rust 后端获取系统语言
   */
  const getSystemLocale = async (): Promise<'zh-CN' | 'en'> => {
    try {
      const systemLang = await invoke<string>('get_system_locale')

      // 解析系统语言
      if (systemLang.startsWith('zh')) {
        return 'zh-CN'
      }
      if (systemLang.startsWith('en')) {
        return 'en'
      }

      return 'zh-CN' // 默认中文
    } catch {
      return 'zh-CN' // 出错时默认中文
    }
  }

  /**
   * 初始化 i18n
   */
  const init = async () => {
    try {
      const stored = await invoke<string | null>('get_user_setting', {
        key: 'language',
      })

      if (stored === 'system' || !stored) {
        locale.value = await getSystemLocale()
      } else if (stored === 'zh-CN' || stored === 'en') {
        locale.value = stored
      } else {
        // 无效值，回退到系统语言
        locale.value = await getSystemLocale()
      }

      initialized.value = true
    } catch (error) {
      console.error('Failed to initialize i18n:', error)
      locale.value = await getSystemLocale()
      initialized.value = true
    }
  }

  /**
   * 设置语言
   */
  const setLocale = async (newLocale: SupportedLocale) => {
    try {
      await invoke('set_user_setting', {
        key: 'language',
        value: newLocale,
      })

      if (newLocale === 'system') {
        locale.value = await getSystemLocale()
      } else {
        locale.value = newLocale
      }
    } catch (error) {
      console.error('Failed to set locale:', error)
      throw error
    }
  }

  return {
    locale,
    t,
    initialized: initialized as Ref<boolean>,
    setLocale,
    init,
    getSystemLocale,
  }
}
