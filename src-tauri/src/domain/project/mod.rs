// Project集約
// Phase 6-8で実際に使用されるまで未使用警告を抑制
#![allow(dead_code)]

mod project_code;
pub mod repository;

use crate::domain::common::sync_metadata::SyncMetadata;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use project_code::{ProjectCode, ProjectCodeError};
#[allow(unused_imports)]
pub use repository::ProjectRepository;

/// プロジェクト集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    // 識別子
    code: ProjectCode,

    // 基本情報
    title: String,
    description: Option<String>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl Project {
    /// 新しいプロジェクトを作成
    pub fn new(code: ProjectCode, title: String, description: Option<String>) -> Self {
        Self {
            code,
            title,
            description,
            sync_metadata: SyncMetadata::new(),
        }
    }

    /// プロジェクトコードを取得
    pub fn code(&self) -> &ProjectCode {
        &self.code
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 説明を取得
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// タイトルを更新
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.sync_metadata.mark_local_modified();
    }

    /// 説明を更新
    pub fn update_description(&mut self, new_description: Option<String>) {
        self.description = new_description;
        self.sync_metadata.mark_local_modified();
    }

    /// 削除マークを付ける
    pub fn mark_for_deletion(&mut self) {
        self.sync_metadata.mark_deleted();
    }

    /// 同期メタデータを取得
    pub fn sync_metadata(&self) -> &SyncMetadata {
        &self.sync_metadata
    }

    /// 同期メタデータを可変参照で取得
    pub fn sync_metadata_mut(&mut self) -> &mut SyncMetadata {
        &mut self.sync_metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::common::sync_metadata::SyncStatus;

    #[test]
    fn test_project_new() {
        let code = ProjectCode::new("DEMO".to_string()).unwrap();
        let project = Project::new(
            code.clone(),
            "Demo Project".to_string(),
            Some("A demo project".to_string()),
        );

        assert_eq!(project.code().value(), "DEMO");
        assert_eq!(project.title(), "Demo Project");
        assert_eq!(project.description(), Some("A demo project"));
        assert_eq!(project.sync_metadata().sync_status(), SyncStatus::Clean);
    }

    #[test]
    fn test_project_update_title() {
        let code = ProjectCode::new("DEMO".to_string()).unwrap();
        let mut project = Project::new(code, "Old Title".to_string(), None);

        project.update_title("New Title".to_string());

        assert_eq!(project.title(), "New Title");
        assert_eq!(
            project.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }

    #[test]
    fn test_project_update_description() {
        let code = ProjectCode::new("DEMO".to_string()).unwrap();
        let mut project = Project::new(code, "Project".to_string(), None);

        project.update_description(Some("New description".to_string()));

        assert_eq!(project.description(), Some("New description"));
        assert_eq!(
            project.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }

    #[test]
    fn test_project_mark_for_deletion() {
        let code = ProjectCode::new("DEMO".to_string()).unwrap();
        let mut project = Project::new(code, "Project".to_string(), None);

        project.mark_for_deletion();

        assert!(project.sync_metadata().is_deleted());
        assert_eq!(
            project.sync_metadata().sync_status(),
            SyncStatus::LocalModified
        );
    }
}
