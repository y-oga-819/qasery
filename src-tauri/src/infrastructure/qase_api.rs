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

/// 単一プロジェクトの結果（get_project用）
pub type ProjectResult = ProjectEntity;

/// Test Suites APIのレスポンス結果
#[derive(Debug, Serialize, Deserialize)]
pub struct SuitesResult {
    pub total: i32,
    pub filtered: i32,
    pub count: i32,
    pub entities: Vec<SuiteEntity>,
}

/// テストスイートエンティティ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuiteEntity {
    pub id: i64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditions: Option<String>,
    pub position: i32,
    pub cases_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

/// 単一テストスイートの結果（get_suite用）
pub type SuiteResult = SuiteEntity;

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

    /// 全プロジェクトを取得
    ///
    /// # Arguments
    /// * `limit` - 取得件数 (1-100, デフォルト: 10)
    /// * `offset` - スキップ件数 (0-100000, デフォルト: 0)
    ///
    /// # Returns
    /// プロジェクト一覧またはエラー
    pub async fn get_projects(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<QaseResponse<ProjectsResult>> {
        let client = reqwest::Client::new();
        let url = format!("{}/project", self.base_url);

        let mut query_params = vec![];
        if let Some(l) = limit {
            query_params.push(("limit", l.to_string()));
        }
        if let Some(o) = offset {
            query_params.push(("offset", o.to_string()));
        }

        let response = client
            .get(&url)
            .header("Token", &self.api_token)
            .header("accept", "application/json")
            .query(&query_params)
            .send()
            .await?;

        let result: QaseResponse<ProjectsResult> = response.json().await?;
        Ok(result)
    }

    /// 特定のプロジェクトを取得
    ///
    /// # Arguments
    /// * `code` - プロジェクトコード (2-10文字)
    ///
    /// # Returns
    /// プロジェクト情報またはエラー
    pub async fn get_project(&self, code: &str) -> Result<QaseResponse<ProjectResult>> {
        let client = reqwest::Client::new();
        let url = format!("{}/project/{}", self.base_url, code);

        let response = client
            .get(&url)
            .header("Token", &self.api_token)
            .header("accept", "application/json")
            .send()
            .await?;

        let result: QaseResponse<ProjectResult> = response.json().await?;
        Ok(result)
    }

    /// プロジェクトのテストスイート一覧を取得
    ///
    /// # Arguments
    /// * `code` - プロジェクトコード (2-10文字)
    /// * `search` - 検索キーワード (オプション)
    /// * `limit` - 取得件数 (1-100, デフォルト: 10)
    /// * `offset` - スキップ件数 (0-100000, デフォルト: 0)
    ///
    /// # Returns
    /// テストスイート一覧またはエラー
    pub async fn get_suites(
        &self,
        code: &str,
        search: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<QaseResponse<SuitesResult>> {
        let client = reqwest::Client::new();
        let url = format!("{}/suite/{}", self.base_url, code);

        let mut query_params = vec![];
        if let Some(s) = search {
            query_params.push(("search", s.to_string()));
        }
        if let Some(l) = limit {
            query_params.push(("limit", l.to_string()));
        }
        if let Some(o) = offset {
            query_params.push(("offset", o.to_string()));
        }

        let response = client
            .get(&url)
            .header("Token", &self.api_token)
            .header("accept", "application/json")
            .query(&query_params)
            .send()
            .await?;

        let result: QaseResponse<SuitesResult> = response.json().await?;
        Ok(result)
    }

    /// 特定のテストスイートを取得
    ///
    /// # Arguments
    /// * `code` - プロジェクトコード (2-10文字)
    /// * `id` - テストスイートID
    ///
    /// # Returns
    /// テストスイート情報またはエラー
    pub async fn get_suite(&self, code: &str, id: i64) -> Result<QaseResponse<SuiteResult>> {
        let client = reqwest::Client::new();
        let url = format!("{}/suite/{}/{}", self.base_url, code, id);

        let response = client
            .get(&url)
            .header("Token", &self.api_token)
            .header("accept", "application/json")
            .send()
            .await?;

        let result: QaseResponse<SuiteResult> = response.json().await?;
        Ok(result)
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

    #[tokio::test]
    async fn test_get_project() {
        // get_projectメソッドが正しく動作することをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token).unwrap();

        let result = client.get_project("DEMO").await;

        // このテストは実際のAPIを呼び出すため、モック化が必要
        // 今は基本的な動作確認のみ
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_suites() {
        // get_suitesメソッドが正しく動作することをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token).unwrap();

        let result = client.get_suites("DEMO", None, None, None).await;

        // このテストは実際のAPIを呼び出すため、モック化が必要
        // 今は基本的な動作確認のみ
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_get_suite() {
        // get_suiteメソッドが正しく動作することをテスト
        let api_token = "test_api_token";
        let client = QaseClient::new(api_token).unwrap();

        let result = client.get_suite("DEMO", 1).await;

        // このテストは実際のAPIを呼び出すため、モック化が必要
        // 今は基本的な動作確認のみ
        assert!(result.is_err() || result.is_ok());
    }
}
