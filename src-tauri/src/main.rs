// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::appstate::AppState;
use presentation::commands::{
    delete_api_token,
    get_api_token,
    get_case,
    get_case_steps,
    get_cases_by_suite,
    get_project,
    get_shared_step,
    get_shared_step_steps,
    get_shared_steps,
    get_suite,
    get_suites_tree,
    list_projects,
    save_api_token,
    sync_cases,
    sync_projects,
    sync_shared_steps,
    sync_suites,
    // Phase 2で実装中: save_case
};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // アプリのデータディレクトリを取得
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            // ディレクトリが存在しない場合は作成
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            // データベースファイルのパスを構築
            let db_path = app_data_dir.join("qasery.db");
            let db_path_str = db_path
                .to_str()
                .expect("Failed to convert db path to string");

            // アプリケーション状態を初期化
            let app_state = AppState::new(db_path_str).expect("Failed to initialize AppState");
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // プロジェクト関連コマンド
            list_projects, // Query
            get_project,   // Query
            sync_projects, // Command
            // テストスイート関連コマンド
            get_suites_tree, // Query
            get_suite,       // Query
            sync_suites,     // Command
            // テストケース関連コマンド
            get_cases_by_suite, // Query
            get_case,           // Query
            get_case_steps,     // Query
            sync_cases,         // Command
            // Shared Step関連コマンド
            get_shared_steps,      // Query
            get_shared_step,       // Query
            get_shared_step_steps, // Query
            sync_shared_steps,     // Command
            // API トークン関連コマンド
            save_api_token,
            delete_api_token,
            get_api_token,
            // Phase 2で実装中: save_case
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
