// Tauri commands (functions called from frontend)

use crate::domain::models::{Project, TestCase, TestSuite};
use anyhow::Result;

/// プロジェクト一覧を取得
///
/// # Returns
/// プロジェクト一覧またはエラー
#[tauri::command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    // Phase 1では未実装（アプリケーション初期化が必要）
    Err("Not implemented yet".to_string())
}

/// プロジェクトを取得
///
/// # Arguments
/// * `code` - プロジェクトコード
///
/// # Returns
/// プロジェクトまたはエラー
#[tauri::command]
pub async fn get_project(code: String) -> Result<Option<Project>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// Qaseからプロジェクトを同期
///
/// # Returns
/// 取り込んだプロジェクト数またはエラー
#[tauri::command]
pub async fn sync_projects() -> Result<usize, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// プロジェクトのテストスイート一覧を取得（階層構造）
///
/// # Arguments
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// テストスイート一覧またはエラー
#[tauri::command]
pub async fn get_suites_tree(project_code: String) -> Result<Vec<TestSuite>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// テストスイートを取得
///
/// # Arguments
/// * `id` - テストスイートID
///
/// # Returns
/// テストスイートまたはエラー
#[tauri::command]
pub async fn get_suite(id: i64) -> Result<Option<TestSuite>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// Qaseからテストスイートを同期
///
/// # Arguments
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// 取り込んだテストスイート数またはエラー
#[tauri::command]
pub async fn sync_suites(project_code: String) -> Result<usize, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// スイート別にテストケースを取得
///
/// # Arguments
/// * `suite_id` - テストスイートID
///
/// # Returns
/// テストケース一覧またはエラー
#[tauri::command]
pub async fn get_cases_by_suite(suite_id: i64) -> Result<Vec<TestCase>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// テストケースを取得
///
/// # Arguments
/// * `id` - テストケースID
///
/// # Returns
/// テストケースまたはエラー
#[tauri::command]
pub async fn get_case(id: i64) -> Result<Option<TestCase>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// テストケースを保存
///
/// # Arguments
/// * `test_case` - テストケース
///
/// # Returns
/// 成功またはエラー
#[tauri::command]
pub async fn save_case(test_case: TestCase) -> Result<(), String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// Qaseからテストケースを同期
///
/// # Arguments
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// 取り込んだテストケース数またはエラー
#[tauri::command]
pub async fn sync_cases(project_code: String) -> Result<usize, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}

/// APIトークンを保存
///
/// # Arguments
/// * `token` - APIトークン
///
/// # Returns
/// 成功またはエラー
#[tauri::command]
pub async fn save_api_token(token: String) -> Result<(), String> {
    // Phase 1では未実装（設定ファイル管理が必要）
    Err("Not implemented yet".to_string())
}

/// APIトークンを取得
///
/// # Returns
/// APIトークンまたはエラー
#[tauri::command]
pub async fn get_api_token() -> Result<Option<String>, String> {
    // Phase 1では未実装
    Err("Not implemented yet".to_string())
}
