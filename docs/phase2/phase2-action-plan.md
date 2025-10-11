# Phase 2 アクションプラン - Qasery品質改善ロードマップ

**作成日**: 2025-10-11  
**対象**: Phase 2開発着手前の必須改善タスク  
**目標**: ミッションクリティカルな品質基準の達成

---

## 📋 概要

Phase 1コードレビューで特定された重大な問題に対処し、Phase 2以降の安定的な開発基盤を構築する。すべてのCriticalタスクをPhase 2機能開発前に完了させることが必須。

**見積もり工数**: 約2-3週間（1名のフルタイム開発者想定）

---

## 🔥 Phase 1: Critical Issues（必須対応）

### タスク1: Value Objectの導入とドメインモデル強化

**優先度**: ★★★★★  
**見積もり**: 3日  
**担当**: ドメイン設計・Rustバックエンド

#### 目的
型システムでビジネスルール（不変条件）を保証し、不正データの流入を防ぐ。

#### 実装内容

1. **ProjectCodeの実装**
```rust
// src-tauri/src/domain/value_objects.rs (新規作成)

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// プロジェクトコード（2-10文字の英数字）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectCode(String);

impl ProjectCode {
    /// 新しいProjectCodeを生成
    pub fn new(code: impl Into<String>) -> Result<Self> {
        let code = code.into();
        
        // 長さのバリデーション
        if code.len() < 2 || code.len() > 10 {
            return Err(anyhow!(
                "Project code must be between 2 and 10 characters, got: {}",
                code.len()
            ));
        }
        
        // 英数字のみ許可
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(anyhow!(
                "Project code must contain only alphanumeric characters"
            ));
        }
        
        Ok(Self(code.to_uppercase()))
    }
    
    /// 内部値への参照を取得
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_code() {
        assert!(ProjectCode::new("AB").is_ok());
        assert!(ProjectCode::new("TEST123").is_ok());
        assert!(ProjectCode::new("1234567890").is_ok());
    }

    #[test]
    fn test_invalid_project_code_length() {
        assert!(ProjectCode::new("A").is_err());        // 短すぎる
        assert!(ProjectCode::new("12345678901").is_err()); // 長すぎる
    }

    #[test]
    fn test_invalid_project_code_characters() {
        assert!(ProjectCode::new("TEST-01").is_err());  // ハイフン不可
        assert!(ProjectCode::new("test_01").is_err());  // アンダースコア不可
    }

    #[test]
    fn test_case_normalization() {
        let code = ProjectCode::new("test").unwrap();
        assert_eq!(code.as_str(), "TEST");
    }
}
```

2. **その他Value Objectsの実装**
   - `TestSuiteId(i64)`: 正の整数のみ許可
   - `TestCaseId(i64)`: 正の整数のみ許可
   - `SharedStepHash(String)`: SHA256ハッシュ形式の検証
   - `Severity(i32)`: 0-4の範囲制限
   - `Priority(i32)`: 0-4の範囲制限

3. **Projectモデルの更新**
```rust
// src-tauri/src/domain/models.rs

use crate::domain::value_objects::ProjectCode;

pub struct Project {
    pub code: ProjectCode,  // Stringから変更
    pub title: String,
    // ...
}
```

4. **Repositoryの更新**
   - `get_by_code(&self, code: &ProjectCode)`に変更
   - データベース層でのシリアライズ/デシリアライズ対応

#### 成果物
- [ ] `src-tauri/src/domain/value_objects.rs` 新規作成
- [ ] すべてのValue Objectに対する単体テスト
- [ ] ドメインモデルの更新
- [ ] Repository/Serviceレイヤーの更新
- [ ] 既存テストの修正（25テスト→35テストに増加想定）

#### 完了条件
- すべてのValue Objectが不変条件を強制
- 100%のテストカバレッジ
- `cargo test`がパス

---

### タスク2: Aggregate境界の明確化とトランザクション整理

**優先度**: ★★★★★  
**見積もり**: 2日  
**担当**: ドメイン設計・Rustバックエンド

#### 目的
Aggregateルートを通じたトランザクション整合性を保証する。

#### 実装内容

1. **TestCase Aggregateの定義**
```rust
// src-tauri/src/domain/models.rs

/// テストケースAggregate（ルート）
pub struct TestCase {
    pub id: TestCaseId,
    pub project_code: ProjectCode,
    // ...
    steps: Vec<TestCaseStep>, // privateフィールド化
}

impl TestCase {
    /// ステップを追加（不変条件を保証）
    pub fn add_step(&mut self, step: TestCaseStep) -> Result<()> {
        // ビジネスルール検証
        if self.steps.len() >= 100 {
            return Err(anyhow!("Cannot add more than 100 steps"));
        }
        
        // position の自動採番
        let next_position = self.steps.len() as i32 + 1;
        let mut step = step;
        step.position = next_position;
        
        self.steps.push(step);
        Ok(())
    }
    
    /// ステップ一覧を取得（読み取り専用）
    pub fn steps(&self) -> &[TestCaseStep] {
        &self.steps
    }
}
```

2. **Repository層でのAggregate単位保存**
```rust
// src-tauri/src/domain/repository.rs

pub trait TestCaseRepository: Send + Sync {
    /// TestCase Aggregate全体を保存（ステップ含む）
    fn save_aggregate(&self, test_case: &TestCase) -> Result<()>;
    
    // save_steps()メソッドは削除
}
```

3. **トランザクション境界の整理**
```rust
// src-tauri/src/infrastructure/db.rs

impl TestCaseRepository for Database {
    fn save_aggregate(&self, test_case: &TestCase) -> Result<()> {
        let mut conn = self.conn.lock()?;
        let tx = conn.transaction()?;
        
        // TestCase本体を保存
        tx.execute(/* ... */)?;
        
        // 既存ステップを削除
        tx.execute("DELETE FROM test_case_steps WHERE test_case_id = ?", [test_case.id])?;
        
        // 新しいステップを保存
        for step in test_case.steps() {
            tx.execute(/* ... */)?;
        }
        
        tx.commit()?;
        Ok(())
    }
}
```

#### 成果物
- [ ] TestCase Aggregateの実装
- [ ] SharedStep Aggregateの実装
- [ ] Repositoryインターフェースの更新
- [ ] トランザクションテストの追加

#### 完了条件
- すべてのAggregate操作がトランザクション境界内
- Aggregate rootを通じた操作のみ可能
- 部分的な状態更新が不可能

---

### タスク3: 層の依存関係修正

**優先度**: ★★★★★  
**見積もり**: 1日  
**担当**: バックエンド全般

#### 目的
Presentation層からInfrastructure層への直接依存を排除。

#### 実装内容

1. **TestStepService の追加**
```rust
// src-tauri/src/application/services.rs

pub struct TestStepService {
    repository: Arc<dyn TestCaseRepository>,
}

impl TestStepService {
    pub fn get_steps(&self, test_case_id: TestCaseId) -> Result<Vec<TestCaseStep>> {
        let test_case = self.repository.get_by_id(test_case_id)?
            .ok_or_else(|| anyhow!("TestCase not found"))?;
        Ok(test_case.steps().to_vec())
    }
}
```

2. **Presentation層の修正**
```rust
// src-tauri/src/presentation/commands.rs

#[tauri::command]
pub async fn get_case_steps(
    state: State<'_, AppState>,
    test_case_id: i64,
) -> Result<Vec<TestCaseStep>, String> {
    // 修正前: TestCaseRepository::get_steps(&*state.db, test_case_id)
    // 修正後:
    state
        .test_step_service  // 新規追加
        .get_steps(TestCaseId::new(test_case_id)?)
        .map_err(|e| e.to_string())
}
```

#### 成果物
- [ ] すべてのPresentation→Infrastructure参照を削除
- [ ] 必要に応じてServiceレイヤーにメソッド追加
- [ ] 依存関係グラフの可視化（`cargo-depgraph`）

#### 完了条件
- `presentation/`モジュールから`infrastructure/`への`use`文ゼロ
- すべてのTauri commandsがApplication層経由

---

### タスク4: 構造化エラーハンドリングの実装

**優先度**: ★★★★★  
**見積もり**: 2日  
**担当**: バックエンド全般

#### 目的
型安全なエラー処理により、デバッグ性と回復可能性を向上。

#### 実装内容

1. **ドメインエラーの定義**
```rust
// src-tauri/src/domain/error.rs (新規作成)

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid value: {field} - {message}")]
    InvalidValue { field: String, message: String },
    
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },
    
    #[error("Conflict detected: {message}")]
    Conflict { message: String },
    
    #[error("Business rule violation: {rule}")]
    BusinessRuleViolation { rule: String },
}

pub type DomainResult<T> = Result<T, DomainError>;
```

2. **Application層エラー**
```rust
// src-tauri/src/application/error.rs (新規作成)

use thiserror::Error;
use crate::domain::error::DomainError;
use crate::infrastructure::error::InfrastructureError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),
    
    #[error("Sync error: {0}")]
    Sync(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
```

3. **Infrastructure層エラー**
```rust
// src-tauri/src/infrastructure/error.rs (新規作成)

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("API error: {0}")]
    Api(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
```

4. **Presentation層でのエラー変換**
```rust
// src-tauri/src/presentation/commands.rs

use crate::application::error::ApplicationError;

#[derive(serde::Serialize)]
struct ErrorResponse {
    error_type: String,
    message: String,
}

impl From<ApplicationError> for ErrorResponse {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::Domain(e) => ErrorResponse {
                error_type: "DOMAIN_ERROR".to_string(),
                message: e.to_string(),
            },
            ApplicationError::Infrastructure(e) => ErrorResponse {
                error_type: "INFRASTRUCTURE_ERROR".to_string(),
                message: e.to_string(),
            },
            ApplicationError::Sync(msg) => ErrorResponse {
                error_type: "SYNC_ERROR".to_string(),
                message: msg,
            },
        }
    }
}

#[tauri::command]
pub async fn list_projects(
    state: State<'_, AppState>,
) -> Result<Vec<Project>, ErrorResponse> {
    state
        .project_service
        .list_projects()
        .map_err(|e| e.into())
}
```

5. **unwrap/unwrap_orの全削除**
   - `application/services.rs:301` の `unwrap_or(now)` を削除
   - パースエラーを適切にログ出力＆エラー返却

#### 成果物
- [ ] 各層のエラー型定義
- [ ] すべての`unwrap()`, `expect()`, `unwrap_or()`の削除
- [ ] エラー変換ロジックの実装
- [ ] エラーハンドリングのテスト

#### 完了条件
- `grep -r "unwrap\|expect" src-tauri/src/ | wc -l` が 0
- すべてのエラーが構造化され、型情報を保持
- フロントエンドでエラー種別に応じた適切なハンドリングが可能

---

### タスク5: APIトークンの安全な保存

**優先度**: ★★★★★  
**見積もり**: 1日  
**担当**: セキュリティ・バックエンド

#### 目的
APIトークンをOSのキーチェーンに保存し、セキュリティリスクを低減。

#### 実装内容

1. **依存関係の追加**
```toml
# src-tauri/Cargo.toml
[dependencies]
keyring = "2.0"
```

2. **KeychainServiceの実装**
```rust
// src-tauri/src/infrastructure/keychain.rs (新規作成)

use keyring::{Entry, Error as KeyringError};
use anyhow::{Context, Result};

const SERVICE_NAME: &str = "io.github.qasery";
const TOKEN_KEY: &str = "qase_api_token";

pub struct KeychainService;

impl KeychainService {
    /// APIトークンを保存
    pub fn save_token(token: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)
            .context("Failed to create keychain entry")?;
        
        entry.set_password(token)
            .context("Failed to save token to keychain")?;
        
        Ok(())
    }
    
    /// APIトークンを取得
    pub fn get_token() -> Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)
            .context("Failed to create keychain entry")?;
        
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(KeyringError::NoEntry) => Ok(None),
            Err(e) => Err(e).context("Failed to get token from keychain"),
        }
    }
    
    /// APIトークンを削除
    pub fn delete_token() -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)
            .context("Failed to create keychain entry")?;
        
        entry.delete_password()
            .context("Failed to delete token from keychain")?;
        
        Ok(())
    }
}
```

3. **AppStateの更新**
```rust
// src-tauri/src/application/appstate.rs

use crate::infrastructure::keychain::KeychainService;

pub struct AppState {
    pub db: Arc<Database>,
    // api_token: Arc<Mutex<Option<String>>> を削除
    // ...
}

impl AppState {
    pub fn set_api_token(&self, token: String) -> Result<()> {
        KeychainService::save_token(&token)
    }
    
    pub fn get_api_token(&self) -> Result<Option<String>> {
        KeychainService::get_token()
    }
}
```

#### 成果物
- [ ] Keychainサービスの実装
- [ ] AppStateの更新
- [ ] クロスプラットフォーム対応の確認（macOS/Windows/Linux）
- [ ] セキュリティテスト

#### 完了条件
- APIトークンがメモリに平文保存されない
- アプリ再起動後もトークンが永続化
- 3大OSすべてで動作確認

---

### タスク6: Qase API Mockの実装

**優先度**: ★★★★★  
**見積もり**: 2日  
**担当**: テスト・バックエンド

#### 目的
CI/CD環境での自動テスト実行を可能にし、テスト実行速度を向上。

#### 実装内容

1. **依存関係の追加**
```toml
# src-tauri/Cargo.toml
[dev-dependencies]
mockito = "1.2"
tokio-test = "0.4"
```

2. **Mock Serverを使ったテスト**
```rust
// src-tauri/src/infrastructure/qase_api.rs

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_get_projects_with_mock() {
        // Mockサーバーのセットアップ
        let mock_response = r#"{
            "status": true,
            "result": {
                "total": 2,
                "filtered": 2,
                "count": 2,
                "entities": [
                    {
                        "code": "DEMO",
                        "title": "Demo Project"
                    },
                    {
                        "code": "TEST",
                        "title": "Test Project"
                    }
                ]
            }
        }"#;
        
        let _m = mock("GET", "/v1/project")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();
        
        // Clientの作成（mock server URLを使用）
        let client = QaseClient {
            api_token: "test_token".to_string(),
            base_url: server_url(), // mockitoのURL
        };
        
        // テスト実行
        let result = client.get_projects(None, None).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.result.total, 2);
        assert_eq!(response.result.entities.len(), 2);
        assert_eq!(response.result.entities[0].code, "DEMO");
    }
    
    #[tokio::test]
    async fn test_get_projects_api_error() {
        // エラーケースのテスト
        let _m = mock("GET", "/v1/project")
            .with_status(401)
            .with_body(r#"{"status": false, "errorMessage": "Invalid API token"}"#)
            .create();
        
        let client = QaseClient {
            api_token: "invalid_token".to_string(),
            base_url: server_url(),
        };
        
        let result = client.get_projects(None, None).await;
        assert!(result.is_err());
    }
}
```

3. **既存テストの書き換え**
   - `test_get_projects()` → Mock使用
   - `test_get_suites()` → Mock使用
   - すべてのAPI呼び出しテストをMock化

#### 成果物
- [ ] すべてのQase APIメソッドのMockテスト
- [ ] 正常系・異常系のテストカバレッジ
- [ ] CI/CD設定（GitHub Actions等）

#### 完了条件
- 実APIキーなしで`cargo test`が完全に動作
- テスト実行時間が30秒以内
- すべてのAPIエンドポイントがMock化

---

### タスク7: 統合テストの追加

**優先度**: ★★★★☆  
**見積もり**: 2日  
**担当**: テスト全般

#### 目的
各層を組み合わせた動作を検証し、リグレッションを防止。

#### 実装内容

1. **統合テストディレクトリの作成**
```
src-tauri/
  tests/
    integration_test.rs  # 統合テスト
    common/
      mod.rs             # テストヘルパー
```

2. **テストヘルパーの実装**
```rust
// src-tauri/tests/common/mod.rs

use qasery::infrastructure::db::Database;
use qasery::application::appstate::AppState;
use std::sync::Arc;

pub fn setup_test_app() -> AppState {
    // インメモリDBでAppStateを初期化
    AppState::new(":memory:").expect("Failed to create test AppState")
}

pub fn create_test_project() -> qasery::domain::models::Project {
    // テストデータのファクトリ
    // ...
}
```

3. **統合テストの実装**
```rust
// src-tauri/tests/integration_test.rs

mod common;

use common::{setup_test_app, create_test_project};
use qasery::domain::repository::ProjectRepository;

#[test]
fn test_project_lifecycle() {
    // Given: AppStateを初期化
    let app = setup_test_app();
    let project = create_test_project();
    
    // When: プロジェクトを保存
    app.db.save(&project).unwrap();
    
    // Then: 保存されたプロジェクトを取得できる
    let retrieved = app.db.get_by_code(&project.code).unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, project.title);
}

#[tokio::test]
async fn test_sync_projects_integration() {
    // Presentation → Application → Infrastructure の統合テスト
    // ...
}
```

#### 成果物
- [ ] 主要ユースケースの統合テスト（10シナリオ以上）
- [ ] Given-When-Thenパターンの徹底
- [ ] テストヘルパー・ファクトリの整備

#### 完了条件
- 統合テストカバレッジ70%以上
- すべての統合テストがパス
- CI/CDパイプラインに組み込み

---

## ⚠️ Phase 2: High Priority Tasks（Phase 2開発中）

### タスク8: 非同期処理の統一（DB操作のAsync化）

**優先度**: ★★★★☆  
**見積もり**: 3日  
**担当**: バックエンド・パフォーマンス

#### 目的
Mutexボトルネックを解消し、並行処理性能を向上。

#### 実装内容

1. **sqlxへの移行**
```toml
# src-tauri/Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
# rusqliteを削除
```

2. **非同期Repositoryの実装**
```rust
// src-tauri/src/infrastructure/db.rs

use sqlx::SqlitePool;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        // マイグレーション
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl ProjectRepository for Database {
    async fn get_all(&self) -> Result<Vec<Project>> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE is_deleted = 0"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

3. **すべてのRepository/Serviceをasync化**

#### 成果物
- [ ] sqlxへの完全移行
- [ ] コネクションプールの導入
- [ ] パフォーマンスベンチマーク（1000件挿入で比較）

#### 完了条件
- Mutexが完全に削除
- 並行テストで性能向上を確認
- すべてのテストがパス

---

### タスク9: 型安全性の向上（Rust↔TypeScript）

**優先度**: ★★★★☆  
**見積もり**: 2日  
**担当**: フロントエンド・バックエンド連携

#### 実装内容

1. **tauri-spectaの導入**
```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-specta = "2.0"
specta = "2.0"
```

2. **型定義の自動生成**
```rust
// src-tauri/src/main.rs

use tauri_specta::collect_commands;
use specta::Type;

#[derive(Type, Serialize, Deserialize)]
pub struct Project {
    // ...
}

fn main() {
    let builder = tauri_specta::Builder::new()
        .commands(collect_commands![
            list_projects,
            get_project,
            // ...
        ]);
    
    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../frontend/src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");
}
```

3. **フロントエンドでの使用**
```typescript
// frontend/src/services/tauri.ts
import { commands } from './bindings';

export async function listProjects(): Promise<Project[]> {
  return await commands.listProjects();
}
```

#### 成果物
- [ ] TypeScript型定義の自動生成
- [ ] ビルド時の型チェック強化

---

### タスク10: パフォーマンス改善

**優先度**: ★★★☆☆  
**見積もり**: 2日

#### 実装内容

1. **ツリー構築アルゴリズムの最適化**（O(n²) → O(n)）
2. **DBインデックスの追加**
3. **ページネーションの導入**

---

### タスク11: フロントエンドテストの導入

**優先度**: ★★★★☆  
**見積もり**: 3日

#### 実装内容

1. **Vitest + React Testing Libraryのセットアップ**
2. **主要コンポーネントの単体テスト**
3. **カスタムフックのテスト**

---

## スケジュール

| Week | タスク | 工数 |
|------|--------|------|
| Week 1 | タスク1（Value Object）+ タスク2（Aggregate） | 5日 |
| Week 2 | タスク3（依存関係）+ タスク4（エラー処理）+ タスク5（Keychain） | 4日 |
| Week 3 | タスク6（Mock）+ タスク7（統合テスト） | 4日 |
| **バッファ** | 予備日・コードレビュー | 2日 |

**合計**: 15営業日（3週間）

---

## マイルストーン

### M1: Value Object & Aggregate完了（Week 1終了時）
- [ ] すべてのValue Objectが実装され、単体テストがパス
- [ ] Aggregate境界が明確化され、トランザクション整合性を保証
- [ ] テストカバレッジ85%以上

### M2: アーキテクチャ改善完了（Week 2終了時）
- [ ] 層の依存関係が適切
- [ ] エラーハンドリングが構造化
- [ ] APIトークンが安全に保存

### M3: テスト基盤完成（Week 3終了時）
- [ ] Qase API Mockが実装され、CI/CDが動作
- [ ] 統合テストが整備
- [ ] すべてのCriticalタスクが完了

---

## リスク管理

| リスク | 影響度 | 対策 |
|--------|--------|------|
| sqlx移行時のバグ | 高 | Week 3で実施、十分なテスト時間確保 |
| Keychainのプラットフォーム依存 | 中 | 早期に3大OSで動作確認 |
| Value Object導入の影響範囲 | 高 | 段階的移行、型エイリアスの活用 |
| テスト作成の工数超過 | 中 | 優先度の高いテストから着手 |

---

## 成功基準

Phase 2機能開発前に以下をすべて達成：

1. ✅ Value Objectで型安全性を保証
2. ✅ Aggregate境界でトランザクション整合性を保証
3. ✅ 層の依存関係が適切
4. ✅ エラーが構造化され、デバッグが容易
5. ✅ APIトークンが安全に保存
6. ✅ CI/CDで自動テストが動作
7. ✅ 統合テストで主要シナリオをカバー

**すべてのCriticalタスクが完了するまで、Phase 2の機能開発には着手しない。**

---

## 次のステップ

1. このアクションプランをチームでレビュー
2. タスクをGitHub Issuesまたはプロジェクト管理ツールに登録
3. 各タスクに担当者をアサイン
4. Week 1からタスク1（Value Object）に着手
5. 週次でマイルストーン達成状況をレビュー

---

**最終更新**: 2025-10-11  
**作成者**: Code Review AI Assistant
