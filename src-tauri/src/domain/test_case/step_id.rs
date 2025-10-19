// StepId値オブジェクト

use serde::{Deserialize, Serialize};

/// 固有ステップID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StepId(i64);

impl StepId {
    /// 新しいStepIdを作成
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// 値を取得
    pub fn value(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_id_new() {
        let id = StepId::new(1);
        assert_eq!(id.value(), 1);
    }
}
