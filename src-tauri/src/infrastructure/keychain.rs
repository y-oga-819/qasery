//! OSキーチェーンを使用したセキュアなAPI Token管理
//!
//! macOS Keychain、Windows Credential Manager、Linux Secret Service APIを
//! 統一的に扱うためのラッパーモジュール

use anyhow::{Context, Result};
use keyring::Entry;

/// Qaseryアプリケーション用のキーチェーンサービス名
const SERVICE_NAME: &str = "com.qasery.app";

/// API Tokenのエントリ名
const API_TOKEN_KEY: &str = "qase_api_token";

/// キーチェーンサービス
pub struct KeychainService;

impl KeychainService {
    /// API Tokenを保存
    ///
    /// # Arguments
    /// * `token` - 保存するAPI Token
    ///
    /// # Returns
    /// 成功またはエラー
    pub fn save_api_token(token: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, API_TOKEN_KEY)
            .context("Failed to create keychain entry")?;

        entry
            .set_password(token)
            .context("Failed to save API token to keychain")?;

        Ok(())
    }

    /// API Tokenを取得
    ///
    /// # Returns
    /// API TokenまたはNone（未設定の場合）
    pub fn get_api_token() -> Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, API_TOKEN_KEY)
            .context("Failed to create keychain entry")?;

        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e).context("Failed to get API token from keychain"),
        }
    }

    /// API Tokenを削除
    ///
    /// # Returns
    /// 成功またはエラー
    pub fn delete_api_token() -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, API_TOKEN_KEY)
            .context("Failed to create keychain entry")?;

        match entry.delete_password() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // 既に存在しない場合は成功とみなす
            Err(e) => Err(e).context("Failed to delete API token from keychain"),
        }
    }

    /// 内部用: カスタムのサービス名/キー名でAPI Tokenを削除（テスト用）
    ///
    /// # Arguments
    /// * `service_name` - サービス名
    /// * `key_name` - キー名
    ///
    /// # Returns
    /// 成功またはエラー
    #[cfg(test)]
    fn delete_api_token_internal(service_name: &str, key_name: &str) -> Result<()> {
        let entry = Entry::new(service_name, key_name)
            .context("Failed to create keychain entry")?;

        match entry.delete_password() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // 既に存在しない場合は成功とみなす
            Err(e) => Err(e).context("Failed to delete API token from keychain"),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// テスト用のサービス名（本番と分離）
    const TEST_SERVICE_NAME: &str = "com.qasery.app.test";
    const TEST_API_TOKEN_KEY: &str = "qase_api_token_test";

    /// テスト用のキーチェーンエントリをクリーンアップ
    fn cleanup_test_entry() {
        // エントリが存在する場合は削除、存在しない場合は何もしない
        let _ = KeychainService::delete_api_token_internal(TEST_SERVICE_NAME, TEST_API_TOKEN_KEY);
    }

    #[test]
    #[ignore] // OS Keychain に依存するため、手動実行時のみテスト
    fn test_save_and_get_api_token() {
        cleanup_test_entry();

        // テスト用のエントリを使用
        let entry = Entry::new(TEST_SERVICE_NAME, TEST_API_TOKEN_KEY).unwrap();

        // 保存
        let test_token = "test_api_token_12345";
        entry.set_password(test_token).unwrap();

        // 取得
        let retrieved = entry.get_password().unwrap();
        assert_eq!(test_token, retrieved);

        cleanup_test_entry();
    }

    #[test]
    #[ignore] // OS Keychain に依存するため、手動実行時のみテスト
    fn test_get_nonexistent_token() {
        cleanup_test_entry();

        let entry = Entry::new(TEST_SERVICE_NAME, TEST_API_TOKEN_KEY).unwrap();

        // 存在しないエントリを取得
        let result = entry.get_password();
        assert!(matches!(result, Err(keyring::Error::NoEntry)));

        cleanup_test_entry();
    }

    #[test]
    #[ignore] // OS Keychain に依存するため、手動実行時のみテスト
    fn test_delete_api_token() {
        cleanup_test_entry();

        let entry = Entry::new(TEST_SERVICE_NAME, TEST_API_TOKEN_KEY).unwrap();

        // 保存
        entry.set_password("test_token").unwrap();

        // 削除
        entry.delete_password().unwrap();

        // 削除されたことを確認
        let result = entry.get_password();
        assert!(matches!(result, Err(keyring::Error::NoEntry)));

        // 既に削除済みなのでcleanup不要
    }

    #[test]
    #[ignore] // OS Keychain に依存するため、手動実行時のみテスト
    fn test_update_api_token() {
        cleanup_test_entry();

        let entry = Entry::new(TEST_SERVICE_NAME, TEST_API_TOKEN_KEY).unwrap();

        // 最初のトークンを保存
        entry.set_password("old_token").unwrap();

        // 取得して確認
        let old_retrieved = entry.get_password().unwrap();
        assert_eq!("old_token", old_retrieved);

        // macOS Keychainでは既存エントリの更新前に削除が必要
        entry.delete_password().unwrap();
        
        // 新しいトークンを保存
        entry.set_password("new_token").unwrap();

        // 新しいトークンが取得できることを確認
        let new_retrieved = entry.get_password().unwrap();
        assert_eq!("new_token", new_retrieved);

        cleanup_test_entry();
    }
}
