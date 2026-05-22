/// 获取系统语言
///
/// 返回系统首选语言代码，如 "zh-CN", "en-US" 等
#[tauri::command]
pub fn get_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "zh-CN".to_string())
}
