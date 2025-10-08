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
