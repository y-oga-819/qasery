// TestStepエンティティ

use super::expected_result::ExpectedResult;
use super::step_id::StepId;
use serde::{Deserialize, Serialize};

/// テストステップ（固有ステップ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    // 識別子
    id: StepId,

    // Qase API由来のメタデータ
    hash: String,

    // 基本情報
    action: String,
    expected_result: Option<ExpectedResult>,
    data: Option<String>,

    // 階層構造（ネストされたステップ用）
    parent_step_id: Option<StepId>,
}

impl TestStep {
    /// 新しいテストステップを作成
    pub fn new(
        id: StepId,
        hash: String,
        action: String,
        expected_result: Option<ExpectedResult>,
    ) -> Self {
        Self {
            id,
            hash,
            action,
            expected_result,
            data: None,
            parent_step_id: None,
        }
    }

    /// IDを取得
    pub fn id(&self) -> StepId {
        self.id
    }

    /// ハッシュを取得
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// アクションを取得
    pub fn action(&self) -> &str {
        &self.action
    }

    /// 期待結果を取得
    pub fn expected_result(&self) -> Option<&ExpectedResult> {
        self.expected_result.as_ref()
    }

    /// データを設定
    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    /// データを取得
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }

    /// 親ステップを設定（ネスト構造）
    pub fn set_parent(&mut self, parent_id: Option<StepId>) {
        self.parent_step_id = parent_id;
    }

    /// 親ステップIDを取得
    pub fn parent_step_id(&self) -> Option<StepId> {
        self.parent_step_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_step_new() {
        let expected = ExpectedResult::new("Expected output".to_string()).unwrap();
        let step = TestStep::new(
            StepId::new(1),
            "hash123".to_string(),
            "Click button".to_string(),
            Some(expected.clone()),
        );

        assert_eq!(step.id().value(), 1);
        assert_eq!(step.hash(), "hash123");
        assert_eq!(step.action(), "Click button");
        assert!(step.expected_result().is_some());
        assert_eq!(step.expected_result().unwrap().value(), "Expected output");
    }

    #[test]
    fn test_test_step_set_data() {
        let step = TestStep::new(
            StepId::new(1),
            "hash123".to_string(),
            "Input text".to_string(),
            None,
        );
        let mut step = step;
        step.set_data(Some("test data".to_string()));

        assert_eq!(step.data(), Some("test data"));
    }

    #[test]
    fn test_test_step_set_parent() {
        let mut step = TestStep::new(
            StepId::new(1),
            "hash123".to_string(),
            "Sub-step".to_string(),
            None,
        );
        step.set_parent(Some(StepId::new(10)));

        assert_eq!(step.parent_step_id(), Some(StepId::new(10)));
    }
}
