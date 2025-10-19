// TestCaseId値オブジェクト

use serde::{Deserialize, Serialize};

/// テストケースID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestCaseId(i64);

impl TestCaseId {
    /// 新しいTestCaseIdを作成
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
    fn test_test_case_id_new() {
        let id = TestCaseId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_test_case_id_equality() {
        let id1 = TestCaseId::new(1);
        let id2 = TestCaseId::new(1);
        let id3 = TestCaseId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
