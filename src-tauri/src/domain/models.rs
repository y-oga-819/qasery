// エンティティ定義 (Project, TestSuite, TestCase, SharedStep)

use serde::{Deserialize, Serialize};

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
}
