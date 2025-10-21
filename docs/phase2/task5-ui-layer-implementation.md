# Task 5: UI層（Tauri Command）実装計画

## 目的

フロントエンド（React/TypeScript）とバックエンド（Rust）を接続するUI層（Tauri Command）を実装する。
CQRSアーキテクチャに基づき、QueryとCommandのエンドポイントを提供する。

## 背景

- PR #26: Infrastructure層のAggregate単位分割（Command Repository完了）
- PR #27: ProjectQueryRepository実装（Query側の基盤）
- PR #28: 残りのQuery Repository実装完了（TestSuite, TestCase, SharedStep）

**現状**: Domain層とInfrastructure層は完成。フロントエンドから呼び出すためのUI層が未実装。

## アーキテクチャ

```
┌─────────────────────────────────────┐
│  Frontend (React/TypeScript)        │
│  invoke('list_projects')            │
└─────────────┬───────────────────────┘
              │ Tauri IPC
┌─────────────▼───────────────────────┐
│  UI Layer (Tauri Commands)          │ ← 本タスクで実装
│  #[tauri::command]                  │
│  - Query Commands (読み取り)         │
│  - Command Commands (書き込み)       │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│  Application Layer (Services)       │
│  - ProjectService                   │
│  - TestSuiteService                 │
│  - TestCaseService                  │
│  - SharedStepService                │
└─────────────┬───────────────────────┘
              │
    ┌─────────┴─────────┐
    │                   │
┌───▼────────┐  ┌───────▼────────┐
│  Query     │  │  Command       │
│  Repo      │  │  Repo          │
└────────────┘  └────────────────┘

```

## ファイル構成

```
src-tauri/src/
├── commands/              # 新規作成
│   ├── mod.rs            # モジュール定義
│   ├── project.rs        # Projectコマンド
│   ├── test_suite.rs     # TestSuiteコマンド
│   ├── test_case.rs      # TestCaseコマンド
│   └── shared_step.rs    # SharedStepコマンド
├── application/
│   └── services.rs       # 既存（Application Service）
├── domain/               # 既存
├── infrastructure/       # 既存
└── main.rs              # コマンド登録を追加
```

## 実装する機能

### Phase 1: Project Commands（優先度：高）

#### Query系
- `list_projects() -> Result<Vec<ProjectReadModel>, String>`
  - すべてのプロジェクトを取得

- `get_project(code: String) -> Result<Option<ProjectReadModel>, String>`
  - 特定のプロジェクトを取得

#### Command系
- `create_project(code: String, title: String, description: Option<String>) -> Result<(), String>`
  - 新規プロジェクト作成

- `update_project(code: String, title: String, description: Option<String>) -> Result<(), String>`
  - プロジェクト更新

- `delete_project(code: String) -> Result<(), String>`
  - プロジェクト削除（論理削除）

### Phase 2: TestSuite Commands（優先度：高）

#### Query系
- `list_test_suites(project_code: String) -> Result<Vec<TestSuiteReadModel>, String>`
  - プロジェクトのテストスイート一覧を取得

- `get_test_suite(id: i64) -> Result<Option<TestSuiteReadModel>, String>`
  - 特定のテストスイートを取得

#### Command系
- `create_test_suite(...) -> Result<i64, String>`
  - 新規テストスイート作成

- `update_test_suite(...) -> Result<(), String>`
  - テストスイート更新

- `delete_test_suite(id: i64) -> Result<(), String>`
  - テストスイート削除

### Phase 3: TestCase Commands（優先度：中）

#### Query系
- `list_test_cases(suite_id: i64) -> Result<Vec<TestCaseReadModel>, String>`
- `get_test_case(id: i64) -> Result<Option<TestCaseReadModel>, String>`
- `get_test_case_steps(test_case_id: i64) -> Result<Vec<TestCaseStepReadModel>, String>`

#### Command系
- `create_test_case(...) -> Result<i64, String>`
- `update_test_case(...) -> Result<(), String>`
- `delete_test_case(id: i64) -> Result<(), String>`
- `add_test_case_step(...) -> Result<(), String>`
- `remove_test_case_step(...) -> Result<(), String>`

### Phase 4: SharedStep Commands（優先度：中）

#### Query系
- `list_shared_steps(project_code: String) -> Result<Vec<SharedStepReadModel>, String>`
- `get_shared_step(hash: String) -> Result<Option<SharedStepReadModel>, String>`
- `get_shared_step_steps(shared_step_hash: String) -> Result<Vec<SharedStepStepReadModel>, String>`

#### Command系
- `create_shared_step(...) -> Result<String, String>`
- `update_shared_step(...) -> Result<(), String>`
- `delete_shared_step(hash: String) -> Result<(), String>`
- `add_shared_step_step(...) -> Result<(), String>`
- `remove_shared_step_step(...) -> Result<(), String>`

## 実装方針

### 1. エラーハンドリング

```rust
// anyhow::Result を String に変換してフロントエンドに返す
query_repo
    .get_all()
    .map_err(|e| e.to_string())
```

### 2. 状態管理

```rust
// main.rsでAppStateを定義
pub struct AppState {
    pub db: Database,
}

// Tauri Commandで使用
#[tauri::command]
pub async fn list_projects(
    state: State<'_, AppState>
) -> Result<Vec<ProjectReadModel>, String> {
    let db = &state.db;
    let query_repo = db as &dyn ProjectQueryRepository;
    query_repo.get_all().map_err(|e| e.to_string())
}
```

### 3. CQRS分離

- **Query Commands**: QueryRepositoryを直接使用
  - 読み取り専用
  - ReadModelを返す

- **Command Commands**: Application Serviceを経由
  - ビジネスロジックを実行
  - ドメインモデルを操作
  - 結果を永続化

### 4. テスト戦略

- 各コマンドに対してユニットテストを作成
- モックを使わず実際のDatabaseを使用（in-memory SQLite）
- Query/Commandのラウンドトリップテスト

例：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_list_projects() {
        let state = setup_test_state();

        // Create
        let result = create_project(
            "DEMO".to_string(),
            "Demo Project".to_string(),
            None,
            state.clone()
        ).await;
        assert!(result.is_ok());

        // List
        let projects = list_projects(state).await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].code, "DEMO");
    }
}
```

## 実装手順

### Step 1: 基盤の準備
1. `src/commands/mod.rs` を作成
2. `src/commands/project.rs` を作成（空の骨格）
3. `src/main.rs` にAppStateとコマンド登録を追加

### Step 2: Project Query Commands（TDD）
1. `list_projects` のテストを追加（Red）
2. `list_projects` を実装（Green）
3. リファクタリング（Refactor）
4. コミット

5. `get_project` のテストを追加（Red）
6. `get_project` を実装（Green）
7. リファクタリング（Refactor）
8. コミット

### Step 3: Project Command Commands（TDD）
1. `create_project` のテストを追加（Red）
2. `create_project` を実装（Green）
3. リファクタリング（Refactor）
4. コミット

5-12. `update_project`, `delete_project` も同様

### Step 4: TestSuite Commands
- Project Commandsと同じTDDサイクルで実装

### Step 5: TestCase Commands
- 同上

### Step 6: SharedStep Commands
- 同上

### Step 7: 統合テスト
- すべてのコマンドが連携して動作することを確認
- エラーケースのテスト

### Step 8: PR作成
- すべてのテストが通ることを確認
- Clippy/fmtチェック
- PRを作成

## コミット戦略

CLAUDE.mdのTDD方針に従い、Red-Green-Refactorごとにコミット：

```bash
# Red
test: list_projectsの失敗するテストを追加

# Green
feat: list_projectsを実装

# Refactor
refactor: list_projectsのエラーハンドリングを改善

# 次のコマンドへ
test: get_projectの失敗するテストを追加
...
```

## 成功基準

- [ ] すべてのQuery Commandsが実装され、テストが通る
- [ ] すべてのCommand Commandsが実装され、テストが通る
- [ ] エラーハンドリングが適切に実装されている
- [ ] フロントエンドから呼び出し可能（型定義も含む）
- [ ] Clippy警告ゼロ
- [ ] すべてのテストが通る

## 備考

### フロントエンドとの連携

フロントエンド側では以下のように呼び出せる：

```typescript
// TypeScript
import { invoke } from '@tauri-apps/api/tauri';

interface ProjectReadModel {
  code: string;
  title: string;
  description: string | null;
  local_updated_at: string;
  remote_updated_at: string | null;
  sync_status: 'clean' | 'local_modified' | 'remote_modified' | 'conflict';
  is_deleted: boolean;
}

// プロジェクト一覧を取得
const projects = await invoke<ProjectReadModel[]>('list_projects');

// プロジェクトを作成
await invoke('create_project', {
  code: 'DEMO',
  title: 'Demo Project',
  description: 'This is a demo project'
});
```

### 今後の拡張

- 同期機能（Pull/Push）のコマンド追加
- バッチ操作のコマンド
- 検索・フィルタリング機能
- ページネーション対応

## 関連ドキュメント

- [Task 4: CQRS実装](./task4-cqrs-implementation.md)
- [Phase 2 チェックリスト](./phase2-checklist.md)
- [Phase 2 アクションプラン](./phase2-action-plan.md)
