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

---

## 3. 同期・差分管理

### 3.1 同期の基本フロー

#### Qaseからローカルへの取り込み（Pull）
1. Qase APIから全エンティティを取得
2. ローカルDBの`updated_at`と比較
3. 差分を検出し、`sync_status`を更新：
   - ローカルのみ変更 → `local_modified`
   - リモートのみ変更 → `remote_modified`
   - 両方変更 → `conflict`
4. 差分がある場合、ユーザーに通知

#### ローカルからQaseへの反映（Push）
1. `sync_status = 'local_modified'`のレコードを抽出
2. Qase APIを呼び出して更新
3. 成功したら`sync_status = 'clean'`、`remote_updated_at`を更新
4. 失敗したら理由を表示し、ユーザーに対応を促す

### 3.2 差分検出の方法

#### タイムスタンプベースの比較
- `updated_at`（Qase API）と`local_updated_at`（ローカルDB）を比較
- `remote_updated_at`（前回同期時のリモートタイムスタンプ）も記録

#### 判定ロジック
```
if local_updated_at > remote_updated_at AND qase_updated_at > remote_updated_at:
    → conflict (両方で変更)
elif local_updated_at > remote_updated_at:
    → local_modified (ローカルのみ変更)
elif qase_updated_at > remote_updated_at:
    → remote_modified (リモートのみ変更)
else:
    → clean (変更なし)
```

### 3.3 コンフリクト解決の方針

#### 3種類の解決方法
1. **ローカルを優先**: ローカルの変更をQaseに反映（上書き）
2. **リモートを優先**: Qaseの変更をローカルに取り込み（上書き）
3. **手動マージ**: ユーザーが差分を見ながら手動で統合

#### 差分表示
- サイドバイサイド比較UI
- フィールドごとの差分をハイライト表示
- 変更箇所のみを選択的にマージ可能

### 3.4 Shared Stepの特殊な扱い

#### 使用状況の考慮
- Shared Stepを削除する際、使用中のテストケースをチェック
- 使用中の場合は警告を表示し、削除を確認
- 削除後、関連するテストケースの同期状態も更新

#### 参照の整合性
- Shared Stepが削除された場合、テストケースのステップから参照を削除
- または、削除されたShared Stepの内容を通常のステップに変換

### 3.5 同期UI/UX

#### 同期状態の表示
- プロジェクト全体の同期状態を一目で確認
- 未同期の変更数をバッジ表示
- 最終同期日時の表示

#### バックグラウンド同期
- 定期的な自動同期（オプション）
- 手動同期ボタン
- 同期中のプログレス表示

#### エラーハンドリング
- ネットワークエラー時のリトライ機能
- 部分的な同期失敗時の詳細表示
- 失敗したアイテムのみ再同期可能

## 4. UI/UX設計

### 4.1 画面構成

#### メインレイアウト（3ペイン）
```
┌─────────────────────────────────────────────────┐
│  Header (プロジェクト選択、同期ボタン)          │
├──────────┬──────────────────┬────────────────────┤
│          │                  │                    │
│  Left    │  Center          │  Right             │
│  Pane    │  Pane            │  Pane              │
│          │                  │                    │
│ ・Project│ ・TestSuiteTree  │ ・TestCaseEditor   │
│  List    │  or              │  or                │
│          │ ・TestCaseList   │ ・SharedStepEditor │
│          │  or              │  or                │
│          │ ・SharedStepList │ ・SharedStepUsage  │
│          │                  │  or                │
│          │                  │ ・SyncDiffViewer   │
│          │                  │                    │
└──────────┴──────────────────┴────────────────────┘
```

#### 主要画面
1. **プロジェクト管理画面**
   - プロジェクト一覧
   - 新規プロジェクト作成
   - プロジェクト選択

2. **テストスイート管理画面**
   - ツリー表示（階層構造）
   - ドラッグ&ドロップで並び替え・階層変更
   - 右クリックメニュー（作成、編集、削除）

3. **テストケース編集画面**
   - タイトル、説明、preconditions、postconditionsの編集
   - ステップの追加・編集・削除・並び替え
   - Shared Stepの挿入・参照

4. **Shared Step管理画面**
   - Shared Step一覧
   - 編集画面
   - 使用箇所の表示（どのテストケースで使用されているか）

5. **同期・差分表示画面**
   - 差分一覧
   - サイドバイサイド比較
   - コンフリクト解決UI

### 4.2 ユーザーフロー

#### テストケース作成フロー
1. プロジェクトを選択
2. テストスイートを選択（または新規作成）
3. 「新規テストケース」ボタンをクリック
4. タイトル・説明を入力
5. ステップを追加：
   - 通常ステップを追加
   - または Shared Stepを挿入
6. 保存
7. 自動的に`sync_status = 'local_modified'`に設定

#### Shared Step活用フロー
1. **Shared Step作成**:
   - 「新規Shared Step」から作成
   - または 既存のテストケースステップから変換

2. **Shared Step挿入**:
   - テストケース編集中に「Shared Step挿入」
   - 一覧から選択して挿入

3. **使用箇所確認**:
   - Shared Stepを選択
   - 右ペインに使用しているテストケース一覧を表示
   - クリックで該当テストケースに遷移

#### 同期フロー
1. **Pull（Qaseから取り込み）**:
   - 「同期」ボタン → 「Qaseから取り込み」
   - 差分検出
   - 差分があれば一覧表示
   - 一括取り込み or 個別選択

2. **Push（Qaseへ反映）**:
   - 「同期」ボタン → 「Qaseへ反映」
   - `local_modified`のアイテム一覧表示
   - 確認後、反映実行

3. **コンフリクト解決**:
   - コンフリクトアイテムを選択
   - 差分表示（左：ローカル、右：リモート）
   - 3つの選択肢から選択：
     - ローカルを優先
     - リモートを優先
     - 手動マージ

### 4.3 主要な機能要件

#### 検索・フィルタリング
- テストケース検索（タイトル、説明、ステップ内容）
- Shared Step検索
- フィルタ（ステータス、優先度、担当者など）
- タグによる絞り込み

#### キーボードショートカット
- `Ctrl/Cmd + S`: 保存
- `Ctrl/Cmd + F`: 検索
- `Ctrl/Cmd + N`: 新規作成
- `Ctrl/Cmd + Shift + S`: 同期

#### ドラッグ&ドロップ
- テストスイートの階層変更
- テストケースのスイート間移動
- ステップの並び替え

#### リアルタイムプレビュー
- Markdown形式の説明をプレビュー
- ステップのプレビュー表示

### 4.4 レスポンシブデザイン

#### 画面サイズ対応
- 最小幅: 1280px
- 推奨: 1920px以上
- ペインのリサイズ可能

#### ダークモード対応
- システム設定に追従
- 手動切り替え可能

## 5. 開発フェーズ

### 5.1 MVP（最小機能セット）

#### Phase 1: 基礎機能（2-3週間）
**目標**: Qaseとの基本的な連携とローカルでのCRUD操作

**実装内容**:
- プロジェクト選択機能
- テストケースの表示・編集・保存（ローカルDB）
- テストスイートのツリー表示
- 基本的なUI（3ペインレイアウト）
- Qase APIからのデータ取り込み（Pull）

**完了条件**:
- Qaseからテストケースを取り込める
- ローカルで編集・保存できる
- テストスイートの階層を表示できる

#### Phase 2: Shared Step機能（1-2週間）
**目標**: Shared Stepの作成・利用・管理

**実装内容**:
- Shared Stepの作成・編集・削除
- テストケースへのShared Step挿入
- Shared Stepの使用箇所表示
- Shared Stepからテストケースへの遷移

**完了条件**:
- Shared Stepを作成・編集できる
- テストケースにShared Stepを挿入できる
- どのテストケースで使われているか確認できる

#### Phase 3: 同期機能（2-3週間）
**目標**: QaseとローカルDBの双方向同期

**実装内容**:
- ローカルからQaseへの反映（Push）
- 差分検出ロジック
- コンフリクト解決UI
- 同期状態の表示

**完了条件**:
- ローカルの変更をQaseに反映できる
- 差分を検出し、コンフリクトを解決できる
- 同期状態が可視化されている

### 5.2 拡張機能（Phase 4以降）

#### Phase 4: UX改善（1-2週間）
- 検索・フィルタリング機能
- キーボードショートカット
- ドラッグ&ドロップ操作
- Markdownプレビュー
- ダークモード対応

#### Phase 5: 高度な機能（2-3週間）
- バッチ操作（複数選択して一括編集）
- テストケースのインポート/エクスポート
- バックグラウンド自動同期
- 変更履歴の表示

### 5.3 技術的な実装順序

#### バックエンド優先アプローチ
1. **Infrastructure層**:
   - SQLiteのセットアップとマイグレーション
   - Qase API Clientの実装

2. **Domain層**:
   - エンティティ定義
   - Repository Trait定義

3. **Application層**:
   - サービスロジック実装
   - 同期ロジック実装

4. **Presentation層**:
   - Tauriコマンド実装

#### フロントエンド
1. **基盤構築**:
   - Zustand storeのセットアップ
   - Tauriサービスラッパー
   - 基本的なレイアウト

2. **コンポーネント実装**:
   - プロジェクト選択
   - テストスイートツリー
   - テストケースエディタ
   - Shared Step管理
   - 同期UI

### 5.4 テスト戦略

#### ユニットテスト
- Rustのロジック（services, repositories）
- フロントエンドのhooks、utils

#### 統合テスト
- Qase API連携
- SQLiteとの連携
- Tauri IPC

#### E2Eテスト（Playwright）
- 主要なユーザーフロー
- テストケース作成〜同期までの一連の流れ

### 5.5 リリース計画

#### ベータ版（MVP完成後）
- 限定ユーザーでのテスト
- フィードバック収集
- バグ修正

#### v1.0リリース
- Phase 1-3完了
- 基本的なドキュメント整備
- 一般公開

#### v1.x以降
- ユーザーフィードバックに基づく改善
- Phase 4-5の機能追加
- パフォーマンス最適化
