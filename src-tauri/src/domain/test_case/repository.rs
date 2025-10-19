// TestCaseRepositoryトレイト

use super::test_case::TestCase;
use super::test_case_id::TestCaseId;
use crate::domain::test_suite::TestSuiteId;
use anyhow::Result;

/// テストケースリポジトリ
pub trait TestCaseRepository: Send + Sync {
    /// IDでテストケースを取得
    fn get_by_id(&self, id: TestCaseId) -> Result<Option<TestCase>>;
    
    /// スイートIDでテストケースを取得
    fn get_by_suite(&self, suite_id: TestSuiteId) -> Result<Vec<TestCase>>;
    
    /// テストケースを保存（新規作成または更新）
    fn save(&self, test_case: &TestCase) -> Result<()>;
    
    /// テストケースを削除
    fn delete(&self, id: TestCaseId) -> Result<()>;
}
