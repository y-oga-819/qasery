# Task 4: CQRSパターンの導入

## 背景と課題

### 当初の問題

Phase 1で設計した新DDDモデル（カプセル化されたドメインモデル）と、旧models.rs（フラットなデータ構造）の統合をどう進めるか検討していた。

**問題点:**
- 新DDDモデル: privateフィールド + ビジネスロジック（更新に最適）
- 旧models.rs: publicフィールド（画面表示に最適）
- 両者を無理に統一しようとすると複雑な変換ロジックが必要

### 解決策: CQRSパターン

**CQRS (Command Query Responsibility Segregation)** を導入することで、更新系と参照系の責務を分離する。

```
更新系（Command側）: 新DDDモデルを使用
参照系（Query側）  : 旧models.rs（ReadModel）を使用
```

この分離により：
- ✅ それぞれの用途に最適化されたモデルを使える
- ✅ 無理な統合や複雑な変換が不要
- ✅ パフォーマンスの最適化が容易

## CQRSアーキテクチャ

### 全体像

```
┌─────────────────────────────────────────────────────────────┐
│                     Presentation層                          │
│                   (src/presentation/)                       │
│                                                             │
│  ┌──────────────────┐        ┌──────────────────┐         │
│  │ Command Handlers │        │ Query Handlers   │         │
│  │ - create_project │        │ - list_projects  │         │
│  │ - update_project │        │ - get_project    │         │
│  └────────┬─────────┘        └────────┬─────────┘         │
└───────────┼──────────────────────────┼───────────────────┘
            │                          │
            ▼                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application層                            │
│                  (src/application/)                         │
│                                                             │
│  ┌──────────────────┐        ┌──────────────────┐         │
│  │ CommandService   │        │ QueryService     │         │
│  │ - ビジネスロジック│        │ - データ取得のみ  │         │
│  │ - バリデーション  │        │ - 変換なし       │         │
│  └────────┬─────────┘        └────────┬─────────┘         │
└───────────┼──────────────────────────┼───────────────────┘
            │                          │
            ▼                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      Domain層                               │
│                    (src/domain/)                            │
│                                                             │
│  ┌──────────────────┐        ┌──────────────────┐         │
│  │ Aggregates       │        │ ReadModels       │         │
│  │ - Project        │        │ - ProjectRM      │         │
│  │ - TestCase       │        │ - TestCaseRM     │         │
│  │ - (新DDDモデル)   │        │ - (旧models.rs)  │         │
│  └────────┬─────────┘        └────────┬─────────┘         │
│           │                           │                    │
│  ┌────────┴─────────┐        ┌────────┴─────────┐         │
│  │ CommandRepo      │        │ QueryRepo        │         │
│  │ (trait)          │        │ (trait)          │         │
│  └────────┬─────────┘        └────────┬─────────┘         │
└───────────┼──────────────────────────┼───────────────────┘
            │                          │
            ▼                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  Infrastructure層                           │
│                (src/infrastructure/)                        │
│                                                             │
│  ┌──────────────────┐        ┌──────────────────┐         │
│  │ Command Repo     │        │ Query Repo       │         │
│  │ Impl             │        │ Impl             │         │
│  │ - 集約→DTO変換   │        │ - 直接SELECT     │         │
│  │ - トランザクション│        │ - JOINも可能     │         │
│  └────────┬─────────┘        └────────┬─────────┘         │
└───────────┼──────────────────────────┼───────────────────┘
            │                          │
            └──────────┬───────────────┘
                       ▼
            ┌────────────────────┐
            │     Database       │
            │     (SQLite)       │
            └────────────────────┘
```

## ディレクトリ構造

### 提案する構造

```
src/
├── domain/
│   ├── commands/              # Command側（更新系）
│   │   ├── mod.rs
│   │   ├── project/           # Project集約
│   │   │   ├── mod.rs         # Project構造体
│   │   │   ├── project_code.rs
│   │   │   └── repository.rs  # ProjectCommandRepository trait
│   │   ├── test_case/         # TestCase集約
│   │   ├── test_suite/        # TestSuite集約
│   │   └── shared_step/       # SharedStep集約
│   │
│   ├── queries/               # Query側（参照系）
│   │   ├── mod.rs
│   │   ├── read_models.rs     # ReadModel定義（旧models.rs）
│   │   └── repository.rs      # Query用Repository trait
│   │
│   └── common/                # 共通（SyncStatusなど）
│       └── sync_metadata.rs
│
├── application/
│   ├── commands/              # Command Service
│   │   ├── mod.rs
│   │   ├── project_command_service.rs
│   │   ├── test_case_command_service.rs
│   │   ├── test_suite_command_service.rs
│   │   └── shared_step_command_service.rs
│   │
│   └── queries/               # Query Service
│       ├── mod.rs
│       ├── project_query_service.rs
│       ├── test_case_query_service.rs
│       ├── test_suite_query_service.rs
│       └── shared_step_query_service.rs
│
├── infrastructure/
│   ├── db/
│   │   ├── commands/          # Command Repo実装
│   │   │   ├── mod.rs
│   │   │   ├── project.rs
│   │   │   ├── test_case.rs
│   │   │   ├── test_suite.rs
│   │   │   └── shared_step.rs
│   │   │
│   │   ├── queries/           # Query Repo実装
│   │   │   ├── mod.rs
│   │   │   ├── project.rs
│   │   │   ├── test_case.rs
│   │   │   ├── test_suite.rs
│   │   │   └── shared_step.rs
│   │   │
│   │   └── dto.rs             # DB↔集約の変換用DTO
│   │
│   └── db.rs                  # Database構造体
│
└── presentation/
    └── commands.rs            # Tauriコマンド
```

## 実装ステップ

### Phase 1: Query側の実装（このPR）

Query側（参照系）を先に実装し、既存機能を維持しながらCQRSに移行。

#### Step 1: ReadModelの作成

旧models.rsをQuery側のReadModelとして再定義：

```bash
# 旧models.rsの内容をコピーして新しい場所に配置
```

**ファイル:** `src/domain/queries/read_models.rs`

```rust
// 参照専用のReadModel
// 画面表示に最適化されたフラットな構造

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// プロジェクトReadModel（参照専用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectReadModel {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub local_updated_at: DateTime<Utc>,
    pub remote_updated_at: Option<DateTime<Utc>>,
    pub sync_status: String,  // "clean", "local_modified", etc.
    pub is_deleted: bool,
}

// TestSuiteReadModel, TestCaseReadModel, SharedStepReadModel も同様
```

#### Step 2: Query用Repositoryトレイトの定義

**ファイル:** `src/domain/queries/repository.rs`

```rust
use super::read_models::*;
use anyhow::Result;

/// プロジェクト参照用Repository
pub trait ProjectQueryRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<ProjectReadModel>>;
    fn get_by_code(&self, code: &str) -> Result<Option<ProjectReadModel>>;
}

/// テストスイート参照用Repository
pub trait TestSuiteQueryRepository: Send + Sync {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuiteReadModel>>;
    fn get_by_id(&self, id: i64) -> Result<Option<TestSuiteReadModel>>;
}

/// テストケース参照用Repository
pub trait TestCaseQueryRepository: Send + Sync {
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCaseReadModel>>;
    fn get_by_id(&self, id: i64) -> Result<Option<TestCaseReadModel>>;
}

/// Shared Step参照用Repository
pub trait SharedStepQueryRepository: Send + Sync {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStepReadModel>>;
    fn get_by_hash(&self, hash: &str) -> Result<Option<SharedStepReadModel>>;
}
```

#### Step 3: Infrastructure層でQuery Repositoryを実装

**ファイル:** `src/infrastructure/db/queries/project.rs`

```rust
use crate::domain::queries::{read_models::ProjectReadModel, repository::ProjectQueryRepository};
use crate::infrastructure::db::Database;
use anyhow::Result;

impl ProjectQueryRepository for Database {
    fn get_all(&self) -> Result<Vec<ProjectReadModel>> {
        let conn = self.conn.lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT code, title, description, local_updated_at,
                    remote_updated_at, sync_status, is_deleted
             FROM projects
             WHERE is_deleted = 0"
        )?;

        let projects = stmt.query_map([], |row| {
            Ok(ProjectReadModel {
                code: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                local_updated_at: row.get::<_, String>(3)?.parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                remote_updated_at: row.get::<_, Option<String>>(4)?
                    .and_then(|s| s.parse().ok()),
                sync_status: row.get(5)?,
                is_deleted: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    fn get_by_code(&self, code: &str) -> Result<Option<ProjectReadModel>> {
        // 同様の実装
        // ...
    }
}
```

#### Step 4: Application層にQueryServiceを追加

**ファイル:** `src/application/queries/project_query_service.rs`

```rust
use crate::domain::queries::{read_models::ProjectReadModel, repository::ProjectQueryRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ProjectQueryService {
    repo: Arc<dyn ProjectQueryRepository>,
}

impl ProjectQueryService {
    pub fn new(repo: Arc<dyn ProjectQueryRepository>) -> Self {
        Self { repo }
    }

    /// 全プロジェクトを取得
    pub fn get_all_projects(&self) -> Result<Vec<ProjectReadModel>> {
        self.repo.get_all()
    }

    /// プロジェクトコードでプロジェクトを取得
    pub fn get_project(&self, code: &str) -> Result<Option<ProjectReadModel>> {
        self.repo.get_by_code(code)
    }
}
```

#### Step 5: Presentation層を対応

**ファイル:** `src/presentation/commands.rs`

```rust
use crate::application::queries::ProjectQueryService;
use crate::domain::queries::read_models::ProjectReadModel;
use tauri::State;

#[tauri::command]
pub fn list_projects(
    query_service: State<ProjectQueryService>,
) -> Result<Vec<ProjectReadModel>, String> {
    query_service.get_all_projects()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(
    query_service: State<ProjectQueryService>,
    code: String,
) -> Result<Option<ProjectReadModel>, String> {
    query_service.get_project(&code)
        .map_err(|e| e.to_string())
}
```

#### Step 6: main.rsでDI設定

```rust
use crate::infrastructure::db::Database;
use crate::application::queries::ProjectQueryService;
use std::sync::Arc;

fn main() {
    let db = Arc::new(Database::new("qasery.db").unwrap());

    // Query Service
    let project_query_service = ProjectQueryService::new(db.clone());

    tauri::Builder::default()
        .manage(project_query_service)
        .invoke_handler(tauri::generate_handler![
            list_projects,
            get_project,
            // ...
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Step 7: テスト

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
```

#### Step 8: 旧models.rsを削除

Query側の実装が完了し、全テストが通ったら：

```bash
rm src-tauri/src/domain/models.rs
# domain/mod.rsからもmodels参照を削除
```

### Phase 2: Command側の実装（次のPR以降）

#### ProjectCommandRepositoryの実装

**ファイル:** `src/domain/commands/project/repository.rs`

```rust
use super::Project;
use crate::domain::commands::project::ProjectCode;
use anyhow::Result;

pub trait ProjectCommandRepository: Send + Sync {
    /// プロジェクトを保存（作成または更新）
    fn save(&self, project: &Project) -> Result<()>;

    /// プロジェクトを削除
    fn delete(&self, code: &ProjectCode) -> Result<()>;

    /// 更新用にプロジェクトを取得（ビジネスロジック実行用）
    fn get_for_update(&self, code: &ProjectCode) -> Result<Option<Project>>;
}
```

#### ProjectCommandServiceの実装

```rust
pub struct ProjectCommandService {
    repo: Arc<dyn ProjectCommandRepository>,
}

impl ProjectCommandService {
    pub fn create_project(&self, code: String, title: String, description: Option<String>) -> Result<()> {
        // バリデーション
        let code = ProjectCode::new(code)?;

        // ビジネスロジック
        let project = Project::new(code, title, description);

        // 永続化
        self.repo.save(&project)?;

        Ok(())
    }

    pub fn update_project_title(&self, code: String, new_title: String) -> Result<()> {
        let code = ProjectCode::new(code)?;

        // 集約を取得
        let mut project = self.repo.get_for_update(&code)?
            .ok_or_else(|| anyhow!("Project not found"))?;

        // ビジネスロジック実行
        project.update_title(new_title);

        // 永続化
        self.repo.save(&project)?;

        Ok(())
    }
}
```

## メリット

### 1. 責務の明確化

- **Query側**: データ取得のみ、ビジネスロジックなし
- **Command側**: ビジネスルール保護、整合性担保

### 2. パフォーマンス最適化

- **Query側**:
  - 複雑なJOINも自由に
  - インデックス最適化
  - キャッシュ戦略も適用しやすい
  - 変換オーバーヘッドなし

- **Command側**:
  - 集約単位でロード（必要最小限）
  - トランザクション境界が明確

### 3. スケーラビリティ

- Query用DBとCommand用DBを物理的に分離可能（将来）
- ReadModelを複数種類用意可能（画面ごとに最適化）

### 4. テストのしやすさ

- Command側: ビジネスロジックの単体テストが容易
- Query側: シンプルなデータ取得のテスト

### 5. 段階的移行

- ✅ まずQuery側を実装（影響小）
- ✅ Command側は徐々に追加
- ✅ 既存機能を壊さない

## デメリットと対策

### デメリット1: コード量の増加

**対策:**
- Repositoryの数は増えるが、各Repositoryはシンプル
- コード生成ツールの活用（将来）

### デメリット2: 一時的な不整合

CQRSでは、更新後すぐにReadModelに反映されない可能性がある（EventualConsistency）。

**対策:**
- 今回は同一DBを使うため、即座に反映される
- 将来的にイベント駆動にする場合は、UIで考慮

## 完了条件

### Phase 1（このPR）

- [ ] Query側の設計ドキュメント作成
- [ ] ReadModelの定義（domain/queries/read_models.rs）
- [ ] QueryRepository traitの定義
- [ ] Infrastructure層でQueryRepository実装
- [ ] QueryServiceの実装
- [ ] Presentation層の対応
- [ ] 旧models.rs削除
- [ ] 全テスト通過
- [ ] CI通過

### Phase 2以降（次のPR）

- [ ] CommandRepository traitの定義
- [ ] CommandService実装
- [ ] DTO変換ロジック
- [ ] Command側のテスト

## 参考資料

- Martin Fowler『CQRS』: https://martinfowler.com/bliki/CQRS.html
- Greg Young『CQRS Documents』
- Eric Evans『Domain-Driven Design』第5章
- Vaughn Vernon『Implementing Domain-Driven Design』第4章
