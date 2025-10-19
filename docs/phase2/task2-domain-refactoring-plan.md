# Domain層リファクタリング計画書

## 概要

DDD設計書（`ddd-domain-design.md`）に基づき、現在のDomain層を集約・値オブジェクト・エンティティの概念を明確にした設計にリファクタリングする。

---

## 現状分析

### 現在のDomain層の構造

**ファイル構成:**
- `src-tauri/src/domain/models.rs` - エンティティ定義
- `src-tauri/src/domain/repository.rs` - リポジトリトレイト定義
- `src-tauri/src/domain/mod.rs` - モジュール定義

**現在のモデル:**
1. `Project` - プロジェクト（同期フィールド付き）
2. `TestSuite` - テストスイート（同期フィールド + 階層構造）
3. `TestCase` - テストケース（同期フィールド）
4. `TestCaseStep` - テストケースステップ
5. `SharedStep` - 共有ステップ（同期フィールド）
6. `SharedStepStep` - 共有ステップ内のステップ
7. `SyncStatus` - 同期状態のenum

**現在の問題点:**
1. **集約境界が不明確**
   - すべてがフラットな構造で、集約ルートとエンティティの区別がない
   - 一貫性境界が定義されていない

2. **値オブジェクトが存在しない**
   - ID、順序、期待結果などがプリミティブ型のまま
   - ドメインルールが型で表現されていない

3. **ステップの順序管理が不明確**
   - `position` がi32のまま
   - 固有ステップと共有ステップの区別が不明確

4. **同期フィールドが各エンティティに散在**
   - `sync_status`, `local_updated_at`, `remote_updated_at` が各構造体に重複

---

## 目標設計（DDD準拠）

### 新しいディレクトリ構造（集約単位で整理）

```
src-tauri/src/domain/
├── mod.rs                           # モジュール定義
├── common/                          # 共通の値オブジェクト
│   ├── mod.rs
│   └── sync_metadata.rs             # SyncMetadata, SyncStatus（全集約で共通）
├── project/                         # Project集約
│   ├── mod.rs
│   ├── project.rs                   # Project集約ルート
│   ├── project_code.rs              # ProjectCode（値オブジェクト）
│   └── repository.rs                # ProjectRepository trait
├── test_suite/                      # TestSuite集約
│   ├── mod.rs
│   ├── test_suite.rs                # TestSuite集約ルート
│   ├── test_suite_id.rs             # TestSuiteId（値オブジェクト）
│   └── repository.rs                # TestSuiteRepository trait
├── test_case/                       # TestCase集約
│   ├── mod.rs
│   ├── test_case.rs                 # TestCase集約ルート
│   ├── test_case_id.rs              # TestCaseId（値オブジェクト）
│   ├── step_id.rs                   # StepId（値オブジェクト）
│   ├── test_step.rs                 # TestStep（エンティティ）
│   ├── ordered_step.rs              # OrderedStep, StepType（値オブジェクト）
│   ├── expected_result.rs           # ExpectedResult（値オブジェクト）
│   └── repository.rs                # TestCaseRepository trait
└── shared_step/                # SharedStep集約
    ├── mod.rs
    ├── shared_step.rs          # SharedStep集約ルート
    ├── shared_step_step.rs     # SharedStepStep（内部ステップエンティティ）
    ├── shared_step_id.rs       # SharedStepId（値オブジェクト）
    └── repository.rs           # SharedStepRepository trait
```

**設計方針（Rustの慣習に従う）:**
- 各集約が独立したディレクトリを持つ
- 集約内で使用する値オブジェクトは集約ディレクトリ内に配置
- 複数の集約で共有される値オブジェクト（SyncMetadata）は`common/`に配置
- **ファイル名は定義する主要な型名と一致**（`test_suite.rs` に `TestSuite` を定義）
- リポジトリトレイトも集約ごとに配置
- `mod.rs` で主要な型を re-export して外部から使いやすくする

### 値オブジェクト設計

#### 1. IDs（識別子）

**ProjectCode:**
```rust
// src-tauri/src/domain/project/project_code.rs

/// プロジェクトコード（2-10文字の識別子）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectCode(String);

impl ProjectCode {
    /// プロジェクトコードを作成
    ///
    /// # Errors
    /// - コードが2文字未満または10文字を超える場合
    /// - 許可されていない文字を含む場合
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.len() < 2 || value.len() > 10 {
            return Err(DomainError::InvalidProjectCode(
                "Project code must be 2-10 characters".to_string()
            ));
        }
        // 英数字とハイフン、アンダースコアのみ許可
        if !value.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(DomainError::InvalidProjectCode(
                "Project code contains invalid characters".to_string()
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

**TestSuiteId:**
```rust
// src-tauri/src/domain/test_suite/test_suite_id.rs

/// テストスイートID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestSuiteId(i64);

impl TestSuiteId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}
```

**TestCaseId:**
```rust
// src-tauri/src/domain/test_case/test_case_id.rs

/// テストケースID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestCaseId(i64);

impl TestCaseId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}
```

**StepId:**
```rust
// src-tauri/src/domain/test_case/step_id.rs

/// 固有ステップID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StepId(i64);

impl StepId {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}
```

**SharedStepId:**
```rust
// src-tauri/src/domain/shared_step/shared_step_id.rs

/// 共有ステップID（Qase APIではハッシュ文字列）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SharedStepId(String);

impl SharedStepId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

#### 2. OrderedStep（ステップ順序）
```rust
// src-tauri/src/domain/test_case/ordered_step.rs

use super::step_id::StepId;
use crate::domain::shared_step::shared_step_id::SharedStepId;

/// ステップの種別
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepType {
    /// 固有ステップ（TestCaseに直接紐付く）
    Own(StepId),
    /// 共有ステップ（SharedTestStepを参照）
    Shared(SharedStepId),
}

/// 順序付きステップ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedStep {
    step_type: StepType,
    position: i32,
}

impl OrderedStep {
    pub fn new_own(step_id: StepId, position: i32) -> Self { ... }
    pub fn new_shared(shared_id: SharedStepId, position: i32) -> Self { ... }

    // 順序検証ロジック
    pub fn validate_position(position: i32) -> Result<i32, DomainError> { ... }
}
```

#### 3. ExpectedResult（期待結果）
```rust
// src-tauri/src/domain/test_case/expected_result.rs

/// 期待結果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedResult(String);

impl ExpectedResult {
    pub fn new(value: String) -> Result<Self, DomainError> {
        // 空文字列の検証など
        if value.trim().is_empty() {
            return Err(DomainError::EmptyExpectedResult);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

#### 4. SyncMetadata（同期メタデータ）
```rust
// src-tauri/src/domain/common/sync_metadata.rs

/// 同期状態
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Clean,
    LocalModified,
    RemoteModified,
    Conflict,
}

/// 同期メタデータ（各集約が保持）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMetadata {
    local_updated_at: DateTime<Utc>,
    remote_updated_at: Option<DateTime<Utc>>,
    sync_status: SyncStatus,
    is_deleted: bool,
}

impl SyncMetadata {
    pub fn new() -> Self { ... }
    pub fn mark_local_modified(&mut self) { ... }
    pub fn mark_synced(&mut self, remote_time: DateTime<Utc>) { ... }
    pub fn detect_conflict(&mut self, remote_time: DateTime<Utc>) { ... }
}
```

### 集約設計

#### 1. Project集約
```rust
// src-tauri/src/domain/project/project.rs

use super::project_code::ProjectCode;
use crate::domain::common::sync_metadata::SyncMetadata;

/// プロジェクト集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    // 識別子
    code: ProjectCode,

    // 基本情報
    title: String,
    description: Option<String>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl Project {
    /// 新しいプロジェクトを作成
    pub fn new(code: ProjectCode, title: String, description: Option<String>) -> Self {
        Self {
            code,
            title,
            description,
            sync_metadata: SyncMetadata::new(),
        }
    }

    /// プロジェクトコードを取得
    pub fn code(&self) -> &ProjectCode {
        &self.code
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// タイトルを更新
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.sync_metadata.mark_local_modified();
    }

    /// 説明を更新
    pub fn update_description(&mut self, new_description: Option<String>) {
        self.description = new_description;
        self.sync_metadata.mark_local_modified();
    }

    /// 削除マークを付ける
    pub fn mark_for_deletion(&mut self) {
        self.sync_metadata.mark_deleted();
    }

    /// 同期メタデータを取得
    pub fn sync_metadata(&self) -> &SyncMetadata {
        &self.sync_metadata
    }

    /// 同期メタデータを可変参照で取得
    pub fn sync_metadata_mut(&mut self) -> &mut SyncMetadata {
        &mut self.sync_metadata
    }
}
```

#### 2. TestSuite集約
```rust
// src-tauri/src/domain/test_suite/test_suite.rs

use super::test_suite_id::TestSuiteId;
use crate::domain::common::sync_metadata::SyncMetadata;

/// テストスイート集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    // 識別子
    id: TestSuiteId,
    project_code: String,

    // 基本情報
    name: String,
    description: Option<String>,
    preconditions: Option<String>,

    // 階層構造
    parent_id: Option<TestSuiteId>,
    position: i32,

    // Qase API由来のメタデータ
    cases_count: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,

    // 階層表示用（オプショナル）
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TestSuite>>,
}

impl TestSuite {
    // ファクトリーメソッド
    pub fn new(name: String, project_code: String) -> Self { ... }

    // ビジネスロジック
    pub fn rename(&mut self, new_name: String) { ... }
    pub fn move_to(&mut self, new_parent_id: Option<TestSuiteId>) { ... }
    pub fn mark_for_deletion(&mut self) { ... }
}
```

#### 3. TestCase集約

**TestStepエンティティ（TestCase内部のエンティティ）:**
```rust
// src-tauri/src/domain/test_case/test_step.rs

use super::step_id::StepId;
use super::expected_result::ExpectedResult;

/// テストステップ（固有ステップ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    // 識別子
    id: StepId,

    // Qase API由来のメタデータ
    hash: String,

    // 基本情報
    action: String,
    expected_result: Option<ExpectedResult>,
    data: Option<String>,

    // 階層構造（ネストされたステップ用）
    parent_step_id: Option<StepId>,
}

impl TestStep {
    /// 新しいテストステップを作成
    pub fn new(
        id: StepId,
        hash: String,
        action: String,
        expected_result: Option<ExpectedResult>,
    ) -> Self {
        Self {
            id,
            hash,
            action,
            expected_result,
            data: None,
            parent_step_id: None,
        }
    }

    /// データを設定
    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    /// 親ステップを設定（ネスト構造）
    pub fn set_parent(&mut self, parent_id: Option<StepId>) {
        self.parent_step_id = parent_id;
    }

    /// アクションを取得
    pub fn action(&self) -> &str {
        &self.action
    }

    /// 期待結果を取得
    pub fn expected_result(&self) -> Option<&ExpectedResult> {
        self.expected_result.as_ref()
    }
}
```

**TestCase集約ルート:**
```rust
// src-tauri/src/domain/test_case/test_case.rs

use super::test_case_id::TestCaseId;
use super::ordered_step::OrderedStep;
use crate::domain::test_suite::TestSuiteId;
use crate::domain::common::sync_metadata::SyncMetadata;

// TestStepエンティティは別ファイル（test_step.rs）に定義

/// テストケース集約（集約ルート）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    // 識別子
    id: TestCaseId,
    project_code: String,
    suite_id: TestSuiteId,

    // 基本情報
    title: String,
    description: Option<String>,
    preconditions: Option<String>,
    postconditions: Option<String>,

    // メタデータ
    severity: i32,
    priority: i32,
    test_type: i32,
    status: i32,
    automation: i32,
    author_id: Option<i64>,

    // ステップ（順序付き）
    steps: Vec<OrderedStep>,

    // Qase API由来のメタデータ
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl TestCase {
    // ファクトリーメソッド
    pub fn new(title: String, suite_id: TestSuiteId, project_code: String) -> Self { ... }

    // ステップ操作（ドメインロジック）
    pub fn add_step(&mut self, step: OrderedStep) -> Result<(), DomainError> {
        // 重複チェック
        // 順序の一意性チェック
        // 追加
    }

    pub fn remove_step(&mut self, position: i32) -> Result<(), DomainError> { ... }

    pub fn reorder_steps(&mut self, new_order: Vec<i32>) -> Result<(), DomainError> {
        // 順序の妥当性検証
        // 重複チェック
        // 並び替え
    }

    pub fn add_shared_step(&mut self, shared_id: SharedStepId, position: i32) -> Result<(), DomainError> { ... }

    pub fn move_to_suite(&mut self, new_suite_id: TestSuiteId) {
        self.suite_id = new_suite_id;
        self.sync_metadata.mark_local_modified();
    }
}
```

#### 4. SharedStep集約
```rust
// src-tauri/src/domain/shared_step/shared_step.rs

use super::shared_step_id::SharedStepId;
use crate::domain::test_case::ExpectedResult;
use crate::domain::common::sync_metadata::SyncMetadata;

/// SharedStepの内部ステップ（エンティティ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStepStep {
    // 識別子
    id: i64,
    
    // Qase API由来のメタデータ
    hash: String,
    
    // 基本情報
    action: String,
    expected_result: Option<String>,
    data: Option<String>,
    
    // 階層構造（ネスト構造）
    parent_step_id: Option<i64>,
    
    // 順序
    position: i32,
}

/// 共有ステップ集約（集約ルート）
/// 
/// 複数のTestCaseで使い回すことが可能な、
/// 階層1の子要素（SharedStepStep）を持つTestStepのような存在
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStep {
    // 識別子
    id: SharedStepId,
    project_code: String,

    // 基本情報
    title: String,
    
    // 内部ステップ（階層1の子要素）
    steps: Vec<SharedStepStep>,

    // 使用状況
    cases_count: i32,

    // Qase API由来のメタデータ
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // 同期メタデータ
    sync_metadata: SyncMetadata,
}

impl SharedStepStep {
    /// 新しい内部ステップを作成
    pub fn new(
        id: i64,
        hash: String,
        action: String,
        expected_result: Option<String>,
        position: i32,
    ) -> Self {
        Self {
            id,
            hash,
            action,
            expected_result,
            data: None,
            parent_step_id: None,
            position,
        }
    }

    /// データを設定
    pub fn set_data(&mut self, data: Option<String>) {
        self.data = data;
    }

    /// 親ステップを設定
    pub fn set_parent(&mut self, parent_id: Option<i64>) {
        self.parent_step_id = parent_id;
    }
}

impl SharedStep {
    /// 新しい共有ステップを作成
    pub fn new(title: String, project_code: String) -> Self {
        Self {
            id: SharedStepId::new(String::new()), // Qase APIから取得後に設定
            project_code,
            title,
            steps: Vec::new(),
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sync_metadata: SyncMetadata::new(),
        }
    }

    /// 内部ステップを追加
    pub fn add_step(&mut self, step: SharedStepStep) -> Result<(), DomainError> {
        // 位置の重複チェック
        if self.steps.iter().any(|s| s.position == step.position) {
            return Err(DomainError::DuplicateStepPosition);
        }
        self.steps.push(step);
        self.sync_metadata.mark_local_modified();
        Ok(())
    }

    /// 内部ステップを削除
    pub fn remove_step(&mut self, position: i32) -> Result<(), DomainError> {
        let index = self.steps.iter().position(|s| s.position == position)
            .ok_or(DomainError::StepNotFound)?;
        self.steps.remove(index);
        self.sync_metadata.mark_local_modified();
        Ok(())
    }

    /// タイトルを更新
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.sync_metadata.mark_local_modified();
    }

    /// 削除前の検証（参照元テストケースが存在しないか確認）
    pub fn can_delete(&self) -> Result<(), DomainError> {
        if self.cases_count > 0 {
            return Err(DomainError::SharedStepInUse(self.cases_count));
        }
        Ok(())
    }

    /// 内部ステップの一覧を取得
    pub fn steps(&self) -> &[SharedStepStep] {
        &self.steps
    }
}
```

### リポジトリトレイト更新

各集約のディレクトリ内にリポジトリトレイトを配置します。

**ProjectRepository:**
```rust
// src-tauri/src/domain/project/repository.rs

use super::project::Project;
use super::project_code::ProjectCode;
use anyhow::Result;

pub trait ProjectRepository: Send + Sync {
    /// プロジェクトコードでプロジェクトを取得
    fn get_by_code(&self, code: &ProjectCode) -> Result<Option<Project>>;
    
    /// すべてのプロジェクトを取得
    fn get_all(&self) -> Result<Vec<Project>>;
    
    /// プロジェクトを保存（新規作成または更新）
    fn save(&self, project: &Project) -> Result<()>;
    
    /// プロジェクトを削除
    fn delete(&self, code: &ProjectCode) -> Result<()>;
}
```

**TestSuiteRepository:**
```rust
// src-tauri/src/domain/test_suite/repository.rs

use super::test_suite::TestSuite;
use super::test_suite_id::TestSuiteId;
use anyhow::Result;

pub trait TestSuiteRepository: Send + Sync {
    fn get_by_id(&self, id: TestSuiteId) -> Result<Option<TestSuite>>;
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuite>>;
    fn save(&self, suite: &TestSuite) -> Result<()>;
    fn delete(&self, id: TestSuiteId) -> Result<()>;
}
```

**TestCaseRepository:**
```rust
// src-tauri/src/domain/test_case/repository.rs

use super::test_case::TestCase;
use super::test_case_id::TestCaseId;
use crate::domain::test_suite::TestSuiteId;
use anyhow::Result;

pub trait TestCaseRepository: Send + Sync {
    fn get_by_id(&self, id: TestCaseId) -> Result<Option<TestCase>>;
    fn get_by_suite(&self, suite_id: TestSuiteId) -> Result<Vec<TestCase>>;
    fn save(&self, test_case: &TestCase) -> Result<()>;
    fn delete(&self, id: TestCaseId) -> Result<()>;
}
```

**SharedStepRepository:**
```rust
// src-tauri/src/domain/shared_step/repository.rs

use super::shared_step::SharedStep;
use super::shared_step_id::SharedStepId;
use anyhow::Result;

pub trait SharedStepRepository: Send + Sync {
    fn get_by_id(&self, id: SharedStepId) -> Result<Option<SharedStep>>;
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStep>>;
    fn save(&self, shared_step: &SharedStep) -> Result<()>;
    fn delete(&self, id: SharedStepId) -> Result<()>;
}
```

---

## 移行戦略

### Phase 1: 共通値オブジェクトの導入（影響小）

**目的**: 全集約で共通利用する値オブジェクトを作成

**タスク:**
1. `src-tauri/src/domain/common/` ディレクトリ作成
2. `sync_metadata.rs` 実装（SyncMetadata, SyncStatus）
3. ユニットテスト追加

**影響範囲:**
- Domain層のみ（Infrastructure層、Application層は未変更）
- 既存のmodels.rsは残す

**検証方法:**
- ユニットテストが通ること
- コンパイルエラーがないこと

### Phase 2: TestSuite集約の導入（影響小〜中）

**目的**: TestSuite集約とその値オブジェクトを作成

**タスク:**
1. `src-tauri/src/domain/test_suite/` ディレクトリ作成
2. `test_suite_id.rs` 実装（TestSuiteId値オブジェクト）
3. `test_suite.rs` 実装（TestSuite集約ルート）
4. `repository.rs` 実装（TestSuiteRepositoryトレイト）
5. `mod.rs` 実装（モジュール公開設定）
6. ユニットテスト追加

**影響範囲:**
- Domain層（test_suite/ディレクトリ追加）
- 既存のmodels.rsは残す

**検証方法:**
- TestSuite集約のドメインロジックのテストが通ること
- 制約違反が正しくエラーになること

### Phase 3: Project集約の導入（影響小）

**目的**: Project集約とその値オブジェクトを作成

**タスク:**
1. `src-tauri/src/domain/project/` ディレクトリ作成
2. `project_code.rs` 実装（ProjectCode値オブジェクト、バリデーション含む）
3. `project.rs` 実装（Project集約ルート）
4. `repository.rs` 実装（ProjectRepositoryトレイト）
5. `mod.rs` 実装（モジュール公開設定）
6. ユニットテスト追加

**影響範囲:**
- Domain層（project/ディレクトリ追加）
- 既存のmodels.rsは残す

**検証方法:**
- ProjectCode値オブジェクトのバリデーションが正しく動作すること
- Project集約のドメインロジックのテストが通ること
- プロジェクトコードの長さと文字種制約が正しく機能すること

### Phase 4: TestCase集約の導入（影響中）

**目的**: TestCase集約とその値オブジェクト・エンティティを作成

**タスク:**
1. `src-tauri/src/domain/test_case/` ディレクトリ作成
2. `test_case_id.rs` 実装（TestCaseId値オブジェクト）
3. `step_id.rs` 実装（StepId値オブジェクト）
4. `expected_result.rs` 実装（ExpectedResult値オブジェクト）
5. `ordered_step.rs` 実装（OrderedStep, StepType値オブジェクト）
6. `test_step.rs` 実装（TestStepエンティティ）
7. `test_case.rs` 実装（TestCase集約ルート）
8. `repository.rs` 実装（TestCaseRepositoryトレイト）
9. `mod.rs` 実装（モジュール公開設定）
10. ユニットテスト追加

**影響範囲:**
- Domain層（test_case/ディレクトリ追加）
- 既存のmodels.rsは残す

**検証方法:**
- TestCase集約のドメインロジックのテストが通ること
- ステップの順序管理が正しく動作すること
- 制約違反が正しくエラーになること

### Phase 5: SharedStep集約の導入（影響小〜中）

**目的**: SharedStep集約とその値オブジェクトを作成

**タスク:**
1. `src-tauri/src/domain/shared_step/` ディレクトリ作成
2. `shared_step_id.rs` 実装（SharedStepId値オブジェクト）
3. `shared_step_step.rs` 実装（SharedStepStep内部ステップエンティティ）
4. `shared_step.rs` 実装（SharedStep集約ルート、内部ステップ管理含む）
5. `repository.rs` 実装（SharedStepRepositoryトレイト）
6. `mod.rs` 実装（モジュール公開設定）
7. ユニットテスト追加（内部ステップの追加/削除/順序管理）

**影響範囲:**
- Domain層（shared_step/ディレクトリ追加）
- 既存のmodels.rsは残す

**検証方法:**
- SharedStep集約のドメインロジックのテストが通ること
- 内部ステップの追加/削除/順序管理が正しく動作すること
- 削除前の検証ロジックが正しく動作すること（cases_count > 0で削除不可）

### Phase 6: Infrastructure層の更新（影響大）

**目的**: 新しい集約・VOに対応したリポジトリ実装

**タスク:**
1. `infrastructure/db.rs` を更新
   - 値オブジェクトのシリアライズ/デシリアライズ対応
   - 各集約の永続化ロジック更新（TestSuite → TestCase → SharedTestStep）
2. リポジトリトレイト実装の更新
3. 統合テスト追加（DB ↔ 集約の変換テスト）

**影響範囲:**
- Infrastructure層（db.rs, qase_api.rs）

**検証方法:**
- 統合テストが通ること
- DB保存・取得が正しく動作すること
- 値オブジェクトの変換が正しく動作すること

### Phase 7: Application層・Presentation層の更新（影響大）

**目的**: 新しい集約を使用するようアプリケーションロジックを更新

**タスク:**
1. `application/appstate.rs` の更新
2. `presentation/commands.rs` の更新
3. フロントエンド型定義の更新（`frontend/src/types/index.ts`）
4. E2Eテストの更新

**影響範囲:**
- Application層
- Presentation層
- フロントエンド（型定義のみ）

**検証方法:**
- すべてのTauriコマンドが動作すること
- フロントエンドとの連携が正常に動作すること

### Phase 8: 旧models.rsの削除とクリーンアップ

**目的**: 旧モデルを削除し、新設計に完全移行

**タスク:**
1. `src-tauri/src/domain/models.rs` 削除
2. `src-tauri/src/domain/repository.rs` 削除（各集約に移動済み）
3. 未使用のimport削除
4. ドキュメント更新
5. 全体テスト実行

**影響範囲:**
- Domain層（models.rs, repository.rs削除）
- ドキュメント

**検証方法:**
- すべてのテストが通ること
- コンパイルエラーがないこと
- CIが通ること

---

## リスクと対策

### リスク1: 変更範囲が大きすぎる

**対策:**
- Phaseを細かく分割し、各Phaseでコミット
- 旧モデルと新モデルを一時的に共存させる
- Phase 2完了まで既存コードに影響を与えない

### リスク2: テストの不足

**対策:**
- 各Phaseでテストを追加
- ドメインロジックのユニットテスト優先
- 統合テスト、E2Eテストも追加

### リスク3: Infrastructure層の変更が複雑

**対策:**
- Phase 3を細かく分割（TestSuite → TestCase → SharedStep）
- 各集約ごとにリポジトリ実装を更新
- 統合テストで検証

---

## スケジュール見積もり

| Phase | タスク | 見積もり時間 | 累積時間 |
|-------|--------|--------------|----------|
| Phase 1 | 共通値オブジェクトの導入 | 2-3時間 | 2-3時間 |
| Phase 2 | TestSuite集約の導入 | 3-4時間 | 5-7時間 |
| Phase 3 | Project集約の導入 | 2-3時間 | 7-10時間 |
| Phase 4 | TestCase集約の導入 | 5-7時間 | 12-17時間 |
| Phase 5 | SharedStep集約の導入（内部ステップ含む） | 4-6時間 | 16-23時間 |
| Phase 6 | Infrastructure層の更新 | 8-12時間 | 24-35時間 |
| Phase 7 | Application層・Presentation層の更新 | 6-8時間 | 30-43時間 |
| Phase 8 | 旧models.rsの削除とクリーンアップ | 2-3時間 | 32-46時間 |

**合計見積もり**: 32-46時間（4-6日間）

**集約単位の利点:**
- Phase 2-5で集約ごとに独立して開発可能
- 各Phaseが小さく、レビューしやすい
- 問題が発生した場合も影響範囲が限定的

---

## 成功基準

1. **すべてのテストが通る**
   - ユニットテスト
   - 統合テスト
   - E2Eテスト
   - CI/CD

2. **ドメインロジックが集約内に集約されている**
   - ステップの順序検証
   - 重複チェック
   - 制約違反のエラーハンドリング

3. **集約境界が明確**
   - 一貫性の範囲が明確
   - ID参照による疎結合

4. **値オブジェクトでドメインルールを表現**
   - IDの型安全性
   - 期待結果の検証
   - 同期メタデータのカプセル化

5. **既存機能が壊れていない**
   - フロントエンドとの連携が正常
   - Qase APIとの同期が正常

---

## 次のステップ

1. **このリファクタリング計画をレビュー**
   - ユーザーに確認してもらう
   - フィードバックを反映

2. **新しいブランチを作成**
   ```bash
   git checkout -b refactor/domain-ddd-design
   ```

3. **Phase 1から開始**
   - 値オブジェクトの実装
   - TDDサイクルを守る（Red → Green → Refactor）

4. **各Phaseごとにコミット**
   - 小さいステップで進める
   - レビュー可能な単位でコミット
