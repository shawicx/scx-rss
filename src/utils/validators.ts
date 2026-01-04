/**
 * 验证工具函数
 */

/**
 * 验证 URL 是否有效
 * @param url 要验证的 URL 字符串
 * @returns 是否为有效的 URL
 */
export function validateUrl(url: string): boolean {
  try {
    const urlObj = new URL(url)
    // 只支持 http 和 https 协议
    return urlObj.protocol === 'http:' || urlObj.protocol === 'https:'
  } catch {
    return false
  }
}

/**
 * 验证 Feed URL 是否有效
 * @param url Feed URL
 * @returns 验证结果和错误消息
 */
export function validateFeedUrl(url: string): { valid: boolean; error?: string } {
  if (!url || url.trim().length === 0) {
    return { valid: false, error: 'URL 不能为空' }
  }

  if (!validateUrl(url)) {
    return { valid: false, error: 'URL 格式无效' }
  }

  // 检查是否为常见的 RSS/Atom 扩展
  const commonExtensions = ['.xml', '.rss', '.atom', '.rdf']
  const hasValidExtension = commonExtensions.some((ext) =>
    url.toLowerCase().endsWith(ext)
  )

  // 注意：不强制要求扩展名，因为有些 feed URL 不带扩展名
  // 这里只是给出警告提示
  if (!hasValidExtension) {
    console.warn('URL 可能不是标准的 Feed URL:', url)
  }

  return { valid: true }
}

/**
 * 验证字符串是否为空
 * @param str 要验证的字符串
 * @returns 是否为空字符串
 */
export function isEmpty(str: string | null | undefined): boolean {
  return !str || str.trim().length === 0
}
