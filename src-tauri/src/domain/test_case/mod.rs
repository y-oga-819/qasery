// TestCase集約
// Phase 6-8で実際に使用されるまで未使用警告を抑制
#![allow(dead_code)]

mod expected_result;
mod ordered_step;
pub mod repository;
mod step_id;
mod test_case_id;
mod test_step;

use crate::domain::common::sync_metadata::SyncMetadata;
use crate::domain::project::ProjectCode;
use crate::domain::test_suite::TestSuiteId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use expected_result::{ExpectedResult, ExpectedResultError};
#[allow(unused_imports)]
pub use ordered_step::{OrderedStep, StepType};
#[allow(unused_imports)]
pub use repository::TestCaseRepository;
#[allow(unused_imports)]
pub use step_id::StepId;
#[allow(unused_imports)]
pub use test_case_id::TestCaseId;
#[allow(unused_imports)]
pub use test_step::TestStep;

/// テストケース集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    // 識別子
    id: TestCaseId,
    project_code: ProjectCode,
    suite_id: TestSuiteId,

    // 基本情報
    title: String,
    description: Option<String>,
    preconditions: Option<String>,
    postconditions: Option<String>,

    // メタデータ
    severity: i32,
    priority: i32,
    test_type: i32,
    status: i32,
    automation: i32,
    author_id: Option<i64>,

    // ステップ（順序付き）
    steps: Vec<OrderedStep>,

    // Qase API由来のメタデータ
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl TestCase {
    /// 新しいテストケースを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: TestCaseId,
        title: String,
        suite_id: TestSuiteId,
        project_code: ProjectCode,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            project_code,
            suite_id,
            title,
            description: None,
            preconditions: None,
            postconditions: None,
            severity: 0,
            priority: 0,
            test_type: 0,
            status: 0,
            automation: 0,
            author_id: None,
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
            sync_metadata: SyncMetadata::new(),
        }
    }

    /// テストケースIDを取得
    pub fn id(&self) -> TestCaseId {
        self.id
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// スイートIDを取得
    pub fn suite_id(&self) -> TestSuiteId {
        self.suite_id
    }

    /// プロジェクトコードを取得
    pub fn project_code(&self) -> &ProjectCode {
        &self.project_code
    }

    /// ステップ一覧を取得
    pub fn steps(&self) -> &[OrderedStep] {
        &self.steps
    }

    /// ステップを追加
    pub fn add_step(&mut self, step: OrderedStep) {
        self.steps.push(step);
        self.sync_metadata.mark_local_modified();
    }

    /// ステップを削除
    pub fn remove_step(&mut self, position: i32) -> Result<(), String> {
        let index = self
            .steps
            .iter()
            .position(|s| s.position() == position)
            .ok_or_else(|| format!("Step at position {} not found", position))?;

        self.steps.remove(index);
        self.sync_metadata.mark_local_modified();
        Ok(())
    }

    /// 別のスイートに移動
    pub fn move_to_suite(&mut self, new_suite_id: TestSuiteId) {
        self.suite_id = new_suite_id;
        self.sync_metadata.mark_local_modified();
    }

    /// タイトルを更新
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.sync_metadata.mark_local_modified();
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
    fn test_test_case_new() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let test_case = TestCase::new(
            TestCaseId::new(1),
            "Test Login".to_string(),
            TestSuiteId::new(10),
            project_code.clone(),
        );

        assert_eq!(test_case.id().value(), 1);
        assert_eq!(test_case.title(), "Test Login");
        assert_eq!(test_case.suite_id().value(), 10);
        assert_eq!(test_case.project_code().value(), "PROJ");
        assert_eq!(test_case.steps().len(), 0);
    }

    #[test]
    fn test_test_case_add_step() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut test_case = TestCase::new(
            TestCaseId::new(1),
            "Test".to_string(),
            TestSuiteId::new(10),
            project_code.clone(),
        );

        let step = OrderedStep::new_own(StepId::new(1), 0);
        test_case.add_step(step);

        assert_eq!(test_case.steps().len(), 1);
        assert_eq!(
            test_case.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }

    #[test]
    fn test_test_case_remove_step() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut test_case = TestCase::new(
            TestCaseId::new(1),
            "Test".to_string(),
            TestSuiteId::new(10),
            project_code.clone(),
        );

        let step = OrderedStep::new_own(StepId::new(1), 0);
        test_case.add_step(step);

        let result = test_case.remove_step(0);
        assert!(result.is_ok());
        assert_eq!(test_case.steps().len(), 0);
    }

    #[test]
    fn test_test_case_move_to_suite() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let mut test_case = TestCase::new(
            TestCaseId::new(1),
            "Test".to_string(),
            TestSuiteId::new(10),
            project_code.clone(),
        );

        test_case.move_to_suite(TestSuiteId::new(20));

        assert_eq!(test_case.suite_id().value(), 20);
        assert_eq!(
            test_case.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }
}
