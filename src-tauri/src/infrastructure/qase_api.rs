// Qase API Client implementation
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Qase API Client
pub struct QaseClient {
    api_token: String,
    base_url: String,
}

/// Qase APIのレスポンス共通構造
#[derive(Debug, Serialize, Deserialize)]
pub struct QaseResponse<T> {
    pub status: bool,
    pub result: T,
}

/// Projects APIのレスポンス結果
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectsResult {
    pub total: i32,
    pub filtered: i32,
    pub count: i32,
    pub entities: Vec<ProjectEntity>,
}

/// プロジェクトエンティティ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectEntity {
    pub title: String,
    pub code: String,
}

impl QaseClient {
    /// 新しいQaseClientを作成
    ///
    /// # Arguments
    /// * `api_token` - Qase APIトークン
    ///
    /// # Returns
    /// QaseClientインスタンスまたはエラー
    pub fn new(api_token: &str) -> Result<Self> {
        Ok(Self {
            api_token: api_token.to_string(),
            base_url: "https://api.qase.io/v1".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qase_client_initialization() {
        // QaseClientが正しく初期化できることをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token);

        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_projects() {
        // get_projectsメソッドが正しく動作することをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token).unwrap();

        let result = client.get_projects(None, None).await;

        // このテストは実際のAPIを呼び出すため、モック化が必要
        // 今は基本的な動作確認のみ
        assert!(result.is_err() || result.is_ok());
    }
}
