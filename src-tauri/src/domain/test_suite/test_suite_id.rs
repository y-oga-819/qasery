// TestSuiteId値オブジェクト

use serde::{Deserialize, Serialize};

/// テストスイートID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestSuiteId(i64);

impl TestSuiteId {
    /// 新しいTestSuiteIdを作成
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
    fn test_test_suite_id_new() {
        // TestSuiteId::new()が存在しないため失敗する（Red）
        let id = TestSuiteId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_test_suite_id_equality() {
        // 同じ値のIDは等しい
        let id1 = TestSuiteId::new(1);
        let id2 = TestSuiteId::new(1);
        let id3 = TestSuiteId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
