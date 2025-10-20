// TestSuite集約
// Phase 6-8で実際に使用されるまで未使用警告を抑制
#![allow(dead_code)]

pub mod repository;
mod test_suite_id;

use crate::domain::common::sync_metadata::SyncMetadata;
use crate::domain::project::ProjectCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use test_suite_id::TestSuiteId;

/// テストスイート集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    // 識別子
    id: TestSuiteId,
    project_code: ProjectCode,

    // 基本情報
    title: String,
    description: Option<String>,
    preconditions: Option<String>,

    // 階層構造
    parent_id: Option<TestSuiteId>,
    position: i32,

    // メタデータ
    cases_count: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,

    // 階層表示用（オプショナル）
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TestSuite>>,
}

impl TestSuite {
    /// 新しいテストスイートを作成
    pub fn new(id: TestSuiteId, title: String, project_code: ProjectCode) -> Self {
        let now = Utc::now();
        Self {
            id,
            project_code,
            title,
            description: None,
            preconditions: None,
            parent_id: None,
            position: 0,
            cases_count: 0,
            created_at: now,
            updated_at: now,
            sync_metadata: SyncMetadata::new(),
            children: None,
        }
    }

    /// Qase APIから取得したデータでテストスイートを構築（Builder パターン）
    #[allow(clippy::too_many_arguments)]
    pub fn from_api(
        id: TestSuiteId,
        project_code: ProjectCode,
        title: String,
        description: Option<String>,
        preconditions: Option<String>,
        position: i32,
        cases_count: i32,
        parent_id: Option<TestSuiteId>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            project_code,
            title,
            description,
            preconditions,
            parent_id,
            position,
            cases_count,
            created_at,
            updated_at,
            sync_metadata: SyncMetadata::new(),
            children: None,
        }
    }

    /// テストスイートIDを取得
    pub fn id(&self) -> TestSuiteId {
        self.id
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// プロジェクトコードを取得
    pub fn project_code(&self) -> &ProjectCode {
        &self.project_code
    }

    /// 表示順序を取得
    pub fn position(&self) -> i32 {
        self.position
    }

    /// 親IDを取得
    pub fn parent_id(&self) -> Option<TestSuiteId> {
        self.parent_id
    }

    /// 説明を取得
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// 前提条件を取得
    pub fn preconditions(&self) -> Option<&str> {
        self.preconditions.as_deref()
    }

    /// ケース数を取得
    pub fn cases_count(&self) -> i32 {
        self.cases_count
    }

    /// 作成日時を取得
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 更新日時を取得
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// 同期メタデータを取得
    pub fn sync_metadata(&self) -> &SyncMetadata {
        &self.sync_metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_suite_new() {
        let project_code = ProjectCode::new("PROJ".to_string()).unwrap();
        let suite = TestSuite::new(
            TestSuiteId::new(1),
            "Test Suite".to_string(),
            project_code.clone(),
        );

        assert_eq!(suite.id().value(), 1);
        assert_eq!(suite.title(), "Test Suite");
        assert_eq!(suite.project_code().value(), "PROJ");
        assert_eq!(suite.position(), 0);
        assert!(suite.parent_id().is_none());
    }
}
