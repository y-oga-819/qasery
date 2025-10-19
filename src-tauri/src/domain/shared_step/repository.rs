// SharedStepRepositoryトレイト

use super::{SharedStep, SharedStepId};
use anyhow::Result;

/// 共有ステップリポジトリ
pub trait SharedStepRepository: Send + Sync {
    /// IDで共有ステップを取得
    fn get_by_id(&self, id: &SharedStepId) -> Result<Option<SharedStep>>;

    /// プロジェクトコードで共有ステップを取得
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStep>>;

    /// 共有ステップを保存（新規作成または更新）
    fn save(&self, shared_step: &SharedStep) -> Result<()>;

    /// 共有ステップを削除
    fn delete(&self, id: &SharedStepId) -> Result<()>;
}
