# Qasery - Qase IDE 仕様書

## 概要
テスト管理サービスQaseのテストスイート、テストケース、shared stepを編集できるデスクトップIDEアプリケーション。

## 技術スタック
- **フロントエンド**: React/TypeScript
- **バックエンド**: Rust
- **フレームワーク**: Tauri
- **データベース**: SQLite
- **アーキテクチャ**: モノレポ構成

---

## 1. プロジェクト構成

### ディレクトリ構造

```
qasery/
├── docs/                     # ドキュメント
│   └── specification.md      # 本仕様書
├── backend/                  # Tauriバックエンド（Rust）
│   ├── src/
│   │   ├── main.rs
│   │   ├── domain/           # ビジネスロジック（外部依存なし）
│   │   │   ├── mod.rs
│   │   │   ├── models.rs     # エンティティ（Project, TestSuite, TestCase, SharedStep）
│   │   │   └── repository.rs # リポジトリトレイト（インターフェース）
│   │   ├── application/      # ユースケース
│   │   │   ├── mod.rs
│   │   │   └── services.rs   # ビジネスロジックの調整
│   │   ├── infrastructure/   # 外部実装
│   │   │   ├── mod.rs
│   │   │   ├── db.rs         # SQLite実装
│   │   │   └── qase_api.rs   # Qase API実装
│   │   └── presentation/     # Tauriコマンド
│   │       ├── mod.rs
│   │       └── commands.rs   # フロントエンドから呼び出されるコマンド
│   ├── Cargo.toml
│   └── tauri.conf.json
├── frontend/                 # フロントエンド（React/TypeScript）
│   ├── src/
│   │   ├── components/       # UIコンポーネント（機能ごと）
│   │   │   ├── ProjectList.tsx
│   │   │   ├── TestSuiteTree.tsx
│   │   │   ├── TestCaseEditor.tsx
│   │   │   ├── SharedStepEditor.tsx
│   │   │   ├── SharedStepUsage.tsx
│   │   │   ├── SyncDiffViewer.tsx
│   │   │   └── common/       # 共通コンポーネント
│   │   │       ├── Button.tsx
│   │   │       ├── Modal.tsx
│   │   │       └── Layout.tsx
│   │   ├── hooks/            # カスタムフック
│   │   │   ├── useProjects.ts
│   │   │   ├── useTestSuites.ts
│   │   │   ├── useTestCases.ts
│   │   │   ├── useSharedSteps.ts
│   │   │   └── useSync.ts
│   │   ├── services/         # Tauriコマンド呼び出しラッパー
│   │   │   └── tauri.ts
│   │   ├── store/            # 状態管理（Zustand）
│   │   │   └── index.ts
│   │   ├── types/            # TypeScript型定義
│   │   │   └── index.ts
│   │   ├── App.tsx           # ルートコンポーネント
│   │   └── main.tsx
│   ├── package.json
│   ├── tsconfig.json
│   └── index.html
├── package.json              # ルートのpackage.json（モノレポ管理用）
└── README.md
```

### 各層の責務

#### Domain層（domain/）
- **役割**: ビジネスロジックの中核、外部依存を持たない
- **内容**:
  - `models.rs`: エンティティ（Project, TestSuite, TestCase, SharedStep）の定義
  - `repository.rs`: データ永続化のためのトレイト定義

#### Application層（application/）
- **役割**: ユースケースの実装、ビジネスロジックの調整
- **内容**:
  - `services.rs`: 各機能のサービス（作成、編集、削除、同期など）

#### Infrastructure層（infrastructure/）
- **役割**: 外部システムとの通信、永続化の実装
- **内容**:
  - `db.rs`: SQLiteデータベースとのやり取り、リポジトリトレイトの実装
  - `qase_api.rs`: Qase APIとの通信

#### Presentation層（presentation/）
- **役割**: フロントエンドとのインターフェース
- **内容**:
  - `commands.rs`: Tauriコマンド（フロントエンドから呼び出される関数）

#### Frontend（frontend/）
- **役割**: UI/UX、ユーザー操作のハンドリング
- **内容**:
  - `components/`: UIコンポーネント
    - 機能ごとのコンポーネント（ProjectList, TestCaseEditor等）
    - `common/`: 共通コンポーネント（Button, Modal, Layout等）
  - `hooks/`: カスタムフック
    - 各機能のデータフェッチ・操作ロジック
    - Tauriコマンドとの連携
  - `services/`: Tauriコマンド呼び出しラッパー
    - バックエンドとの通信を抽象化
  - `store/`: 状態管理（Zustand）
    - グローバル状態の管理
    - シンプルで学習コストが低い
  - `types/`: TypeScript型定義
    - バックエンドと共有する型定義

### アーキテクチャの設計方針

#### Backend（Clean Architecture）
1. **テスタビリティ**: 各層が独立しているため、単体テストが書きやすい
2. **保守性**: 責務が明確で、変更の影響範囲が限定される
3. **拡張性**: 新機能追加時にどこに実装すべきか明確
4. **依存性の方向**: 外側の層が内側の層に依存（domain層は何にも依存しない）

#### Frontend（シンプル構成）
1. **学習コストを低く**: 初心者でも理解しやすいシンプルな構成
2. **段階的な拡張**: 必要に応じてリファクタリング可能
3. **責務の分離**: コンポーネント、ロジック、状態を明確に分離
4. **型安全性**: TypeScriptで型定義を共有し、バックエンドとの整合性を保つ

---

## 2. データモデル設計

### 2.1 Qase APIエンティティ

Qase API v1から取得する主要エンティティ：

#### Project
```
- code: string (2-10文字、プロジェクトの識別子)
- title: string
- description: string
```

#### TestSuite
```
- id: int64
- title: string
- description: string | null
- preconditions: string | null
- parent_id: int64 | null (階層構造をサポート)
- position: integer (表示順序)
- cases_count: integer (このスイート内のテストケース数)
- created_at: datetime
- updated_at: datetime
```

#### TestCase
```
- id: int64
- title: string
- description: string | null
- preconditions: string | null
- postconditions: string | null
- suite_id: int64 | null (所属するテストスイート)
- severity: integer
- priority: integer
- type: integer
- status: integer
- automation: integer
- steps: array of Step
  - hash: string
  - shared_step_hash: string | null (shared stepへの参照)
  - action: string
  - expected_result: string | null
  - data: string | null
  - position: integer
  - steps: array (ネストされたステップ)
- author_id: int64
- created_at: datetime
- updated_at: datetime
```

#### SharedStep
```
- hash: string (ID)
- title: string
- action: string
- expected_result: string
- steps: array of Step (ネストされたステップ)
- cases: array of int64 (このshared stepを使用しているテストケースのIDリスト)
- cases_count: integer (使用されているテストケース数)
- created_at: datetime
- updated_at: datetime
```

### 2.2 ローカルDBスキーマ（SQLite）

Qase APIのデータをローカルで管理するためのテーブル設計：

#### projects テーブル
```sql
CREATE TABLE projects (
    code TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    -- 同期管理用
    local_updated_at DATETIME NOT NULL,
    remote_updated_at DATETIME,
    sync_status TEXT NOT NULL DEFAULT 'clean', -- clean, local_modified, remote_modified, conflict
    is_deleted BOOLEAN NOT NULL DEFAULT 0
);
```

#### test_suites テーブル
```sql
CREATE TABLE test_suites (
    id INTEGER PRIMARY KEY,
    project_code TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    preconditions TEXT,
    parent_id INTEGER,
    position INTEGER NOT NULL,
    cases_count INTEGER NOT NULL DEFAULT 0,
    -- Qase API フィールド
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    -- 同期管理用
    local_updated_at DATETIME NOT NULL,
    remote_updated_at DATETIME,
    sync_status TEXT NOT NULL DEFAULT 'clean',
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (project_code) REFERENCES projects(code) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES test_suites(id) ON DELETE SET NULL
);
```

#### test_cases テーブル
```sql
CREATE TABLE test_cases (
    id INTEGER PRIMARY KEY,
    project_code TEXT NOT NULL,
    suite_id INTEGER,
    title TEXT NOT NULL,
    description TEXT,
    preconditions TEXT,
    postconditions TEXT,
    severity INTEGER,
    priority INTEGER,
    type INTEGER,
    status INTEGER,
    automation INTEGER,
    author_id INTEGER,
    -- Qase API フィールド
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    -- 同期管理用
    local_updated_at DATETIME NOT NULL,
    remote_updated_at DATETIME,
    sync_status TEXT NOT NULL DEFAULT 'clean',
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (project_code) REFERENCES projects(code) ON DELETE CASCADE,
    FOREIGN KEY (suite_id) REFERENCES test_suites(id) ON DELETE SET NULL
);
```

#### test_case_steps テーブル
```sql
CREATE TABLE test_case_steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    test_case_id INTEGER NOT NULL,
    hash TEXT NOT NULL,
    shared_step_hash TEXT,
    position INTEGER NOT NULL,
    action TEXT NOT NULL,
    expected_result TEXT,
    data TEXT,
    parent_step_id INTEGER, -- ネストされたステップ用
    FOREIGN KEY (test_case_id) REFERENCES test_cases(id) ON DELETE CASCADE,
    FOREIGN KEY (shared_step_hash) REFERENCES shared_steps(hash) ON DELETE SET NULL,
    FOREIGN KEY (parent_step_id) REFERENCES test_case_steps(id) ON DELETE CASCADE
);
```

#### shared_steps テーブル
```sql
CREATE TABLE shared_steps (
    hash TEXT PRIMARY KEY,
    project_code TEXT NOT NULL,
    title TEXT NOT NULL,
    action TEXT NOT NULL,
    expected_result TEXT NOT NULL,
    cases_count INTEGER NOT NULL DEFAULT 0,
    -- Qase API フィールド
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    -- 同期管理用
    local_updated_at DATETIME NOT NULL,
    remote_updated_at DATETIME,
    sync_status TEXT NOT NULL DEFAULT 'clean',
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (project_code) REFERENCES projects(code) ON DELETE CASCADE
);
```

#### shared_step_steps テーブル
```sql
CREATE TABLE shared_step_steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    shared_step_hash TEXT NOT NULL,
    hash TEXT NOT NULL,
    position INTEGER NOT NULL,
    action TEXT NOT NULL,
    expected_result TEXT,
    data TEXT,
    parent_step_id INTEGER, -- ネストされたステップ用
    FOREIGN KEY (shared_step_hash) REFERENCES shared_steps(hash) ON DELETE CASCADE,
    FOREIGN KEY (parent_step_id) REFERENCES shared_step_steps(id) ON DELETE CASCADE
);
```

#### shared_step_usage テーブル（使用状況を管理）
```sql
CREATE TABLE shared_step_usage (
    shared_step_hash TEXT NOT NULL,
    test_case_id INTEGER NOT NULL,
    PRIMARY KEY (shared_step_hash, test_case_id),
    FOREIGN KEY (shared_step_hash) REFERENCES shared_steps(hash) ON DELETE CASCADE,
    FOREIGN KEY (test_case_id) REFERENCES test_cases(id) ON DELETE CASCADE
);
```

### 2.3 同期状態管理

#### sync_status の値
- `clean`: ローカルとリモートが同期している
- `local_modified`: ローカルで変更されている（リモートへの反映待ち）
- `remote_modified`: リモートで変更されている（ローカルへの取り込み待ち）
- `conflict`: ローカルとリモートの両方で変更されている（マージ作業が必要）

#### 同期フロー
1. **Qaseからデータ取り込み**:
   - APIから全データをフェッチ
   - `updated_at` を比較してローカルとリモートの差分を検出
   - `sync_status` を更新

2. **Qaseへデータ反映**:
   - `sync_status = 'local_modified'` のレコードを抽出
   - Qase APIを呼び出して更新
   - 成功したら `sync_status = 'clean'` に更新

3. **コンフリクト解決**:
   - `sync_status = 'conflict'` のレコードを表示
   - ユーザーが「ローカルを優先」「リモートを優先」「手動マージ」を選択

### 2.4 エンティティ間リレーション

```
Project (1) ─── (N) TestSuite
                    │
                    ├── parent_id (self-reference for hierarchy)
                    │
                    └─── (N) TestCase
                              │
                              └─── (N) TestCaseStep
                                        │
                                        └── shared_step_hash ──> SharedStep
                                                                      │
                                                                      └─── (N) SharedStepUsage
```

### 2.5 型定義の共有（Rust ↔ TypeScript）

Rustの構造体を`serde`でシリアライズし、TypeScript側で対応する型を定義することで型安全性を確保。

**Rust側（backend/src/domain/models.rs）**:
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestCase {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    // ...
}
```

**TypeScript側（frontend/src/types/index.ts）**:
```typescript
export interface TestCase {
  id: number;
  title: string;
  description: string | null;
  // ...
}
```

## 3. アーキテクチャ設計

（次のセクションで詳細化予定）

## 4. Qase API統合

（次のセクションで詳細化予定）

## 5. 同期・差分管理

（次のセクションで詳細化予定）

## 6. UI/UX設計

（次のセクションで詳細化予定）

## 7. 開発フェーズ

（次のセクションで詳細化予定）
