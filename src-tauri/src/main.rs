// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::appstate::AppState;
use presentation::commands::{
    get_api_token, get_case, get_cases_by_suite, get_project, get_suite, get_suites_tree,
    list_projects, save_api_token, save_case, sync_cases, sync_projects, sync_suites,
};

fn main() {
    // アプリケーション状態を初期化
    let app_state = AppState::new("qasery.db").expect("Failed to initialize AppState");

    tauri::Builder::default()
        .manage(app_state) // AppStateをTauriに登録
        .invoke_handler(tauri::generate_handler![
            // プロジェクト関連コマンド
            list_projects,
            get_project,
            sync_projects,
            // テストスイート関連コマンド
            get_suites_tree,
            get_suite,
            sync_suites,
            // テストケース関連コマンド
            get_cases_by_suite,
            get_case,
            save_case,
            sync_cases,
            // API トークン関連コマンド
            save_api_token,
            get_api_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
