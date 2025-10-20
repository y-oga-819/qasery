# Task 4: 旧models.rsから新DDDモデルへの完全移行

## 目的

現在のinfrastructure層で使用されている旧models.rs（フラットなデータ構造）を、Phase 1で設計した新DDDモデル（カプセル化されたドメインモデル）に完全移行する。

## 現状分析

### 旧models.rsの構造

```rust
// src/domain/models.rs
pub struct Project {
    pub code: String,                      // publicフィールド
    pub title: String,
    pub description: Option<String>,
    pub local_updated_at: DateTime<Utc>,
    pub remote_updated_at: Option<DateTime<Utc>>,
    pub sync_status: SyncStatus,
    pub is_deleted: bool,
}
```

**特徴:**
- 全フィールドがpublic
- ビジネスロジックなし（データ保持のみ）
- データベースの行と1:1でマッピング可能
- DTOとして機能

### 新DDDモデルの構造

```rust
// src/domain/project/mod.rs
pub struct Project {
    code: ProjectCode,                      // privateフィールド + 値オブジェクト
    title: String,
    description: Option<String>,
    sync_metadata: SyncMetadata,            // メタデータをカプセル化
}

impl Project {
    pub fn new(...) -> Self { ... }
    pub fn update_title(&mut self, ...) { ... }  // ビジネスロジック
    pub fn mark_for_deletion(&mut self) { ... }
}
```

**特徴:**
- フィールドがprivate（カプセル化）
- 値オブジェクト使用（ProjectCode、SyncMetadata）
- ビジネスロジックを持つ
- データベース直接マッピング不可

### 問題点

1. **構造の不一致**: 旧モデルはpublicフィールド、新モデルはprivate + メソッド
2. **型の違い**: `String` vs `ProjectCode`、`SyncStatus` vs `SyncMetadata`
3. **責務の違い**: 旧モデル=DTO、新モデル=ドメインモデル

## 解決アプローチ

### アプローチB: 完全移行（採用）

新DDDモデルを中心に据え、infrastructure層にDTOレイヤーを追加する。

```
┌─────────────────────────────────────┐
│ Presentation層                       │
│ (commands.rs)                       │
│ - 新DDDモデルを使用                  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ Application層                        │
│ (services.rs)                       │
│ - 新DDDモデルを使用                  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ Domain層                             │
│ - 新DDDモデル（project/, test_case/）│
│ - Repository trait                  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│ Infrastructure層                     │
│ ┌─────────────────────────────────┐ │
│ │ Repository実装 (db/*.rs)        │ │
│ │ - 新DDDモデルを受け取る         │ │
│ │ - DTOに変換してDBに保存         │ │
│ └──────────┬──────────────────────┘ │
│            │                         │
│ ┌──────────▼──────────────────────┐ │
│ │ DTO層 (dto.rs)                  │ │
│ │ - DBの行データを表現            │ │
│ │ - publicフィールド              │ │
│ │ - 新DDD ↔ DTO 変換ロジック     │ │
│ └──────────┬──────────────────────┘ │
└────────────┼────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ Database (SQLite)                   │
└─────────────────────────────────────┘
```

## 実装ステップ

### Step 1: DTO層の作成

**ファイル:** `src/infrastructure/dto.rs`

旧models.rsの内容をdto.rsに移動し、役割を明確化：

```rust
// src/infrastructure/dto.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// データベース行データを表現するDTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub local_updated_at: DateTime<Utc>,
    pub remote_updated_at: Option<DateTime<Utc>>,
    pub sync_status: String,  // "clean", "local_modified", etc.
    pub is_deleted: bool,
}

// TestSuiteDto, TestCaseDto, SharedStepDto も同様に定義
```

### Step 2: 変換ロジックの実装

**ファイル:** `src/infrastructure/dto.rs`

新DDDモデル ↔ DTO の相互変換を実装：

```rust
impl ProjectDto {
    /// 新DDDモデルからDTOに変換
    pub fn from_domain(project: &crate::domain::project::Project) -> Self {
        Self {
            code: project.code().value().to_string(),
            title: project.title().to_string(),
            description: project.description().map(|s| s.to_string()),
            local_updated_at: project.sync_metadata().local_updated_at(),
            remote_updated_at: project.sync_metadata().remote_updated_at(),
            sync_status: project.sync_metadata().sync_status().to_string(),
            is_deleted: project.sync_metadata().is_deleted(),
        }
    }

    /// DTOから新DDDモデルに変換
    pub fn to_domain(&self) -> Result<crate::domain::project::Project> {
        let code = ProjectCode::new(self.code.clone())?;
        // SyncMetadataも復元
        // ...
        Ok(project)
    }
}
```

### Step 3: Repository実装の書き直し

**対象ファイル:**
- `src/infrastructure/db/project.rs`
- `src/infrastructure/db/test_suite.rs`
- `src/infrastructure/db/test_case.rs`
- `src/infrastructure/db/shared_step.rs`

**変更内容:**

```rust
// Before (旧models.rs使用)
use crate::domain::models::Project;

impl ProjectRepository for Database {
    fn get_all(&self) -> Result<Vec<Project>> {
        // 直接Projectを構築
        let project = Project {
            code: row.get(0)?,
            title: row.get(1)?,
            // ...
        };
    }
}

// After (新DDDモデル + DTO使用)
use crate::domain::project::Project;
use crate::infrastructure::dto::ProjectDto;

impl ProjectRepository for Database {
    fn get_all(&self) -> Result<Vec<Project>> {
        // まずDTOを構築
        let dto = ProjectDto {
            code: row.get(0)?,
            title: row.get(1)?,
            // ...
        };
        // DTOから新DDDモデルに変換
        let project = dto.to_domain()?;
        Ok(project)
    }
}
```

### Step 4: Application層の対応

**ファイル:** `src/application/services.rs`

既に新DDDモデルのRepositoryを使用しているため、大きな変更は不要。
ただし、返り値の型が変わる可能性があるため確認が必要。

### Step 5: Presentation層の対応

**ファイル:** `src/presentation/commands.rs`

Tauriコマンドの引数・返り値を新DDDモデルに対応：

```rust
// Before
use crate::domain::models::Project;

#[tauri::command]
pub fn get_projects() -> Vec<Project> { ... }

// After
use crate::domain::project::Project;

#[tauri::command]
pub fn get_projects() -> Vec<ProjectResponse> { ... }
// ProjectResponseは新DDDモデルをシリアライズ可能にしたDTO
```

### Step 6: 旧models.rsの削除

全ての移行が完了したら、旧models.rsを削除：

```bash
rm src-tauri/src/domain/models.rs
```

domain/mod.rsからもmodels参照を削除。

### Step 7: テスト

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
```

## 期待される成果

### コード品質

- ✅ DDDの原則に完全準拠
- ✅ ドメインモデルのカプセル化が保たれる
- ✅ Infrastructure層とDomain層の責務が明確に分離
- ✅ 値オブジェクトによる型安全性向上

### 保守性

- ✅ ビジネスロジックがドメイン層に集約
- ✅ データベーススキーマ変更の影響がDTO層に限定
- ✅ テストが書きやすい（ドメインロジックの単体テスト）

### パフォーマンス

- ⚠️ DTO変換のオーバーヘッド（微小、無視可能レベル）

## リスクと対策

### リスク1: 変換ロジックのバグ

**対策:**
- DTO変換の単体テストを充実させる
- 既存のintegration testで動作確認

### リスク2: 大規模な変更によるデグレード

**対策:**
- 段階的な実装（Project → TestSuite → TestCase → SharedStep）
- 各ステップでテスト実行
- コミットを細かく刻む（TDD原則）

### リスク3: フロントエンドとの互換性

**対策:**
- Presentation層でシリアライズ可能なDTOを用意
- 既存のAPIシグネチャを維持

## 完了条件

- [ ] DTO層の実装完了
- [ ] 全Repository実装が新DDDモデル対応
- [ ] Application/Presentation層が新DDDモデル使用
- [ ] 旧models.rs削除
- [ ] 全65テストが通過
- [ ] CI通過（Format, Lint, Test）

## 参考資料

- Phase 1 DDD設計: `docs/specification.md`
- Infrastructure層リファクタリング: `docs/phase2/task3-infrastructure-refactoring.md`
- Eric Evans『Domain-Driven Design』
