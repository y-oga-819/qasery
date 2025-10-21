// Application state management for Tauri

// Phase 1実装時点ではまだ一部機能が使用されていないため、警告を抑制
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::application::services::{ProjectService, TestCaseService, TestSuiteService};
use crate::infrastructure::db::Database;
use crate::infrastructure::keychain::KeychainService;
use crate::infrastructure::qase_api::QaseClient;
use anyhow::Result;
use std::sync::{Arc, Mutex};

/// アプリケーション全体の状態を管理
pub struct AppState {
    pub db: Arc<Database>,
    pub project_service: Arc<ProjectService>,
    pub test_suite_service: Arc<TestSuiteService>,
    pub test_case_service: Arc<TestCaseService>,
    pub api_token: Arc<Mutex<Option<String>>>,
}

impl AppState {
    /// 新しいAppStateを作成
    ///
    /// # Arguments
    /// * `db_path` - データベースファイルのパス
    ///
    /// # Returns
    /// AppStateインスタンスまたはエラー
    pub fn new(db_path: &str) -> Result<Self> {
        // データベースを初期化
        let db = Arc::new(Database::new(db_path)?);

        // Servicesを作成（初期状態ではQaseClientなし）
        let project_service = Arc::new(ProjectService::new(db.clone(), None));
        let test_suite_service = Arc::new(TestSuiteService::new(db.clone(), None));
        let test_case_service = Arc::new(TestCaseService::new(db.clone(), None));

        // OSキーチェーンからAPI Tokenを読み込み
        let stored_token = KeychainService::get_api_token().ok().flatten();

        Ok(Self {
            db,
            project_service,
            test_suite_service,
            test_case_service,
            api_token: Arc::new(Mutex::new(stored_token)),
        })
    }

    /// APIトークンを設定し、QaseClientを各Serviceに設定
    ///
    /// # Arguments
    /// * `token` - Qase APIトークン
    ///
    /// # Returns
    /// 成功またはエラー
    pub fn set_api_token(&self, token: String) -> Result<()> {
        // QaseClientを作成して検証
        let _client = Arc::new(QaseClient::new(&token)?);

        // OSキーチェーンに保存
        KeychainService::save_api_token(&token)?;

        // メモリ上のトークンも更新
        let mut api_token = self
            .api_token
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        *api_token = Some(token);

        Ok(())
    }

    /// APIトークンを取得
    ///
    /// # Returns
    /// APIトークンまたはNone
    pub fn get_api_token(&self) -> Result<Option<String>> {
        let api_token = self
            .api_token
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        Ok(api_token.clone())
    }

    /// API Tokenを削除
    ///
    /// # Returns
    /// 成功またはエラー
    pub fn delete_api_token(&self) -> Result<()> {
        // OSキーチェーンから削除
        KeychainService::delete_api_token()?;

        // メモリ上のトークンもクリア
        let mut api_token = self
            .api_token
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        *api_token = None;

        Ok(())
    }

    /// QaseClientを取得（APIトークンが設定されている場合）
    ///
    /// # Returns
    /// QaseClientまたはエラー
    pub fn get_qase_client(&self) -> Result<Arc<QaseClient>> {
        let api_token = self.get_api_token()?;
        let token = api_token.ok_or_else(|| anyhow::anyhow!("API token is not set"))?;
        Ok(Arc::new(QaseClient::new(&token)?))
    }
}
