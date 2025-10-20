//! Query用Repositoryトレイト定義
//!
//! CQRSパターンのQuery側で使用するRepositoryトレイト。
//! **参照専用**であり、データの取得のみを行う。
//!
//! **責務:**
//! - ReadModelの取得
//! - データ更新は含まない（Command側のRepositoryが担当）

use super::read_models::*;
use anyhow::Result;

/// プロジェクト参照用Repository
pub trait ProjectQueryRepository: Send + Sync {
    /// 全プロジェクトを取得
    fn get_all(&self) -> Result<Vec<ProjectReadModel>>;

    /// プロジェクトコードでプロジェクトを取得
    fn get_by_code(&self, code: &str) -> Result<Option<ProjectReadModel>>;
}

/// テストスイート参照用Repository
pub trait TestSuiteQueryRepository: Send + Sync {
    /// プロジェクトコードで全テストスイートを取得
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuiteReadModel>>;

    /// テストスイートIDでテストスイートを取得
    fn get_by_id(&self, id: i64) -> Result<Option<TestSuiteReadModel>>;
}

/// テストケース参照用Repository
pub trait TestCaseQueryRepository: Send + Sync {
    /// テストスイートIDで全テストケースを取得
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCaseReadModel>>;

    /// テストケースIDでテストケースを取得
    fn get_by_id(&self, id: i64) -> Result<Option<TestCaseReadModel>>;

    /// テストケースのステップを取得
    fn get_steps(&self, test_case_id: i64) -> Result<Vec<TestCaseStepReadModel>>;
}

/// Shared Step参照用Repository
pub trait SharedStepQueryRepository: Send + Sync {
    /// プロジェクトコードで全Shared Stepを取得
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStepReadModel>>;

    /// ハッシュIDでShared Stepを取得
    fn get_by_hash(&self, hash: &str) -> Result<Option<SharedStepReadModel>>;

    /// Shared Stepのステップを取得
    fn get_steps(&self, shared_step_hash: &str) -> Result<Vec<SharedStepStepReadModel>>;
}
