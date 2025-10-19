// TestSuite集約ルート

use super::test_suite_id::TestSuiteId;
use crate::domain::common::sync_metadata::SyncMetadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// テストスイート集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    // 識別子
    id: TestSuiteId,
    project_code: String,

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
    pub fn new(id: TestSuiteId, title: String, project_code: String) -> Self {
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

    /// テストスイートIDを取得
    pub fn id(&self) -> TestSuiteId {
        self.id
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// プロジェクトコードを取得
    pub fn project_code(&self) -> &str {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_suite_new() {
        // TestSuite::new()が存在しないため失敗する（Red）
        let suite = TestSuite::new(
            TestSuiteId::new(1),
            "Test Suite".to_string(),
            "PROJ".to_string(),
        );

        assert_eq!(suite.id().value(), 1);
        assert_eq!(suite.title(), "Test Suite");
        assert_eq!(suite.project_code(), "PROJ");
        assert_eq!(suite.position(), 0);
        assert!(suite.parent_id().is_none());
    }
}
