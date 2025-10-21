# Task 5 - Phase 1: Query側移行計画

## 目的

既存のUI層（Tauri Commands）を新しいCQRS設計に移行する。
Phase 1では**読み取り機能のみ**をReadModelベースに書き換え、既存機能を維持する。

## 現状分析

### 既存実装（Phase 1時点）
- `src/presentation/commands.rs`: Tauri Commands実装
- `src/application/services.rs`: Application Services実装
- **使用している型**: 古い`domain::models`の型（`Project`, `TestCase`など）

### 新しい設計（PR #25-#28で実装済み）
- Domain層: Aggregate単位で再設計（`domain::project::Project`など）
- Infrastructure層: Command/Query Repository分離
- **Query側**: ReadModelを使用（`ProjectReadModel`, `TestCaseReadModel`など）

## 移行方針

### Phase 1: Query側の移行（本フェーズ）
既存の**読み取り専用**コマンドを新しいQueryRepositoryとReadModelに書き換え。

#### 対象コマンド（読み取り専用）
1. `list_projects` → `ProjectQueryRepository::get_all()`
2. `get_project` → `ProjectQueryRepository::get_by_code()`
3. `get_suites_tree` → `TestSuiteQueryRepository::get_by_project()`
4. `get_suite` → `TestSuiteQueryRepository::get_by_id()`
5. `get_cases_by_suite` → `TestCaseQueryRepository::get_by_suite()`
6. `get_case` → `TestCaseQueryRepository::get_by_id()`
7. `get_case_steps` → `TestCaseQueryRepository::get_steps()`
8. `get_shared_steps` → `SharedStepQueryRepository::get_by_project()`
9. `get_shared_step` → `SharedStepQueryRepository::get_by_hash()`

#### 非対象コマンド（後のPhaseで対応）
- `sync_projects` ← Command側（書き込み）
- `sync_suites` ← Command側（書き込み）
- `sync_cases` ← Command側（書き込み）
- `sync_shared_steps` ← Command側（書き込み）
- `save_case` ← Command側（書き込み）
- `save_api_token` / `get_api_token` ← 後で対応

## 実装手順

### Step 1: Application Serviceの調整

#### Before（古い実装）
```rust
// src/application/services.rs
impl ProjectService {
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.repository.get_all()  // 古いRepository
    }
}
```

#### After（新しい実装）
```rust
// src/application/services.rs
use crate::domain::queries::{ProjectQueryRepository, ProjectReadModel};

impl ProjectService {
    pub fn list_projects(&self) -> Result<Vec<ProjectReadModel>> {
        let query_repo = &self.db as &dyn ProjectQueryRepository;
        query_repo.get_all()
    }
}
```

### Step 2: Commands.rsの書き換え

#### Before
```rust
#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state
        .project_service
        .list_projects()
        .map_err(|e| e.to_string())
}
```

#### After
```rust
use crate::domain::queries::{ProjectQueryRepository, ProjectReadModel};

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<ProjectReadModel>, String> {
    let query_repo = &state.db as &dyn ProjectQueryRepository;
    query_repo.get_all().map_err(|e| e.to_string())
}
```

**重要な変更点**:
- Application Serviceを経由せず、**直接QueryRepositoryを呼び出す**
- CQRSの原則: Query操作にビジネスロジックは不要
- 返り値の型が`Project`から`ProjectReadModel`に変更

### Step 3: 階層構造の対応（TestSuite）

`get_suites_tree`は階層構造を返す必要があるため、特別な対応が必要。

#### Option A: QueryRepositoryで階層構築
```rust
impl TestSuiteQueryRepository for Database {
    fn get_by_project_tree(&self, project_code: &str) -> Result<Vec<TestSuiteReadModel>> {
        // 1. 全スイートを取得
        let suites = self.get_by_project(project_code)?;

        // 2. 階層構造を構築
        build_tree(suites)
    }
}
```

#### Option B: Commands層で階層構築
```rust
#[tauri::command]
pub async fn get_suites_tree(
    state: State<'_, AppState>,
    project_code: String,
) -> Result<Vec<TestSuiteReadModel>, String> {
    let query_repo = &state.db as &dyn TestSuiteQueryRepository;
    let suites = query_repo.get_by_project(&project_code).map_err(|e| e.to_string())?;

    // Commands層で階層構造を構築
    Ok(build_tree(suites))
}
```

**採用**: Option B（Commands層で構築）
- 理由: QueryRepositoryはシンプルに保つ
- 階層構築は表示ロジックに近い

## 型の対応表

| 古い型（domain::models） | 新しい型（domain::queries） | 対応Repository |
|------------------------|---------------------------|---------------|
| `Project` | `ProjectReadModel` | `ProjectQueryRepository` |
| `TestSuite` | `TestSuiteReadModel` | `TestSuiteQueryRepository` |
| `TestCase` | `TestCaseReadModel` | `TestCaseQueryRepository` |
| `TestCaseStep` | `TestCaseStepReadModel` | `TestCaseQueryRepository` |
| `SharedStep` | `SharedStepReadModel` | `SharedStepQueryRepository` |
| `SharedStepStep` | `SharedStepStepReadModel` | `SharedStepQueryRepository` |

## フィールドの違い

### ProjectReadModel
新しい型にはないフィールド:
- （なし、ほぼ同じ）

### TestSuiteReadModel
新しい型の追加フィールド:
- `children: Option<Vec<TestSuiteReadModel>>` - 階層構造用

### TestCaseReadModel
新しい型のフィールド名変更:
- `type` → `test_type` (Rustの予約語回避)

## コンパイルエラー対応

### 1. `domain::models`のimportを削除
```rust
// Before
use crate::domain::models::{Project, TestCase, TestSuite};

// After
use crate::domain::queries::{
    ProjectReadModel, TestCaseReadModel, TestSuiteReadModel,
    ProjectQueryRepository, TestCaseQueryRepository, TestSuiteQueryRepository,
};
```

### 2. Application Serviceの調整
既存のServiceクラスは一旦使わない方向で進める。
直接QueryRepositoryを呼び出す。

### 3. フロントエンド側の型定義（参考）
フロントエンド（TypeScript）側も型を更新する必要がある。
```typescript
// Before
interface Project {
  code: string;
  title: string;
  // ...
}

// After
interface ProjectReadModel {
  code: string;
  title: string;
  description: string | null;
  local_updated_at: string;
  remote_updated_at: string | null;
  sync_status: 'clean' | 'local_modified' | 'remote_modified' | 'conflict';
  is_deleted: boolean;
}
```

## テスト戦略

### 1. ビルド確認
```bash
cargo check
```

### 2. 既存テストの実行
```bash
cargo test
```

### 3. 手動テスト（後でフロントエンド実装後）
- プロジェクト一覧表示
- テストスイート表示
- テストケース表示

## コミット戦略

Phase 1の移行は大きな変更なので、以下のように分割：

1. **準備**: Application Serviceの調整
   ```
   refactor: Application ServiceをQuery Repository対応に準備
   ```

2. **Project系コマンド移行**
   ```
   refactor: ProjectコマンドをReadModelに移行
   ```

3. **TestSuite系コマンド移行**
   ```
   refactor: TestSuiteコマンドをReadModelに移行
   ```

4. **TestCase系コマンド移行**
   ```
   refactor: TestCaseコマンドをReadModelに移行
   ```

5. **SharedStep系コマンド移行**
   ```
   refactor: SharedStepコマンドをReadModelに移行
   ```

6. **クリーンアップ**
   ```
   refactor: 未使用のimportとコードを削除
   ```

## 成功基準

- [ ] すべてのQuery系コマンドがReadModelを返す
- [ ] `cargo check`が通る
- [ ] 既存のテストが通る（少なくとも壊れない）
- [ ] Application Serviceは使わず、直接QueryRepositoryを呼ぶ
- [ ] `domain::models`の古い型への依存がQuery系から削除されている

## Phase 2以降の予定

Phase 1完了後、次のフェーズで以下を実装：
- Command側の移行（`sync_*`, `save_*`など）
- 新しいDomainモデルを使った更新処理
- トランザクション管理

## 注意事項

### 一時的に動かないコマンド
Phase 1では以下のコマンドは**コンパイルエラーになる可能性**：
- `sync_projects`
- `sync_suites`
- `sync_cases`
- `sync_shared_steps`
- `save_case`

これらは**コメントアウト**して、Phase 2で再実装する。

### AppStateの調整
`AppState`に`db: Arc<Database>`フィールドが必要。
既存の実装を確認して調整。

## 参考資料

- [Task 4: CQRS実装](./task4-cqrs-implementation.md)
- [Task 5: UI層実装計画](./task5-ui-layer-implementation.md)
- PR #27: ProjectQueryRepository実装
- PR #28: Query Repository実装完了
