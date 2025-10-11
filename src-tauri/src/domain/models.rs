// エンティティ定義 (Project, TestSuite, TestCase, SharedStep)

// Phase 1実装時点ではまだ使用されていないが、将来のPhaseで使用するため許可
#![allow(dead_code)]

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
    /// 子スイート（階層構造表示用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TestSuite>>,
}

/// テストケース
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// テストケースID
    pub id: i64,
    /// プロジェクトコード
    pub project_code: String,
    /// 所属するテストスイートID
    pub suite_id: Option<i64>,
    /// テストケース名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// 事前条件
    pub preconditions: Option<String>,
    /// 事後条件
    pub postconditions: Option<String>,
    /// 重要度
    pub severity: i32,
    /// 優先度
    pub priority: i32,
    /// テストタイプ
    pub test_type: i32,
    /// ステータス
    pub status: i32,
    /// 自動化レベル
    pub automation: i32,
    /// 作成者ID
    pub author_id: Option<i64>,
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

/// テストケースステップ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseStep {
    /// ステップID（ローカルDB用）
    pub id: i64,
    /// 所属するテストケースID
    pub test_case_id: i64,
    /// ステップハッシュ（Qase API）
    pub hash: String,
    /// Shared StepハッシュID（参照している場合）
    pub shared_step_hash: Option<String>,
    /// 表示順序
    pub position: i32,
    /// アクション（実行内容）
    pub action: String,
    /// 期待される結果
    pub expected_result: Option<String>,
    /// データ（入力データなど）
    pub data: Option<String>,
    /// 親ステップID（ネストされたステップ用）
    pub parent_step_id: Option<i64>,
}

/// Shared Step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStep {
    /// Shared StepハッシュID
    pub hash: String,
    /// プロジェクトコード
    pub project_code: String,
    /// Shared Step名
    pub title: String,
    /// アクション（実行内容）
    pub action: String,
    /// 期待される結果
    pub expected_result: String,
    /// このShared Stepを使用しているテストケース数
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

/// Shared Step内のステップ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStepStep {
    /// ステップID（ローカルDB用）
    pub id: i64,
    /// 所属するShared StepハッシュID
    pub shared_step_hash: String,
    /// ステップハッシュ（Qase API）
    pub hash: String,
    /// 表示順序
    pub position: i32,
    /// アクション（実行内容）
    pub action: String,
    /// 期待される結果
    pub expected_result: Option<String>,
    /// データ（入力データなど）
    pub data: Option<String>,
    /// 親ステップID（ネストされたステップ用）
    pub parent_step_id: Option<i64>,
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
        assert!(!project.is_deleted);
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
            children: None,
        };

        assert_eq!(test_suite.id, 1);
        assert_eq!(test_suite.project_code, "TEST");
        assert_eq!(test_suite.title, "Test Suite");
        assert_eq!(test_suite.position, 1);
    }

    #[test]
    fn test_test_case_creation() {
        // TestCaseを作成できることを確認
        let now = chrono::Utc::now();
        let test_case = TestCase {
            id: 1,
            project_code: "TEST".to_string(),
            suite_id: Some(1),
            title: "Test Case".to_string(),
            description: Some("Test Description".to_string()),
            preconditions: None,
            postconditions: None,
            severity: 1,
            priority: 1,
            test_type: 1,
            status: 1,
            automation: 0,
            author_id: Some(1),
            created_at: now,
            updated_at: now,
            local_updated_at: now,
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        assert_eq!(test_case.id, 1);
        assert_eq!(test_case.project_code, "TEST");
        assert_eq!(test_case.title, "Test Case");
        assert_eq!(test_case.suite_id, Some(1));
    }

    #[test]
    fn test_test_case_step_creation() {
        // TestCaseStepを作成できることを確認
        let step = TestCaseStep {
            id: 1,
            test_case_id: 1,
            hash: "test_hash".to_string(),
            shared_step_hash: None,
            position: 1,
            action: "テストアクション".to_string(),
            expected_result: Some("期待される結果".to_string()),
            data: None,
            parent_step_id: None,
        };

        assert_eq!(step.id, 1);
        assert_eq!(step.test_case_id, 1);
        assert_eq!(step.action, "テストアクション");
        assert_eq!(step.position, 1);
    }
}
