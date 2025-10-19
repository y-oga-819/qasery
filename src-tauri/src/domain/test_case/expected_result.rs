// ExpectedResult値オブジェクト

use serde::{Deserialize, Serialize};

/// 期待結果のエラー
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedResultError {
    /// 空文字列
    Empty,
}

/// 期待結果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedResult(String);

impl ExpectedResult {
    /// 新しい期待結果を作成
    ///
    /// # Errors
    /// - 空文字列の場合
    pub fn new(value: String) -> Result<Self, ExpectedResultError> {
        if value.trim().is_empty() {
            return Err(ExpectedResultError::Empty);
        }
        Ok(Self(value))
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
    fn test_expected_result_valid() {
        let result = ExpectedResult::new("Expected output".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "Expected output");
    }

    #[test]
    fn test_expected_result_empty() {
        let result = ExpectedResult::new("".to_string());
        assert_eq!(result, Err(ExpectedResultError::Empty));

        let result = ExpectedResult::new("   ".to_string());
        assert_eq!(result, Err(ExpectedResultError::Empty));
    }
}
