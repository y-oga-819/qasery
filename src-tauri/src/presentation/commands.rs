// Tauri commands (functions called from frontend)
//
// Phase 1: Query側をReadModelに移行 ✅
// Phase 2: Command側（sync_*, save_*）を実装 ← 現在

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::application::appstate::AppState;
use crate::domain::project::repository::ProjectRepository;
use crate::domain::project::{Project, ProjectCode};
use crate::domain::queries::{
    ProjectQueryRepository, ProjectReadModel, SharedStepQueryRepository, SharedStepReadModel,
    TestCaseQueryRepository, TestCaseReadModel, TestCaseStepReadModel, TestSuiteQueryRepository,
    TestSuiteReadModel,
};
use crate::domain::test_case::repository::TestCaseRepository;
use crate::domain::test_case::{OrderedStep, StepId, TestCase, TestCaseId};
use crate::domain::test_suite::repository::TestSuiteRepository;
use crate::domain::test_suite::{TestSuite, TestSuiteId};
use chrono::DateTime;
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

    let mut count = 0;
    let limit = 100;
    let mut offset = 0;

    loop {
        // Qase APIからプロジェクト一覧を取得（ページング）
        let response = client
            .get_projects(Some(limit), Some(offset))
            .await
            .map_err(|e| e.to_string())?;
        let entities = response.result.entities;
        let total = response.result.total;

        // 各プロジェクトをDomain Modelに変換して保存
        for entity in &entities {
            // ProjectCodeを作成（バリデーション付き）
            let code = ProjectCode::new(entity.code.clone())
                .map_err(|e| format!("Invalid project code: {:?}", e))?;

            // Domain Modelを作成
            let project = Project::new(code, entity.title.clone(), None);

            // Repositoryで永続化（クリーンアーキテクチャに従う）
            ProjectRepository::save(&*state.db, &project).map_err(|e| e.to_string())?;

            count += 1;
        }

        // 全件取得済みなら終了
        if offset + entities.len() as i32 >= total {
            break;
        }
        offset += limit;
    }

    Ok(count)
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

    // ProjectCodeを作成（バリデーション）
    let project_code_obj = ProjectCode::new(project_code.clone())
        .map_err(|e| format!("Invalid project code: {:?}", e))?;

    let mut count = 0;
    let limit = 100;
    let mut offset = 0;

    loop {
        // Qase APIからテストスイート一覧を取得（ページング）
        let response = client
            .get_suites(&project_code, None, Some(limit), Some(offset))
            .await
            .map_err(|e| e.to_string())?;
        let entities = response.result.entities;
        let total = response.result.total;

        // 各テストスイートをDomain Modelに変換して保存
        for entity in &entities {
            // TestSuiteIdを作成
            let suite_id = TestSuiteId::new(entity.id);

            // 日時文字列をパース（ISO 8601形式）
            let created_at = DateTime::parse_from_rfc3339(&entity.created_at)
                .map_err(|e| format!("Failed to parse created_at: {}", e))?
                .with_timezone(&chrono::Utc);
            let updated_at = DateTime::parse_from_rfc3339(&entity.updated_at)
                .map_err(|e| format!("Failed to parse updated_at: {}", e))?
                .with_timezone(&chrono::Utc);

            // Domain Modelを作成
            let test_suite = TestSuite::from_api(
                suite_id,
                project_code_obj.clone(),
                entity.title.clone(),
                entity.description.clone(),
                entity.preconditions.clone(),
                entity.position,
                entity.cases_count,
                entity.parent_id.map(TestSuiteId::new),
                created_at,
                updated_at,
            );

            // Repositoryで永続化（クリーンアーキテクチャに従う）
            TestSuiteRepository::save(&*state.db, &test_suite).map_err(|e| e.to_string())?;

            count += 1;
        }

        // 全件取得済みなら終了
        if offset + entities.len() as i32 >= total {
            break;
        }
        offset += limit;
    }

    Ok(count)
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

    // ProjectCodeを作成（バリデーション）
    let project_code_obj = ProjectCode::new(project_code.clone())
        .map_err(|e| format!("Invalid project code: {:?}", e))?;

    let mut count = 0;
    let limit = 100;
    let mut offset = 0;

    loop {
        // Qase APIからテストケース一覧を取得（ページング）
        let response = client
            .get_cases(&project_code, None, Some(limit), Some(offset))
            .await
            .map_err(|e| e.to_string())?;
        let entities = response.result.entities;
        let total = response.result.total;

        // 各テストケースをDomain Modelに変換して保存
        for entity in &entities {
            // suite_idがない場合はスキップ（Qase APIでは必須ではない）
            let suite_id = entity
                .suite_id
                .ok_or_else(|| format!("Test case {} has no suite_id", entity.id))?;

            // 日時文字列をパース（ISO 8601形式）
            let created_at = DateTime::parse_from_rfc3339(&entity.created_at)
                .map_err(|e| format!("Failed to parse created_at: {}", e))?
                .with_timezone(&chrono::Utc);
            let updated_at = DateTime::parse_from_rfc3339(&entity.updated_at)
                .map_err(|e| format!("Failed to parse updated_at: {}", e))?
                .with_timezone(&chrono::Utc);

            // ステップを変換
            let steps = if let Some(api_steps) = &entity.steps {
                api_steps
                    .iter()
                    .map(|step| {
                        if let Some(shared_hash) = &step.shared_step_hash {
                            // 共有ステップ
                            OrderedStep::new_shared(shared_hash.clone(), step.position)
                        } else {
                            // 固有ステップ（仮のIDを生成）
                            // 注意: Qase APIのstepには固有のIDがないため、positionベースで生成
                            let step_id = StepId::new(entity.id * 10000 + step.position as i64);
                            OrderedStep::new_own(step_id, step.position)
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

            // Domain Modelを作成
            let test_case = TestCase::from_api(
                TestCaseId::new(entity.id),
                project_code_obj.clone(),
                TestSuiteId::new(suite_id),
                entity.title.clone(),
                entity.description.clone(),
                entity.preconditions.clone(),
                entity.postconditions.clone(),
                entity.severity,
                entity.priority,
                entity.case_type,
                entity.status,
                entity.automation,
                Some(entity.author_id),
                steps,
                created_at,
                updated_at,
            );

            // Repositoryで永続化（クリーンアーキテクチャに従う）
            TestCaseRepository::save(&*state.db, &test_case).map_err(|e| e.to_string())?;

            count += 1;
        }

        // 全件取得済みなら終了
        if offset + entities.len() as i32 >= total {
            break;
        }
        offset += limit;
    }

    Ok(count)
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
