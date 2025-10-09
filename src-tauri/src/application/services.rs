// Business logic services (create, edit, delete, sync, etc.)

use crate::domain::models::{Project, SyncStatus, TestSuite};
use crate::domain::repository::{ProjectRepository, TestSuiteRepository};
use crate::infrastructure::qase_api::{ProjectEntity, QaseClient, SuiteEntity};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;

/// プロジェクトサービス
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
    qase_client: Option<Arc<QaseClient>>,
}

impl ProjectService {
    /// 新しいProjectServiceを作成
    ///
    /// # Arguments
    /// * `repository` - プロジェクトリポジトリ
    /// * `qase_client` - Qase APIクライアント（Optionalる）
    ///
    /// # Returns
    /// ProjectServiceインスタンス
    pub fn new(repository: Arc<dyn ProjectRepository>, qase_client: Option<Arc<QaseClient>>) -> Self {
        Self { repository, qase_client }
    }

    /// ローカルのプロジェクト一覧を取得
    ///
    /// # Returns
    /// プロジェクト一覧またはエラー
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.repository.get_all()
    }

    /// プロジェクトを取得
    ///
    /// # Arguments
    /// * `code` - プロジェクトコード
    ///
    /// # Returns
    /// プロジェクトまたはエラー
    pub fn get_project(&self, code: &str) -> Result<Option<Project>> {
        self.repository.get_by_code(code)
    }

    /// Qaseからプロジェクトを取り込み
    ///
    /// # Returns
    /// 取り込んだプロジェクト数またはエラー
    pub async fn sync_projects(&self) -> Result<usize> {
        let client = self.qase_client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Qase API client is not configured"))?;

        // Qase APIからプロジェクト一覧を取得
        let response = client.get_projects(None, None).await?;
        let entities = response.result.entities;

        // 各プロジェクトをローカルDBに保存
        let mut count = 0;
        for entity in entities {
            let project = self.convert_entity_to_project(&entity);
            self.repository.save(&project)?;
            count += 1;
        }

        Ok(count)
    }

    /// ProjectEntityをProjectに変換
    ///
    /// # Arguments
    /// * `entity` - Qase APIから取得したプロジェクトエンティティ
    ///
    /// # Returns
    /// Projectモデル
    fn convert_entity_to_project(&self, entity: &ProjectEntity) -> Project {
        let now = Utc::now();
        Project {
            code: entity.code.clone(),
            title: entity.title.clone(),
            description: None, // Phase 1では簡略化
            local_updated_at: now,
            remote_updated_at: Some(now),
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        }
    }
}

/// テストスイートサービス
pub struct TestSuiteService {
    repository: Arc<dyn TestSuiteRepository>,
    qase_client: Option<Arc<QaseClient>>,
}

impl TestSuiteService {
    /// 新しいTestSuiteServiceを作成
    ///
    /// # Arguments
    /// * `repository` - テストスイートリポジトリ
    /// * `qase_client` - Qase APIクライアント（Optional）
    ///
    /// # Returns
    /// TestSuiteServiceインスタンス
    pub fn new(repository: Arc<dyn TestSuiteRepository>, qase_client: Option<Arc<QaseClient>>) -> Self {
        Self { repository, qase_client }
    }

    /// プロジェクトのテストスイートを階層構造で取得
    ///
    /// # Arguments
    /// * `project_code` - プロジェクトコード
    ///
    /// # Returns
    /// テストスイート一覧またはエラー
    pub fn get_suites_tree(&self, project_code: &str) -> Result<Vec<TestSuite>> {
        // Phase 1では単純なリスト取得（階層構造は後で実装）
        self.repository.get_by_project(project_code)
    }

    /// テストスイートを取得
    ///
    /// # Arguments
    /// * `id` - テストスイートID
    ///
    /// # Returns
    /// テストスイートまたはエラー
    pub fn get_suite(&self, id: i64) -> Result<Option<TestSuite>> {
        self.repository.get_by_id(id)
    }

    /// Qaseからテストスイートを取り込み
    ///
    /// # Arguments
    /// * `project_code` - プロジェクトコード
    ///
    /// # Returns
    /// 取り込んだテストスイート数またはエラー
    pub async fn sync_suites(&self, project_code: &str) -> Result<usize> {
        let client = self.qase_client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Qase API client is not configured"))?;

        // Qase APIからテストスイート一覧を取得
        let response = client.get_suites(project_code, None, None, None).await?;
        let entities = response.result.entities;

        // 各テストスイートをローカルDBに保存
        let mut count = 0;
        for entity in entities {
            let suite = self.convert_entity_to_test_suite(&entity, project_code);
            self.repository.save(&suite)?;
            count += 1;
        }

        Ok(count)
    }

    /// SuiteEntityをTestSuiteに変換
    ///
    /// # Arguments
    /// * `entity` - Qase APIから取得したテストスイートエンティティ
    /// * `project_code` - プロジェクトコード
    ///
    /// # Returns
    /// TestSuiteモデル
    fn convert_entity_to_test_suite(&self, entity: &SuiteEntity, project_code: &str) -> TestSuite {
        let now = Utc::now();

        // Qase APIのcreated_at/updated_atはISO 8601形式の文字列
        // Phase 1では簡略化のため、パースエラーは現在時刻にフォールバック
        let created_at = entity.created_at.parse().unwrap_or(now);
        let updated_at = entity.updated_at.parse().unwrap_or(now);

        TestSuite {
            id: entity.id,
            project_code: project_code.to_string(),
            title: entity.title.clone(),
            description: entity.description.clone(),
            preconditions: entity.preconditions.clone(),
            parent_id: entity.parent_id,
            position: entity.position,
            cases_count: entity.cases_count,
            created_at,
            updated_at,
            local_updated_at: now,
            remote_updated_at: Some(updated_at),
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        }
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
        let service = ProjectService::new(repository, None);

        let result = service.list_projects();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_get_project_returns_none_when_not_found() {
        // プロジェクトが存在しない場合、Noneが返る
        let repository = Arc::new(MockProjectRepository::new());
        let service = ProjectService::new(repository, None);

        let result = service.get_project("DEMO");

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
