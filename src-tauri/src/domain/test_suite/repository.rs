//! TestSuite集約用のRepositoryトレイト

use super::{TestSuite, TestSuiteId};
use crate::domain::project::ProjectCode;
use anyhow::Result;

/// TestSuiteリポジトリトレイト
pub trait TestSuiteRepository {
    /// 全テストスイートを取得
    fn get_all(&self) -> Result<Vec<TestSuite>>;

    /// IDでテストスイートを取得
    fn get_by_id(&self, id: TestSuiteId) -> Result<Option<TestSuite>>;

    /// プロジェクトコードでテストスイート一覧を取得
    fn get_by_project(&self, project_code: &ProjectCode) -> Result<Vec<TestSuite>>;

    /// テストスイートを保存
    fn save(&self, test_suite: &TestSuite) -> Result<()>;

    /// テストスイートを削除（論理削除）
    fn delete(&self, id: TestSuiteId) -> Result<()>;
}
