// Tauri commands (functions called from frontend)
//
// Phase 1: Query側をReadModelに移行
// Command側（sync_*, save_*）は次フェーズで実装

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::application::appstate::AppState;
use crate::domain::queries::{
    ProjectQueryRepository, ProjectReadModel, SharedStepQueryRepository, SharedStepReadModel,
    TestCaseQueryRepository, TestCaseReadModel, TestCaseStepReadModel, TestSuiteQueryRepository,
    TestSuiteReadModel,
};
use tauri::State;

/// プロジェクト一覧を取得
///
/// # Arguments
/// * `state` - アプリケーション状態
///
/// # Returns
/// プロジェクト一覧またはエラー
#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<ProjectReadModel>, String> {
    let query_repo = &*state.db as &dyn ProjectQueryRepository;
    query_repo.get_all().map_err(|e| e.to_string())
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
) -> Result<Option<ProjectReadModel>, String> {
    let query_repo = &*state.db as &dyn ProjectQueryRepository;
    query_repo.get_by_code(&code).map_err(|e| e.to_string())
}

// TODO: Phase 2で再実装
// /// Qaseからプロジェクトを同期
// ///
// /// # Arguments
// /// * `state` - アプリケーション状態
// ///
// /// # Returns
// /// 取り込んだプロジェクト数またはエラー
// #[tauri::command]
// pub async fn sync_projects(state: State<'_, AppState>) -> Result<usize, String> {
//     // Phase 2で新しいDomainモデルを使って再実装
//     todo!("Phase 2で実装")
// }

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
) -> Result<Vec<TestSuiteReadModel>, String> {
    let query_repo = &*state.db as &dyn TestSuiteQueryRepository;
    let suites = query_repo
        .get_by_project(&project_code)
        .map_err(|e| e.to_string())?;

    // TODO: 階層構造の構築が必要な場合は実装
    // 現時点ではフラットなリストを返す
    Ok(suites)
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
pub async fn get_suite(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<TestSuiteReadModel>, String> {
    let query_repo = &*state.db as &dyn TestSuiteQueryRepository;
    query_repo.get_by_id(id).map_err(|e| e.to_string())
}

// TODO: Phase 2で再実装
// /// Qaseからテストスイートを同期
// #[tauri::command]
// pub async fn sync_suites(
//     state: State<'_, AppState>,
//     project_code: String,
// ) -> Result<usize, String> {
//     todo!("Phase 2で実装")
// }

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
) -> Result<Vec<TestCaseReadModel>, String> {
    let query_repo = &*state.db as &dyn TestCaseQueryRepository;
    query_repo.get_by_suite(suite_id).map_err(|e| e.to_string())
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
pub async fn get_case(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<TestCaseReadModel>, String> {
    let query_repo = &*state.db as &dyn TestCaseQueryRepository;
    query_repo.get_by_id(id).map_err(|e| e.to_string())
}

// TODO: Phase 2で再実装
// /// テストケースを保存
// #[tauri::command]
// pub async fn save_case(state: State<'_, AppState>, test_case: TestCase) -> Result<(), String> {
//     todo!("Phase 2で実装")
// }

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
) -> Result<Vec<TestCaseStepReadModel>, String> {
    let query_repo = &*state.db as &dyn TestCaseQueryRepository;
    query_repo
        .get_steps(test_case_id)
        .map_err(|e| e.to_string())
}

// TODO: Phase 2で再実装
// /// Qaseからテストケースを同期
// #[tauri::command]
// pub async fn sync_cases(state: State<'_, AppState>, project_code: String) -> Result<usize, String> {
//     todo!("Phase 2で実装")
// }

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

/// プロジェクトのShared Step一覧を取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `project_code` - プロジェクトコード
///
/// # Returns
/// Shared Step一覧またはエラー
#[tauri::command]
pub async fn get_shared_steps(
    state: State<'_, AppState>,
    project_code: String,
) -> Result<Vec<SharedStepReadModel>, String> {
    let query_repo = &*state.db as &dyn SharedStepQueryRepository;
    query_repo
        .get_by_project(&project_code)
        .map_err(|e| e.to_string())
}

/// Shared Stepをハッシュで取得
///
/// # Arguments
/// * `state` - アプリケーション状態
/// * `hash` - Shared StepハッシュID
///
/// # Returns
/// Shared Stepまたはエラー
#[tauri::command]
pub async fn get_shared_step(
    state: State<'_, AppState>,
    hash: String,
) -> Result<Option<SharedStepReadModel>, String> {
    let query_repo = &*state.db as &dyn SharedStepQueryRepository;
    query_repo.get_by_hash(&hash).map_err(|e| e.to_string())
}

// TODO: Phase 2で再実装
// /// QaseからShared Stepを同期
// #[tauri::command]
// pub async fn sync_shared_steps(
//     state: State<'_, AppState>,
//     project_code: String,
// ) -> Result<usize, String> {
//     todo!("Phase 2で実装")
// }
