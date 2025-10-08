// エンティティ定義 (Project, TestSuite, TestCase, SharedStep)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// プロジェクト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// プロジェクトコード (2-10文字の識別子)
    pub code: String,
    /// プロジェクト名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
}

/// テストスイート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// テストスイートID
    pub id: i64,
    /// プロジェクトコード
    pub project_code: String,
    /// テストスイート名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// 事前条件
    pub preconditions: Option<String>,
    /// 親テストスイートID（階層構造）
    pub parent_id: Option<i64>,
    /// 表示順序
    pub position: i32,
    /// このスイート内のテストケース数
    pub cases_count: i32,
    /// 作成日時（Qase API）
    pub created_at: DateTime<Utc>,
    /// 更新日時（Qase API）
    pub updated_at: DateTime<Utc>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
}

/// ローカルとリモートの変更を追跡するための同期状態
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    /// ローカルとリモートが同期している
    Clean,
    /// ローカルで変更されている
    LocalModified,
    /// リモートで変更されている
    RemoteModified,
    /// 両方で変更されている（コンフリクト）
    Conflict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        // Projectを作成できることを確認
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            local_updated_at: chrono::Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        assert_eq!(project.code, "TEST");
        assert_eq!(project.title, "Test Project");
        assert_eq!(project.sync_status, SyncStatus::Clean);
        assert_eq!(project.is_deleted, false);
    }

    #[test]
    fn test_test_suite_creation() {
        // TestSuiteを作成できることを確認
        let now = chrono::Utc::now();
        let test_suite = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite".to_string(),
            description: Some("Test Description".to_string()),
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: now,
            updated_at: now,
            local_updated_at: now,
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        assert_eq!(test_suite.id, 1);
        assert_eq!(test_suite.project_code, "TEST");
        assert_eq!(test_suite.title, "Test Suite");
        assert_eq!(test_suite.position, 1);
    }
}
