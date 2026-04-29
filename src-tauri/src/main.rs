// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;

use commands::{db, feed};

fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 初始化数据库
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async {
                db::init_db(app_handle).await.expect("Failed to initialize database");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 数据库命令
            db::init_db,
            db::get_all_feeds,
            db::get_articles,
            db::update_article,
            db::delete_feed,
            db::get_categories,
            db::backup_database,
            db::restore_database,
            // Feed 命令
            feed::add_feed,
            feed::fetch_and_update_feed,
            feed::batch_refresh_feeds,
            feed::cancel_batch_refresh,
            feed::export_opml,
            feed::import_opml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
