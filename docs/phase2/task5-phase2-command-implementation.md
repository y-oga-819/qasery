# Task 5 - Phase 2: Command側実装計画

## 目的

Phase 1で移行したQuery側に続き、**Command側（書き込み処理）**を新しいDDD設計に基づいて実装する。

## Phase 1の成果

✅ Query側（読み取り専用コマンド）を新しいCQRS設計に移行完了
- 9個のコマンドがReadModelを返す
- QueryRepositoryを直接呼び出す方式

## Phase 2で実装するコマンド

### 1. sync_projects
Qase APIからプロジェクトを取得し、ローカルDBに保存

**実装方針**:
- Qase APIから取得したデータをDomainモデル（`domain::project::Project`）に変換
- `ProjectRepository::save()`で永続化
- 新規作成のみ対応（更新は今後）

### 2. sync_suites
Qase APIからテストスイートを取得し、ローカルDBに保存

**実装方針**:
- Qase APIから取得したデータをDomainモデル（`domain::test_suite::TestSuite`）に変換
- `TestSuiteRepository::save()`で永続化
- 階層構造（parent_id）を正しく保存

### 3. sync_cases
Qase APIからテストケースを取得し、ローカルDBに保存

**実装方針**:
- Qase APIから取得したデータをDomainモデル（`domain::test_case::TestCase`）に変換
- テストステップも含めて保存
- `TestCaseRepository::save()`で永続化

### 4. sync_shared_steps
Qase APIからShared Stepを取得し、ローカルDBに保存

**実装方針**:
- Qase APIから取得したデータをDomainモデル（`domain::shared_step::SharedStep`）に変換
- ステップも含めて保存
- `SharedStepRepository::save()`で永続化

### 5. save_case
テストケースを保存（ローカルでの編集）

**実装方針**:
- フロントエンドからTestCaseReadModelを受け取る
- Domainモデルに変換して`TestCaseRepository::save()`

## アーキテクチャ

### Command側のフロー

```
┌─────────────────────────────────────┐
│  Frontend                           │
│  invoke('sync_projects')            │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│  UI Layer (Tauri Commands)          │
│  - sync_projects                    │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│  Application Service                │ ← Phase 2で実装
│  - ProjectSyncService               │
│    1. Qase APIから取得              │
│    2. Domain Modelに変換            │
│    3. Repositoryで永続化            │
└─────────────┬───────────────────────┘
              │
    ┌─────────┴─────────┐
    │                   │
┌───▼────────┐  ┌───────▼────────┐
│  Qase API  │  │  Repository    │
│  Client    │  │  (Command)     │
└────────────┘  └────────────────┘
```

## 実装詳細

### Application Serviceの設計

現在の`application/services.rs`には古い実装が残っているので、新しい設計で再実装します。

#### ProjectSyncService

```rust
pub struct ProjectSyncService {
    repository: Arc<dyn ProjectRepository>,
    qase_client: Arc<QaseClient>,
}

impl ProjectSyncService {
    pub async fn sync_projects(&self) -> Result<usize> {
        let mut count = 0;
        let limit = 100;
        let mut offset = 0;

        loop {
            // 1. Qase APIから取得
            let response = self.qase_client.get_projects(Some(limit), Some(offset)).await?;
            let entities = response.result.entities;

            // 2. Domain Modelに変換して保存
            for entity in &entities {
                let project = self.convert_to_domain_model(entity)?;
                self.repository.save(&project)?;
                count += 1;
            }

            // 全件取得済みなら終了
            if offset + entities.len() as i32 >= response.result.total {
                break;
            }
            offset += limit;
        }

        Ok(count)
    }

    fn convert_to_domain_model(&self, entity: &ProjectEntity) -> Result<Project> {
        use crate::domain::project::{Project, ProjectCode};

        let code = ProjectCode::new(&entity.code)?;
        let project = Project::new(
            code,
            entity.title.clone(),
            entity.description.clone(),
        );

        Ok(project)
    }
}
```

#### TestSuiteSyncService

```rust
pub struct TestSuiteSyncService {
    repository: Arc<dyn TestSuiteRepository>,
    qase_client: Arc<QaseClient>,
}

impl TestSuiteSyncService {
    pub async fn sync_suites(&self, project_code: &str) -> Result<usize> {
        let mut count = 0;
        let limit = 100;
        let mut offset = 0;

        loop {
            // 1. Qase APIから取得
            let response = self.qase_client
                .get_suites(project_code, Some(limit), Some(offset))
                .await?;
            let entities = response.result.entities;

            // 2. Domain Modelに変換して保存
            for entity in &entities {
                let suite = self.convert_to_domain_model(project_code, entity)?;
                self.repository.save(&suite)?;
                count += 1;
            }

            // 全件取得済みなら終了
            if offset + entities.len() as i32 >= response.result.total {
                break;
            }
            offset += limit;
        }

        Ok(count)
    }

    fn convert_to_domain_model(
        &self,
        project_code: &str,
        entity: &SuiteEntity,
    ) -> Result<TestSuite> {
        use crate::domain::test_suite::{TestSuite, TestSuiteId};

        let id = TestSuiteId::new(entity.id);
        let suite = TestSuite::new(
            id,
            project_code.to_string(),
            entity.title.clone(),
            entity.description.clone(),
            entity.preconditions.clone(),
            entity.parent_id,
        );

        Ok(suite)
    }
}
```

### Tauri Commandの実装

```rust
#[tauri::command]
pub async fn sync_projects(state: State<'_, AppState>) -> Result<usize, String> {
    // QaseClientを取得
    let client = state.get_qase_client().map_err(|e| e.to_string())?;

    // Serviceを作成
    let service = ProjectSyncService::new(state.db.clone(), client);

    // 同期実行
    service.sync_projects().await.map_err(|e| e.to_string())
}
```

## 型変換の戦略

### Qase API Entity → Domain Model

| Qase Entity | Domain Model | 変換関数 |
|-------------|-------------|---------|
| `ProjectEntity` | `domain::project::Project` | `convert_to_domain_model()` |
| `SuiteEntity` | `domain::test_suite::TestSuite` | `convert_to_domain_model()` |
| `CaseEntity` | `domain::test_case::TestCase` | `convert_to_domain_model()` |
| `SharedStepEntity` | `domain::shared_step::SharedStep` | `convert_to_domain_model()` |

### 変換時の注意点

1. **バリデーション**: Domain Modelの`new()`でバリデーションが実行される
2. **デフォルト値**: API側にない情報は適切なデフォルト値を設定
3. **エラーハンドリング**: 変換失敗時はスキップせずエラーを返す

## 実装手順

### Step 1: Application Serviceの準備

1. `src/application/sync_services.rs`を新規作成
2. `ProjectSyncService`の骨格を作成
3. `application/mod.rs`に登録

### Step 2: sync_projectsの実装（TDD）

#### Red: 失敗するテストを追加

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_projects() {
        // モックQaseClientを用意
        // テスト実行
        // アサーション
    }
}
```

#### Green: 最小限の実装

1. `convert_to_domain_model()`を実装
2. `sync_projects()`を実装
3. Tauri Commandを有効化

#### Refactor: リファクタリング

- エラーハンドリングを改善
- ログ追加
- コメント追加

### Step 3: sync_suitesの実装（TDD）

同様のサイクルで実装

### Step 4: sync_casesの実装（TDD）

同様のサイクルで実装

### Step 5: sync_shared_stepsの実装（TDD）

同様のサイクルで実装

### Step 6: save_caseの実装（TDD）

同様のサイクルで実装

## テスト戦略

### ユニットテスト

各Serviceに対してユニットテストを作成：
- モックQaseClientを使用
- 正常系・異常系をテスト
- 変換ロジックをテスト

### 統合テスト

実際のDBを使った統合テスト：
- 同期後にQueryRepositoryでデータを取得できるか確認
- トランザクションが正しく動作するか確認

## コミット戦略

Phase 2も段階的にコミット：

1. **Application Service基盤**
   ```
   feat: Application Serviceの基盤を追加
   ```

2. **sync_projects実装**
   ```
   feat: sync_projectsコマンドを実装（TDD）
   ```

3. **sync_suites実装**
   ```
   feat: sync_suitesコマンドを実装（TDD）
   ```

4. **sync_cases実装**
   ```
   feat: sync_casesコマンドを実装（TDD）
   ```

5. **sync_shared_steps実装**
   ```
   feat: sync_shared_stepsコマンドを実装（TDD）
   ```

6. **save_case実装**
   ```
   feat: save_caseコマンドを実装（TDD）
   ```

## 成功基準

- [ ] すべてのCommand系コマンドが実装され、テストが通る
- [ ] Qase APIからデータを取得し、Domain Modelに変換できる
- [ ] Domain ModelをRepositoryで永続化できる
- [ ] QueryRepositoryで保存したデータを取得できる（ラウンドトリップテスト）
- [ ] エラーハンドリングが適切に実装されている
- [ ] Clippy警告ゼロ
- [ ] すべてのテストが通る

## 注意事項

### 既存のservices.rsの扱い

既存の`application/services.rs`は古い実装なので：
- 新しく`application/sync_services.rs`を作成
- 既存ファイルは後で削除または段階的に移行

### トランザクション管理

現時点では各コマンドで個別にコミット。
将来的にはトランザクション管理を実装する可能性あり。

### SyncStatusの扱い

同期時のSyncStatusは`Clean`に設定。
ローカル編集時は`LocalModified`に変更。

## 次のステップ

Phase 2完了後：
- CRUD操作の完全実装（create, update, delete）
- コンフリクト解決の実装
- オフライン対応
- 差分同期の最適化

## 関連ドキュメント

- [Task 5: UI層実装計画](./task5-ui-layer-implementation.md)
- [Task 5 Phase 1: Query側移行計画](./task5-phase1-migration-plan.md)
- [Task 4: CQRS実装](./task4-cqrs-implementation.md)
