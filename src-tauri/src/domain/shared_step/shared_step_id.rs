// SharedStepId値オブジェクト

use serde::{Deserialize, Serialize};

/// 共有ステップID（Qase APIではハッシュ文字列）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SharedStepId(String);

impl SharedStepId {
    /// 新しいSharedStepIdを作成
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// 値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_step_id_new() {
        let id = SharedStepId::new("hash_abc123".to_string());
        assert_eq!(id.value(), "hash_abc123");
    }

    #[test]
    fn test_shared_step_id_equality() {
        let id1 = SharedStepId::new("hash1".to_string());
        let id2 = SharedStepId::new("hash1".to_string());
        let id3 = SharedStepId::new("hash2".to_string());

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
