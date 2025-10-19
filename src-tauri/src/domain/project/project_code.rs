// ProjectCode値オブジェクト

use serde::{Deserialize, Serialize};

/// プロジェクトコードのエラー
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectCodeError {
    /// コードの長さが不正（2-10文字）
    InvalidLength(usize),
    /// 許可されていない文字を含む
    InvalidCharacters,
}

/// プロジェクトコード（2-10文字の識別子）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectCode(String);

impl ProjectCode {
    /// プロジェクトコードを作成
    ///
    /// # Errors
    /// - コードが2文字未満または10文字を超える場合
    /// - 許可されていない文字を含む場合（英数字、ハイフン、アンダースコアのみ許可）
    pub fn new(value: String) -> Result<Self, ProjectCodeError> {
        // 長さチェック
        if value.len() < 2 || value.len() > 10 {
            return Err(ProjectCodeError::InvalidLength(value.len()));
        }
        
        // 文字種チェック（英数字とハイフン、アンダースコアのみ許可）
        if !value.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ProjectCodeError::InvalidCharacters);
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
    fn test_project_code_valid() {
        // 有効なプロジェクトコード
        let code = ProjectCode::new("DEMO".to_string());
        assert!(code.is_ok());
        assert_eq!(code.unwrap().value(), "DEMO");

        let code = ProjectCode::new("project-1".to_string());
        assert!(code.is_ok());

        let code = ProjectCode::new("test_proj".to_string());
        assert!(code.is_ok());
    }

    #[test]
    fn test_project_code_too_short() {
        // 短すぎるコード
        let code = ProjectCode::new("A".to_string());
        assert_eq!(code, Err(ProjectCodeError::InvalidLength(1)));
    }

    #[test]
    fn test_project_code_too_long() {
        // 長すぎるコード
        let code = ProjectCode::new("VERYLONGCODE".to_string());
        assert_eq!(code, Err(ProjectCodeError::InvalidLength(12)));
    }

    #[test]
    fn test_project_code_invalid_chars() {
        // 許可されていない文字
        let code = ProjectCode::new("test@code".to_string());
        assert_eq!(code, Err(ProjectCodeError::InvalidCharacters));

        let code = ProjectCode::new("test code".to_string());
        assert_eq!(code, Err(ProjectCodeError::InvalidCharacters));
    }
}
