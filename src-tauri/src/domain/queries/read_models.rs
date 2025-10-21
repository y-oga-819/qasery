//! ReadModel定義（参照専用）
//!
//! CQRSパターンのQuery側で使用するReadModel。
//! 画面表示に最適化されたフラットな構造を持ち、すべてのフィールドがpublic。
//!
//! **責務:**
//! - データベースから取得したデータを画面に表示するための構造
//! - ビジネスロジックは含まない（Command側の集約が担当）
//! - シリアライズ/デシリアライズ可能（Tauri経由でフロントエンドに送信）

// 段階的実装中のため、未使用の警告を抑制
#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 同期状態（ReadModel用）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    /// ローカルとリモートが同期している
    Clean,
    /// ローカルで変更されている
    LocalModified,
    /// リモートで変更されている
    RemoteModified,
    /// 両方で変更されている（コンフリクト）
    Conflict,
}

/// プロジェクトReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectReadModel {
    /// プロジェクトコード (2-10文字の識別子)
    pub code: String,
    /// プロジェクト名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
}

/// テストスイートReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteReadModel {
    /// テストスイートID
    pub id: i64,
    /// プロジェクトコード
    pub project_code: String,
    /// テストスイート名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// 事前条件
    pub preconditions: Option<String>,
    /// 親テストスイートID（階層構造）
    pub parent_id: Option<i64>,
    /// 表示順序
    pub position: i32,
    /// このスイート内のテストケース数
    pub cases_count: i32,
    /// 作成日時（Qase API）
    pub created_at: DateTime<Utc>,
    /// 更新日時（Qase API）
    pub updated_at: DateTime<Utc>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
    /// 子スイート（階層構造表示用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TestSuiteReadModel>>,
}

/// テストケースReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseReadModel {
    /// テストケースID
    pub id: i64,
    /// プロジェクトコード
    pub project_code: String,
    /// 所属するテストスイートID
    pub suite_id: Option<i64>,
    /// テストケース名
    pub title: String,
    /// 説明
    pub description: Option<String>,
    /// 事前条件
    pub preconditions: Option<String>,
    /// 事後条件
    pub postconditions: Option<String>,
    /// 重要度
    pub severity: i32,
    /// 優先度
    pub priority: i32,
    /// テストタイプ
    pub test_type: i32,
    /// ステータス
    pub status: i32,
    /// 自動化レベル
    pub automation: i32,
    /// 作成者ID
    pub author_id: Option<i64>,
    /// 作成日時（Qase API）
    pub created_at: DateTime<Utc>,
    /// 更新日時（Qase API）
    pub updated_at: DateTime<Utc>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
}

/// テストケースステップReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseStepReadModel {
    /// ステップID（ローカルDB用）
    pub id: i64,
    /// 所属するテストケースID
    pub test_case_id: i64,
    /// ステップハッシュ（Qase API）
    pub hash: String,
    /// Shared StepハッシュID（参照している場合）
    pub shared_step_hash: Option<String>,
    /// 表示順序
    pub position: i32,
    /// アクション（実行内容）
    pub action: String,
    /// 期待される結果
    pub expected_result: Option<String>,
    /// データ（入力データなど）
    pub data: Option<String>,
    /// 親ステップID（ネストされたステップ用）
    pub parent_step_id: Option<i64>,
}

/// Shared StepReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStepReadModel {
    /// Shared StepハッシュID
    pub hash: String,
    /// プロジェクトコード
    pub project_code: String,
    /// Shared Step名
    pub title: String,
    /// このShared Stepを使用しているテストケース数
    pub cases_count: i32,
    /// 作成日時（Qase API）
    pub created_at: DateTime<Utc>,
    /// 更新日時（Qase API）
    pub updated_at: DateTime<Utc>,
    /// ローカル更新日時
    pub local_updated_at: DateTime<Utc>,
    /// リモート更新日時（前回同期時）
    pub remote_updated_at: Option<DateTime<Utc>>,
    /// 同期状態
    pub sync_status: SyncStatus,
    /// 削除フラグ
    pub is_deleted: bool,
}

/// Shared Step内のステップReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStepStepReadModel {
    /// ステップID（ローカルDB用）
    pub id: i64,
    /// 所属するShared StepハッシュID
    pub shared_step_hash: String,
    /// ステップハッシュ（Qase API）
    pub hash: String,
    /// 表示順序
    pub position: i32,
    /// アクション（実行内容）
    pub action: String,
    /// 期待される結果
    pub expected_result: Option<String>,
    /// データ（入力データなど）
    pub data: Option<String>,
    /// 親ステップID（ネストされたステップ用）
    pub parent_step_id: Option<i64>,
}
