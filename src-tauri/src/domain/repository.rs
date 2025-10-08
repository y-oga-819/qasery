// Repository traits (interfaces) for data persistence

use super::models::Project;
use anyhow::Result;

/// プロジェクトリポジトリ
pub trait ProjectRepository: Send + Sync {
    /// 全プロジェクトを取得
    fn get_all(&self) -> Result<Vec<Project>>;

    /// コードでプロジェクトを取得
    fn get_by_code(&self, code: &str) -> Result<Option<Project>>;

    /// プロジェクトを保存（新規作成または更新）
    fn save(&self, project: &Project) -> Result<()>;

    /// プロジェクトを削除
    fn delete(&self, code: &str) -> Result<()>;
}
