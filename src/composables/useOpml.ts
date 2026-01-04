import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { useToast } from './useToast'

/**
 * OPML 导入/导出功能
 */
export function useOpml() {
  const { showSuccess, showError, showInfo } = useToast()

  /**
   * 导出所有 Feeds 为 OPML 文件
   */
  async function exportOpml(): Promise<void> {
    try {
      // 1. 调用后端导出命令,获取 OPML XML 内容
      const opmlXml = await invoke<string>('export_opml')

      // 2. 打开文件保存对话框
      const filePath = await save({
        defaultPath: `scx-rss-feeds-${new Date().toISOString().split('T')[0]}.opml`,
        filters: [
          {
            name: 'OPML Files',
            extensions: ['opml', 'xml']
          },
          {
            name: 'All Files',
            extensions: ['*']
          }
        ]
      })

      if (!filePath) {
        showInfo('Export cancelled')
        return
      }

      // 3. 写入文件
      await writeTextFile(filePath, opmlXml)

      showSuccess(`OPML exported successfully to ${filePath}`)
    } catch (error) {
      console.error('Failed to export OPML:', error)
      showError(`Failed to export OPML: ${error}`)
    }
  }

  /**
   * 从 OPML 文件导入 Feeds
   */
  async function importOpml(): Promise<void> {
    try {
      // 1. 打开文件选择对话框
      // 注意: Tauri v2 的 open dialog 在 plugin-dialog 中
      const { open } = await import('@tauri-apps/plugin-dialog')

      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'OPML Files',
            extensions: ['opml', 'xml']
          },
          {
            name: 'All Files',
            extensions: ['*']
          }
        ]
      })

      if (!selected || typeof selected !== 'string') {
        showInfo('Import cancelled')
        return
      }

      // 2. 读取文件内容
      const opmlContent = await readTextFile(selected)

      // 3. 调用后端导入命令
      const result = await invoke<string>('import_opml', { opmlContent })

      // 4. 显示结果
      showSuccess(result)
    } catch (error) {
      console.error('Failed to import OPML:', error)
      showError(`Failed to import OPML: ${error}`)
    }
  }

  return {
    exportOpml,
    importOpml
  }
}
