// ProjectRepositoryトレイト

use super::{Project, ProjectCode};
use anyhow::Result;

/// プロジェクトリポジトリ
pub trait ProjectRepository: Send + Sync {
    /// プロジェクトコードでプロジェクトを取得
    fn get_by_code(&self, code: &ProjectCode) -> Result<Option<Project>>;

    /// すべてのプロジェクトを取得
    fn get_all(&self) -> Result<Vec<Project>>;

    /// プロジェクトを保存（新規作成または更新）
    fn save(&self, project: &Project) -> Result<()>;

    /// プロジェクトを削除
    fn delete(&self, code: &ProjectCode) -> Result<()>;
}
