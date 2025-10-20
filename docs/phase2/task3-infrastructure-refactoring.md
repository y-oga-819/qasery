# Infrastructure層リファクタリング計画書

## 概要

現在、`infrastructure/db.rs`に全てのRepository実装が集約されており（1596行）、保守性とテスト容易性に課題がある。
DDD設計に従い、各集約ごとにRepository実装を分離し、責任を明確化する。

---

## 現状分析

### 現在のファイル構成

```
src-tauri/src/infrastructure/
├── mod.rs
├── db.rs                    # 1596行 - すべてのRepository実装
└── qase_api.rs
```

### db.rsの構造

```rust
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> { ... }
    pub fn initialize(&self) -> Result<(), rusqlite::Error> { ... }
}

// 204行目から
impl ProjectRepository for Database { ... }      // ~140行

// 347行目から
impl TestSuiteRepository for Database { ... }    // ~180行

// 533行目から
impl TestCaseRepository for Database { ... }     // ~270行

// 808行目から
impl SharedStepRepository for Database { ... }   // ~150行

// テストコード（#[cfg(test)]）が残り
```

### 問題点

1. **単一責任の原則違反**
   - 1ファイルに4つの集約のDB操作が混在
   - 変更時の影響範囲が不明瞭

2. **保守性の低下**
   - ファイルが長すぎて見通しが悪い（1596行）
   - 特定の集約の実装を見つけにくい

3. **テストの困難さ**
   - 各Repository実装を独立してテストしにくい
   - テストコードも同じファイルに混在

4. **DDD設計との不一致**
   - Domain層は集約単位で分離されているのに、Infrastructure層は集約されている
   - レイヤー間で一貫性がない

---

## 目標設計（リファクタリング後）

### 新しいディレクトリ構造

```
src-tauri/src/infrastructure/
├── mod.rs                          # モジュール定義
├── db.rs                           # Database構造体とコネクション管理のみ（~200行）
├── db/                             # Database用のRepository実装群
│   ├── mod.rs                      # サブモジュール定義
│   ├── project.rs                  # impl ProjectRepository for Database
│   ├── test_suite.rs               # impl TestSuiteRepository for Database
│   ├── test_case.rs                # impl TestCaseRepository for Database
│   └── shared_step.rs              # impl SharedStepRepository for Database
└── qase_api.rs                     # Qase API クライアント（変更なし）
```

### 各ファイルの責務

#### `infrastructure/db.rs`
- Database構造体の定義
- コネクション管理（new, initialize）
- 共通のヘルパー関数（あれば）
- **行数**: 約200行（現在の1596行から大幅削減）

#### `infrastructure/db/mod.rs`
```rust
// 各Repository実装をre-export
mod project;
mod test_suite;
mod test_case;
mod shared_step;

// 必要に応じてテストヘルパーも
#[cfg(test)]
mod test_helpers;
```

#### `infrastructure/db/project.rs`
```rust
use crate::domain::project::{Project, ProjectRepository};
use crate::infrastructure::db::Database;
use rusqlite::Result;

impl ProjectRepository for Database {
    fn get_all(&self) -> Result<Vec<Project>> { ... }
    fn get_by_code(&self, code: &str) -> Result<Option<Project>> { ... }
    fn save(&self, project: &Project) -> Result<()> { ... }
    fn delete(&self, code: &str) -> Result<()> { ... }
}

#[cfg(test)]
mod tests {
    // Projectリポジトリのテストのみ
}
```

**予想行数**: 約150行

#### `infrastructure/db/test_suite.rs`
```rust
use crate::domain::test_suite::{TestSuite, TestSuiteRepository};
use crate::infrastructure::db::Database;
use rusqlite::Result;

impl TestSuiteRepository for Database {
    fn get_by_id(&self, id: i64) -> Result<Option<TestSuite>> { ... }
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuite>> { ... }
    fn save(&self, suite: &TestSuite) -> Result<()> { ... }
}

#[cfg(test)]
mod tests {
    // TestSuiteリポジトリのテストのみ
}
```

**予想行数**: 約200行

#### `infrastructure/db/test_case.rs`
```rust
use crate::domain::test_case::{TestCase, TestCaseRepository};
use crate::infrastructure::db::Database;
use rusqlite::Result;

impl TestCaseRepository for Database {
    fn get_by_id(&self, id: i64) -> Result<Option<TestCase>> { ... }
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCase>> { ... }
    fn save(&self, test_case: &TestCase) -> Result<()> { ... }
    fn get_steps(&self, case_id: i64) -> Result<Vec<TestStep>> { ... }
    fn save_steps(&self, case_id: i64, steps: &[TestStep]) -> Result<()> { ... }
}

#[cfg(test)]
mod tests {
    // TestCaseリポジトリのテストのみ
}
```

**予想行数**: 約300行（ステップ管理含む）

#### `infrastructure/db/shared_step.rs`
```rust
use crate::domain::shared_step::{SharedStep, SharedStepRepository};
use crate::infrastructure::db::Database;
use rusqlite::Result;

impl SharedStepRepository for Database {
    fn get_by_hash(&self, hash: &str) -> Result<Option<SharedStep>> { ... }
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStep>> { ... }
    fn save(&self, shared_step: &SharedStep) -> Result<()> { ... }
}

#[cfg(test)]
mod tests {
    // SharedStepリポジトリのテストのみ
}
```

**予想行数**: 約170行

---

## リファクタリング手順

### Phase 1: 準備（この設計書の作成）
- [x] 現状分析
- [x] 目標設計の定義
- [x] リファクタリング手順の策定

### Phase 2: ディレクトリ構造の作成
1. `src-tauri/src/infrastructure/db/` ディレクトリを作成
2. `src-tauri/src/infrastructure/db/mod.rs` を作成

### Phase 3: Repository実装の分離（集約ごと）

各集約について以下の手順を実施：

#### 3-1. ProjectRepository の分離
1. `infrastructure/db/project.rs` を新規作成
2. `db.rs` から `impl ProjectRepository for Database` ブロックをコピー
3. 必要なimport文を追加
4. テストコードも移動
5. `db/mod.rs` でモジュールを宣言
6. コンパイルエラーがないことを確認
7. テストが通ることを確認
8. **コミット**: `refactor: ProjectRepository実装をdb/project.rsに分離`

#### 3-2. TestSuiteRepository の分離
（同様の手順）
- **コミット**: `refactor: TestSuiteRepository実装をdb/test_suite.rsに分離`

#### 3-3. TestCaseRepository の分離
（同様の手順）
- **コミット**: `refactor: TestCaseRepository実装をdb/test_case.rsに分離`

#### 3-4. SharedStepRepository の分離
（同様の手順）
- **コミット**: `refactor: SharedStepRepository実装をdb/shared_step.rsに分離`

### Phase 4: db.rsのクリーンアップ
1. `db.rs` から移動した実装コードを削除
2. Database構造体とコネクション管理のみ残す
3. コンパイルエラーがないことを確認
4. 全テストが通ることを確認
5. **コミット**: `refactor: db.rsをDatabase構造体のみに整理`

### Phase 5: 新DDD モデルへのimport更新
1. 各`db/*.rs`ファイルで旧models.rsのimportを新DDDモデルに更新
   - `Project` → `crate::domain::project::Project`
   - `TestSuite` → `crate::domain::test_suite::TestSuite`
   - `TestCase` → `crate::domain::test_case::TestCase`
   - `SharedStep` → `crate::domain::shared_step::SharedStep`
   - `SyncStatus` → `crate::domain::common::sync_metadata::SyncStatus`
   - `TestCaseStep` → `crate::domain::test_case::TestStep`
   - `SharedStepStep` → `crate::domain::shared_step::SharedStepStep`
2. `application/services.rs` のimport更新
3. `presentation/commands.rs` のimport更新
4. コンパイルエラーを修正
5. 全テストが通ることを確認
6. **コミット**: `refactor: 旧models.rsから新DDDモデルへimport更新`

### Phase 6: 旧ファイルの削除
1. `domain/models.rs` を削除
2. `domain/repository.rs` を削除
3. `domain/mod.rs` から旧モジュールの宣言を削除
4. コンパイルエラーがないことを確認
5. 全テストが通ることを確認
6. **コミット**: `chore: 旧models.rsとrepository.rsを削除`

---

## リファクタリング後の利点

### 1. 保守性の向上
- ✅ 各集約のDB操作が独立したファイルに分離
- ✅ ファイルサイズが適切（各150-300行）
- ✅ 特定の集約の実装を見つけやすい

### 2. テスト容易性の向上
- ✅ 各Repository実装を独立してテスト可能
- ✅ テストコードも集約ごとに分離

### 3. DDD設計との一貫性
- ✅ Domain層とInfrastructure層の構造が一致
- ✅ レイヤー間の対応関係が明確

### 4. 拡張性の向上
- ✅ 新しいストレージバックエンド追加が容易（例: `memory/`, `postgres/`）
- ✅ 各集約の実装を独立して進化させられる

### 5. チーム開発の効率化
- ✅ ファイル単位でのコンフリクトが減少
- ✅ レビューが容易（差分が小さく、集約単位で明確）

---

## 注意事項

### コミット戦略
- 各Phase、各集約ごとに細かくコミット
- コミットメッセージはConventional Commits形式
- 各コミット後に必ずテストを実行

### テスト戦略
- 各Repository実装を移動後、必ずテストを実行
- テストが失敗した場合は即座に修正
- 全テストが通った状態でのみ次のステップへ

### import更新の注意点
- 新DDDモデルのフィールド名・型が旧モデルと異なる可能性
- 特に注意が必要な変更：
  - `TestCaseStep` → `TestStep`（名前変更）
  - `project_code: String` → `project_code: ProjectCode`（型変更）
  - `sync_status: SyncStatus` → `sync_metadata: SyncMetadata`（構造変更）

---

## チェックリスト

### Phase 1: 準備
- [x] 設計書の作成

### Phase 2: ディレクトリ構造
- [ ] `infrastructure/db/` ディレクトリ作成
- [ ] `infrastructure/db/mod.rs` 作成

### Phase 3: Repository実装の分離
- [ ] ProjectRepository分離（db/project.rs）
- [ ] TestSuiteRepository分離（db/test_suite.rs）
- [ ] TestCaseRepository分離（db/test_case.rs）
- [ ] SharedStepRepository分離（db/shared_step.rs）

### Phase 4: db.rsクリーンアップ
- [ ] 移動した実装を削除
- [ ] Database構造体のみ残す

### Phase 5: import更新
- [ ] db/*.rsのimport更新
- [ ] services.rsのimport更新
- [ ] commands.rsのimport更新

### Phase 6: 旧ファイル削除
- [ ] models.rs削除
- [ ] repository.rs削除

### 最終確認
- [ ] 全テストが通る（`cargo test`）
- [ ] Lintが通る（`cargo clippy`）
- [ ] フォーマットが正しい（`cargo fmt --check`）

---

**作成日**: 2025-10-19
**関連PR**: (リファクタリング実施後に記載)
