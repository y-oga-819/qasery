// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::appstate::AppState;
use presentation::commands::{
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
    sync_projects,
    sync_suites,
    // Phase 2で実装中: sync_cases, sync_shared_steps, save_case
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
            // テストケース関連コマンド（Query）
            get_cases_by_suite,
            get_case,
            get_case_steps,
            // Shared Step関連コマンド（Query）
            get_shared_steps,
            get_shared_step,
            // API トークン関連コマンド
            save_api_token,
            get_api_token,
            // Phase 2で実装中: sync_cases, sync_shared_steps, save_case
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
