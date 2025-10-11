// Tauri commands (functions called from frontend)

// Phase 1実装時点ではまだ一部のコマンドが未実装のため、警告を抑制
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::application::appstate::AppState;
use crate::application::services::{ProjectService, TestCaseService, TestSuiteService};
use crate::domain::models::{Project, TestCase, TestCaseStep, TestSuite};
use crate::domain::repository::TestCaseRepository;
use tauri::State;

/// プロジェクト一覧を取得
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// プロジェクト一覧またはエラー
#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state
        .project_service
        .list_projects()
        .map_err(|e| e.to_string())
}

/// プロジェクトを取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `code` - プロジェクトコード
///
/// # Returns
/// プロジェクトまたはエラー
#[tauri::command]
pub async fn get_project(
    state: State<'_, AppState>,
    code: String,
) -> Result<Option<Project>, String> {
    state
        .project_service
        .get_project(&code)
        .map_err(|e| e.to_string())
}

/// Qaseからプロジェクトを同期
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// 取り込んだプロジェクト数またはエラー
#[tauri::command]
pub async fn sync_projects(state: State<'_, AppState>) -> Result<usize, String> {
    // QaseClientを取得
    let client = state.get_qase_client().map_err(|e| e.to_string())?;

    // 一時的にServiceを作成（QaseClientを使用）
    let service = ProjectService::new(state.db.clone(), Some(client));

    service.sync_projects().await.map_err(|e| e.to_string())
}

/// プロジェクトのテストスイート一覧を取得（階層構造）
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// テストスイート一覧またはエラー
#[tauri::command]
pub async fn get_suites_tree(
    state: State<'_, AppState>,
    project_code: String,
) -> Result<Vec<TestSuite>, String> {
    state
        .test_suite_service
        .get_suites_tree(&project_code)
        .map_err(|e| e.to_string())
}

/// テストスイートを取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `id` - テストスイートID
///
/// # Returns
/// テストスイートまたはエラー
#[tauri::command]
pub async fn get_suite(state: State<'_, AppState>, id: i64) -> Result<Option<TestSuite>, String> {
    state
        .test_suite_service
        .get_suite(id)
        .map_err(|e| e.to_string())
}

/// Qaseからテストスイートを同期
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// 取り込んだテストスイート数またはエラー
#[tauri::command]
pub async fn sync_suites(
    state: State<'_, AppState>,
    project_code: String,
) -> Result<usize, String> {
    // QaseClientを取得
    let client = state.get_qase_client().map_err(|e| e.to_string())?;

    // 一時的にServiceを作成（QaseClientを使用）
    let service = TestSuiteService::new(state.db.clone(), Some(client));

    service
        .sync_suites(&project_code)
        .await
        .map_err(|e| e.to_string())
}

/// スイート別にテストケースを取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `suite_id` - テストスイートID
///
/// # Returns
/// テストケース一覧またはエラー
#[tauri::command]
pub async fn get_cases_by_suite(
    state: State<'_, AppState>,
    suite_id: i64,
) -> Result<Vec<TestCase>, String> {
    state
        .test_case_service
        .get_cases_by_suite(suite_id)
        .map_err(|e| e.to_string())
}

/// テストケースを取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `id` - テストケースID
///
/// # Returns
/// テストケースまたはエラー
#[tauri::command]
pub async fn get_case(state: State<'_, AppState>, id: i64) -> Result<Option<TestCase>, String> {
    state
        .test_case_service
        .get_case(id)
        .map_err(|e| e.to_string())
}

/// テストケースを保存
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `test_case` - テストケース
///
/// # Returns
/// 成功またはエラー
#[tauri::command]
pub async fn save_case(state: State<'_, AppState>, test_case: TestCase) -> Result<(), String> {
    state
        .test_case_service
        .save_case(&test_case)
        .map_err(|e| e.to_string())
}

/// テストケースのステップ一覧を取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `test_case_id` - テストケースID
///
/// # Returns
/// テストステップ一覧またはエラー
#[tauri::command]
pub async fn get_case_steps(
    state: State<'_, AppState>,
    test_case_id: i64,
) -> Result<Vec<TestCaseStep>, String> {
    state
        .db
        .get_steps(test_case_id)
        .map_err(|e| e.to_string())
}

/// Qaseからテストケースを同期
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// 取り込んだテストケース数またはエラー
#[tauri::command]
pub async fn sync_cases(state: State<'_, AppState>, project_code: String) -> Result<usize, String> {
    // QaseClientを取得
    let client = state.get_qase_client().map_err(|e| e.to_string())?;

    // 一時的にServiceを作成（QaseClientを使用）
    let service = TestCaseService::new(state.db.clone(), Some(client));

    service
        .sync_cases(&project_code)
        .await
        .map_err(|e| e.to_string())
}

/// APIトークンを保存
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `token` - APIトークン
///
/// # Returns
/// 成功またはエラー
#[tauri::command]
pub async fn save_api_token(state: State<'_, AppState>, token: String) -> Result<(), String> {
    state.set_api_token(token).map_err(|e| e.to_string())
}

/// APIトークンを取得
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// APIトークンまたはエラー
#[tauri::command]
pub async fn get_api_token(state: State<'_, AppState>) -> Result<Option<String>, String> {
    state.get_api_token().map_err(|e| e.to_string())
}
