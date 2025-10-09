// Business logic services (create, edit, delete, sync, etc.)

use crate::domain::models::Project;
use crate::domain::repository::ProjectRepository;
use anyhow::Result;
use std::sync::Arc;

/// プロジェクトサービス
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    /// 新しいProjectServiceを作成
    ///
    /// # Arguments
    /// * `repository` - プロジェクトリポジトリ
    ///
    /// # Returns
    /// ProjectServiceインスタンス
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }

    /// ローカルのプロジェクト一覧を取得
    ///
    /// # Returns
    /// プロジェクト一覧またはエラー
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.repository.get_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用のモックリポジトリ
    struct MockProjectRepository {
        projects: Vec<Project>,
    }

    impl MockProjectRepository {
        fn new() -> Self {
            Self {
                projects: Vec::new(),
            }
        }
    }

    impl ProjectRepository for MockProjectRepository {
        fn get_all(&self) -> Result<Vec<Project>> {
            Ok(self.projects.clone())
        }

        fn get_by_code(&self, _code: &str) -> Result<Option<Project>> {
            Ok(None)
        }

        fn save(&self, _project: &Project) -> Result<()> {
            Ok(())
        }

        fn delete(&self, _code: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_list_projects_returns_empty_list() {
        // 空のリポジトリでlist_projects()を呼ぶと空のリストが返る
        let repository = Arc::new(MockProjectRepository::new());
        let service = ProjectService::new(repository);

        let result = service.list_projects();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
