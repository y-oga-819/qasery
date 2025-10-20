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

#[allow(dead_code)]
impl SyncMetadata {
    /// 新しい同期メタデータを作成
    pub fn new() -> Self {
        Self {
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        }
    }

    /// 同期状態を取得
    pub fn sync_status(&self) -> SyncStatus {
        self.sync_status
    }

    /// 削除フラグを取得
    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    /// ローカル更新日時を取得
    pub fn local_updated_at(&self) -> DateTime<Utc> {
        self.local_updated_at
    }

    /// リモート更新日時を取得
    pub fn remote_updated_at(&self) -> Option<DateTime<Utc>> {
        self.remote_updated_at
    }

    /// ローカル変更としてマーク
    pub fn mark_local_modified(&mut self) {
        self.local_updated_at = Utc::now();
        self.sync_status = SyncStatus::LocalModified;
    }

    /// 同期完了としてマーク
    pub fn mark_synced(&mut self, remote_time: DateTime<Utc>) {
        self.remote_updated_at = Some(remote_time);
        self.sync_status = SyncStatus::Clean;
    }

    /// コンフリクトを検出
    pub fn detect_conflict(&mut self, remote_time: DateTime<Utc>) {
        self.remote_updated_at = Some(remote_time);
        self.sync_status = SyncStatus::Conflict;
    }

    /// 削除マークを付ける
    pub fn mark_deleted(&mut self) {
        self.is_deleted = true;
        self.mark_local_modified();
    }
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
