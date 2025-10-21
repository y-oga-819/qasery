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

fn main() {
    // アプリケーション状態を初期化
    let app_state = AppState::new("qasery.db").expect("Failed to initialize AppState");

    tauri::Builder::default()
        .manage(app_state) // AppStateをTauriに登録
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
            get_shared_steps,  // Query
            get_shared_step,   // Query
            sync_shared_steps, // Command
            // API トークン関連コマンド
            save_api_token,
            delete_api_token,
            get_api_token,
            // Phase 2で実装中: save_case
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
