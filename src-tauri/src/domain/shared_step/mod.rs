// SharedStep集約
// Phase 6-8で実際に使用されるまで未使用警告を抑制
#![allow(dead_code)]

pub mod repository;
mod shared_step_id;

use crate::domain::common::sync_metadata::SyncMetadata;
use crate::domain::project::ProjectCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use repository::SharedStepRepository;
#[allow(unused_imports)]
pub use shared_step_id::SharedStepId;

/// SharedStepの内部ステップ（エンティティ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStepStep {
    // 識別子
    id: i64,

    // Qase API由来のメタデータ
    hash: String,

    // 基本情報
    action: String,
    expected_result: Option<String>,
    data: Option<String>,

    // 階層構造（ネスト構造）
    parent_step_id: Option<i64>,

    // 順序
    position: i32,
}

impl SharedStepStep {
    /// 新しい内部ステップを作成
    pub fn new(
        id: i64,
        hash: String,
        action: String,
        expected_result: Option<String>,
        position: i32,
    ) -> Self {
        Self {
            id,
            hash,
            action,
            expected_result,
            data: None,
            parent_step_id: None,
            position,
        }
    }

    /// IDを取得
    pub fn id(&self) -> i64 {
        self.id
    }

    /// アクションを取得
    pub fn action(&self) -> &str {
        &self.action
    }

    /// 位置を取得
    pub fn position(&self) -> i32 {
        self.position
    }

    /// データを設定
    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    /// 親ステップを設定
    pub fn set_parent(&mut self, parent_id: Option<i64>) {
        self.parent_step_id = parent_id;
    }
}

/// 共有ステップ集約（集約ルート）
///
/// 複数のTestCaseで使い回すことが可能な、
/// 階層1の子要素（SharedStepStep）を持つTestStepのような存在
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStep {
    // 識別子
    id: SharedStepId,
    project_code: ProjectCode,

    // 基本情報
    title: String,

    // 内部ステップ（階層1の子要素）
    steps: Vec<SharedStepStep>,

    // 使用状況
    cases_count: i32,

    // Qase API由来のメタデータ
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl SharedStep {
    /// 新しい共有ステップを作成
    pub fn new(id: SharedStepId, title: String, project_code: ProjectCode) -> Self {
        let now = Utc::now();
        Self {
            id,
            project_code,
            title,
            steps: Vec::new(),
            cases_count: 0,
            created_at: now,
            updated_at: now,
            sync_metadata: SyncMetadata::new(),
        }
    }

    /// IDを取得
    pub fn id(&self) -> &SharedStepId {
        &self.id
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// プロジェクトコードを取得
    pub fn project_code(&self) -> &ProjectCode {
        &self.project_code
    }

    /// 内部ステップの一覧を取得
    pub fn steps(&self) -> &[SharedStepStep] {
        &self.steps
    }

    /// 使用中のテストケース数を取得
    pub fn cases_count(&self) -> i32 {
        self.cases_count
    }

    /// 内部ステップを追加
    pub fn add_step(&mut self, step: SharedStepStep) -> Result<(), String> {
        // 位置の重複チェック
        if self.steps.iter().any(|s| s.position == step.position) {
            return Err(format!("Step position {} already exists", step.position));
        }
        self.steps.push(step);
        self.sync_metadata.mark_local_modified();
        Ok(())
    }

    /// 内部ステップを削除
    pub fn remove_step(&mut self, position: i32) -> Result<(), String> {
        let index = self
            .steps
            .iter()
            .position(|s| s.position == position)
            .ok_or_else(|| format!("Step at position {} not found", position))?;

        self.steps.remove(index);
        self.sync_metadata.mark_local_modified();
        Ok(())
    }

    /// タイトルを更新
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.sync_metadata.mark_local_modified();
    }

    /// 削除前の検証（参照元テストケースが存在しないか確認）
    pub fn can_delete(&self) -> Result<(), String> {
        if self.cases_count > 0 {
            return Err(format!(
                "SharedStep is in use by {} test cases",
                self.cases_count
            ));
        }
        Ok(())
    }

    /// 同期メタデータを取得
    pub fn sync_metadata(&self) -> &SyncMetadata {
        &self.sync_metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::common::sync_metadata::SyncStatus;

    #[test]
    fn test_shared_step_new() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login Flow".to_string(),
            project_code.clone(),
        );

        assert_eq!(shared_step.id().value(), "hash123");
        assert_eq!(shared_step.title(), "Login Flow");
        assert_eq!(shared_step.project_code().value(), "PROJ");
        assert_eq!(shared_step.steps().len(), 0);
        assert_eq!(shared_step.cases_count(), 0);
    }

    #[test]
    fn test_shared_step_add_step() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login".to_string(),
            project_code.clone(),
        );

        let step = SharedStepStep::new(
            1,
            "step_hash".to_string(),
            "Enter username".to_string(),
            Some("Username entered".to_string()),
            0,
        );

        let result = shared_step.add_step(step);
        assert!(result.is_ok());
        assert_eq!(shared_step.steps().len(), 1);
        assert_eq!(
            shared_step.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }

    #[test]
    fn test_shared_step_add_duplicate_position() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login".to_string(),
            project_code.clone(),
        );

        let step1 = SharedStepStep::new(1, "hash1".to_string(), "Step 1".to_string(), None, 0);
        let step2 = SharedStepStep::new(2, "hash2".to_string(), "Step 2".to_string(), None, 0);

        shared_step.add_step(step1).unwrap();
        let result = shared_step.add_step(step2);

        assert!(result.is_err());
        assert_eq!(shared_step.steps().len(), 1);
    }

    #[test]
    fn test_shared_step_remove_step() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login".to_string(),
            project_code.clone(),
        );

        let step = SharedStepStep::new(1, "hash1".to_string(), "Step 1".to_string(), None, 0);
        shared_step.add_step(step).unwrap();

        let result = shared_step.remove_step(0);
        assert!(result.is_ok());
        assert_eq!(shared_step.steps().len(), 0);
    }

    #[test]
    fn test_shared_step_can_delete_when_in_use() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login".to_string(),
            project_code.clone(),
        );
        shared_step.cases_count = 5;

        let result = shared_step.can_delete();
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_step_can_delete_when_not_in_use() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Login".to_string(),
            project_code.clone(),
        );

        let result = shared_step.can_delete();
        assert!(result.is_ok());
    }

    #[test]
    fn test_shared_step_update_title() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut shared_step = SharedStep::new(
            SharedStepId::new("hash123".to_string()),
            "Old Title".to_string(),
            project_code.clone(),
        );

        shared_step.update_title("New Title".to_string());

        assert_eq!(shared_step.title(), "New Title");
        assert_eq!(
            shared_step.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }
}
