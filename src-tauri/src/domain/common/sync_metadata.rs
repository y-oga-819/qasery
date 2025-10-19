// 同期メタデータ（全集約で共通）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ローカルとリモートの変更を追跡するための同期状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// 同期メタデータ（各集約が保持）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMetadata {
    local_updated_at: DateTime<Utc>,
    remote_updated_at: Option<DateTime<Utc>>,
    sync_status: SyncStatus,
    is_deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_metadata_new() {
        // SyncMetadata::new()が存在しないため失敗する（Red）
        let metadata = SyncMetadata::new();
        
        assert_eq!(metadata.sync_status(), SyncStatus::Clean);
        assert!(!metadata.is_deleted());
        assert!(metadata.remote_updated_at().is_none());
    }

    #[test]
    fn test_mark_local_modified() {
        // mark_local_modified()が存在しないため失敗する（Red）
        let mut metadata = SyncMetadata::new();
        metadata.mark_local_modified();
        
        assert_eq!(metadata.sync_status(), SyncStatus::LocalModified);
    }

    #[test]
    fn test_mark_synced() {
        // mark_synced()が存在しないため失敗する（Red）
        let mut metadata = SyncMetadata::new();
        let remote_time = Utc::now();
        
        metadata.mark_synced(remote_time);
        
        assert_eq!(metadata.sync_status(), SyncStatus::Clean);
        assert_eq!(metadata.remote_updated_at(), Some(remote_time));
    }

    #[test]
    fn test_detect_conflict() {
        // detect_conflict()が存在しないため失敗する（Red）
        let mut metadata = SyncMetadata::new();
        metadata.mark_local_modified();
        
        let remote_time = Utc::now();
        metadata.detect_conflict(remote_time);
        
        assert_eq!(metadata.sync_status(), SyncStatus::Conflict);
    }

    #[test]
    fn test_mark_deleted() {
        // mark_deleted()が存在しないため失敗する（Red）
        let mut metadata = SyncMetadata::new();
        metadata.mark_deleted();
        
        assert!(metadata.is_deleted());
        assert_eq!(metadata.sync_status(), SyncStatus::LocalModified);
    }
}
