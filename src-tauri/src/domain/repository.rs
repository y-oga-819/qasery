// Repository traits (interfaces) for data persistence

// Phase 1実装時点ではまだ使用されていないが、将来のPhaseで使用するため許可
#![allow(dead_code)]

use super::models::{Project, TestCase, TestCaseStep, TestSuite};
use anyhow::Result;

/// プロジェクトリポジトリ
pub trait ProjectRepository: Send + Sync {
    /// 全プロジェクトを取得
    fn get_all(&self) -> Result<Vec<Project>>;

    /// コードでプロジェクトを取得
    fn get_by_code(&self, code: &str) -> Result<Option<Project>>;

    /// プロジェクトを保存（新規作成または更新）
    fn save(&self, project: &Project) -> Result<()>;

    /// プロジェクトを削除
    fn delete(&self, code: &str) -> Result<()>;
}

/// テストスイートリポジトリ
pub trait TestSuiteRepository: Send + Sync {
    /// プロジェクト別にテストスイートを取得
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuite>>;

    /// IDでテストスイートを取得
    fn get_by_id(&self, id: i64) -> Result<Option<TestSuite>>;

    /// テストスイートを保存（新規作成または更新）
    fn save(&self, suite: &TestSuite) -> Result<()>;

    /// テストスイートを削除
    fn delete(&self, id: i64) -> Result<()>;
}

/// テストケースリポジトリ
pub trait TestCaseRepository: Send + Sync {
    /// スイート別にテストケースを取得
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCase>>;

    /// IDでテストケースを取得
    fn get_by_id(&self, id: i64) -> Result<Option<TestCase>>;

    /// テストケースを保存（新規作成または更新）
    fn save(&self, test_case: &TestCase) -> Result<()>;

    /// テストケースを削除
    fn delete(&self, id: i64) -> Result<()>;

    /// テストケースのステップを取得
    fn get_steps(&self, test_case_id: i64) -> Result<Vec<TestCaseStep>>;

    /// テストケースのステップを保存
    fn save_steps(&self, test_case_id: i64, steps: &[TestCaseStep]) -> Result<()>;
}
