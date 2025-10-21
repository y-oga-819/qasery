// OrderedStep値オブジェクト（TestCase集約内のEntity）

use super::step_id::StepId;
use serde::{Deserialize, Serialize};

/// ステップの種別（TestCase集約内のEntity）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StepType {
    /// 固有ステップ（TestCaseに直接紐付く）
    #[serde(rename = "own")]
    Own {
        id: StepId,
        action: String,
        expected_result: Option<String>,
        data: Option<String>,
    },
    /// 共有ステップ（SharedStepを参照）
    #[serde(rename = "shared")]
    Shared {
        hash: String, // SharedStepIdはString型
    },
}

/// 順序付きステップ（TestCase集約内のEntity）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderedStep {
    step_type: StepType,
    position: i32,
}

impl OrderedStep {
    /// 固有ステップを作成
    pub fn new_own(
        step_id: StepId,
        position: i32,
        action: String,
        expected_result: Option<String>,
        data: Option<String>,
    ) -> Self {
        Self {
            step_type: StepType::Own {
                id: step_id,
                action,
                expected_result,
                data,
            },
            position,
        }
    }

    /// 共有ステップを作成
    pub fn new_shared(shared_hash: String, position: i32) -> Self {
        Self {
            step_type: StepType::Shared { hash: shared_hash },
            position,
        }
    }

    /// ステップタイプを取得
    pub fn step_type(&self) -> &StepType {
        &self.step_type
    }

    /// 位置を取得
    pub fn position(&self) -> i32 {
        self.position
    }

    /// 位置を設定
    pub fn set_position(&mut self, new_position: i32) {
        self.position = new_position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_step_own() {
        let step = OrderedStep::new_own(
            StepId::new(1),
            0,
            "テストアクション".to_string(),
            Some("期待結果".to_string()),
            None,
        );
        assert_eq!(step.position(), 0);
        assert!(matches!(step.step_type(), StepType::Own { .. }));
    }

    #[test]
    fn test_ordered_step_shared() {
        let step = OrderedStep::new_shared("shared_hash".to_string(), 1);
        assert_eq!(step.position(), 1);
        assert!(matches!(step.step_type(), StepType::Shared { .. }));
    }

    #[test]
    fn test_ordered_step_set_position() {
        let mut step = OrderedStep::new_own(
            StepId::new(1),
            0,
            "テストアクション".to_string(),
            Some("期待結果".to_string()),
            None,
        );
        step.set_position(5);
        assert_eq!(step.position(), 5);
    }
}
