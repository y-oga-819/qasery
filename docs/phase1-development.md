# Phase 1: 基礎機能 - 開発タスク

## 目標
Qaseとの基本的な連携とローカルでのCRUD操作

## 完了条件
- [ ] Qaseからテストケースを取り込める
- [ ] ローカルで編集・保存できる
- [ ] テストスイートの階層を表示できる

---

## 開発タスク

### 1. プロジェクトセットアップ

#### 1.1 Tauriプロジェクト初期化
- [x] Tauri CLIのインストール
- [x] Tauriプロジェクトの作成（`npm create tauri-app`）
- [x] ディレクトリ構造の整理（backend/, frontend/）
- [x] package.jsonの設定（モノレポ構成）

#### 1.2 Rustバックエンド構成
- [x] Cargo.tomlの依存関係設定
  - [x] `serde`, `serde_json`（シリアライゼーション）
  - [x] `tokio`（非同期ランタイム）
  - [x] `rusqlite`（SQLite接続）
  - [x] `reqwest`（HTTP クライアント、Qase API用）
  - [x] `chrono`（日時処理）
  - [x] `anyhow`, `thiserror`（エラーハンドリング）
- [x] backend/srcディレクトリ構造の作成
  - [x] `domain/mod.rs`
  - [x] `application/mod.rs`
  - [x] `infrastructure/mod.rs`
  - [x] `presentation/mod.rs`

#### 1.3 フロントエンド構成
- [ ] package.jsonの依存関係設定
  - [ ] React, TypeScript
  - [ ] Zustand（状態管理）
  - [ ] TailwindCSS（スタイリング）
  - [ ] React Router（ルーティング、必要に応じて）
- [ ] frontend/srcディレクトリ構造の作成
  - [ ] `components/`
  - [ ] `hooks/`
  - [ ] `services/`
  - [ ] `store/`
  - [ ] `types/`
- [ ] TailwindCSSのセットアップ

---

### 2. バックエンド実装

#### 2.1 Domain層（backend/src/domain/）

##### 2.1.1 models.rs - エンティティ定義
- [x] `Project` struct
  - [x] code, title, description
  - [x] local_updated_at, remote_updated_at, sync_status, is_deleted
- [x] `TestSuite` struct
  - [x] id, project_code, title, description, preconditions
  - [x] parent_id, position, cases_count
  - [x] created_at, updated_at
  - [x] local_updated_at, remote_updated_at, sync_status, is_deleted
- [x] `TestCase` struct
  - [x] id, project_code, suite_id, title, description
  - [x] preconditions, postconditions
  - [x] severity, priority, type, status, automation, author_id
  - [x] created_at, updated_at
  - [x] local_updated_at, remote_updated_at, sync_status, is_deleted
- [x] `TestCaseStep` struct
  - [x] id, test_case_id, hash, shared_step_hash
  - [x] position, action, expected_result, data
  - [x] parent_step_id
- [x] `SyncStatus` enum（clean, local_modified, remote_modified, conflict）

##### 2.1.2 repository.rs - リポジトリトレイト
- [x] `ProjectRepository` trait
  - [x] `get_all()`, `get_by_code()`, `save()`, `delete()`
- [x] `TestSuiteRepository` trait
  - [x] `get_by_project()`, `get_by_id()`, `save()`, `delete()`
  - [ ] `get_tree()` (階層構造取得)
- [x] `TestCaseRepository` trait
  - [x] `get_by_suite()`, `get_by_id()`, `save()`, `delete()`
  - [x] `get_steps()`, `save_steps()`

#### 2.2 Infrastructure層（backend/src/infrastructure/）

##### 2.2.1 db.rs - SQLite実装
- [x] データベース接続の初期化
- [x] マイグレーション機能
  - [x] `projects`テーブル作成
  - [x] `test_suites`テーブル作成
  - [x] `test_cases`テーブル作成
  - [x] `test_case_steps`テーブル作成
  - [x] インデックスの作成
- [x] `ProjectRepository`実装
  - [x] `get_all()` - 全プロジェクト取得
  - [x] `get_by_code()` - コードでプロジェクト取得
  - [x] `save()` - プロジェクト保存/更新
  - [x] `delete()` - プロジェクト削除
- [x] `TestSuiteRepository`実装
  - [x] `get_by_project()` - プロジェクト別スイート取得
  - [x] `get_by_id()` - IDでスイート取得
  - [ ] `get_tree()` - 階層構造で取得（parent_idを使用）
  - [x] `save()` - スイート保存/更新
  - [x] `delete()` - スイート削除
- [x] `TestCaseRepository`実装
  - [x] `get_by_suite()` - スイート別ケース取得
  - [x] `get_by_id()` - IDでケース取得
  - [x] `save()` - ケース保存/更新
  - [x] `delete()` - ケース削除
  - [x] `get_steps()` - ステップ取得
  - [x] `save_steps()` - ステップ保存

##### 2.2.2 qase_api.rs - Qase API Client
- [ ] API Clientの初期化（APIトークン管理）
- [ ] Projects API
  - [ ] `get_projects()` - 全プロジェクト取得
  - [ ] `get_project(code)` - プロジェクト詳細取得
- [ ] Test Suites API
  - [ ] `get_suites(project_code)` - スイート一覧取得
  - [ ] `get_suite(project_code, id)` - スイート詳細取得
- [ ] Test Cases API
  - [ ] `get_cases(project_code)` - ケース一覧取得
  - [ ] `get_case(project_code, id)` - ケース詳細取得
- [ ] エラーハンドリング
  - [ ] APIレスポンスのパース
  - [ ] レート制限の考慮
  - [ ] ネットワークエラーハンドリング

#### 2.3 Application層（backend/src/application/）

##### 2.3.1 services.rs - ビジネスロジック
- [ ] `ProjectService`
  - [ ] `list_projects()` - ローカルのプロジェクト一覧取得
  - [ ] `get_project(code)` - プロジェクト詳細取得
  - [ ] `sync_projects()` - Qaseからプロジェクト取り込み
- [ ] `TestSuiteService`
  - [ ] `get_suites_tree(project_code)` - 階層構造で取得
  - [ ] `get_suite(id)` - スイート詳細取得
  - [ ] `sync_suites(project_code)` - Qaseからスイート取り込み
- [ ] `TestCaseService`
  - [ ] `get_cases_by_suite(suite_id)` - スイート別ケース取得
  - [ ] `get_case(id)` - ケース詳細取得（ステップ含む）
  - [ ] `save_case(test_case)` - ケース保存（local_modified設定）
  - [ ] `sync_cases(project_code)` - Qaseからケース取り込み
- [ ] 同期ロジック（Pull用）
  - [ ] APIから取得したデータとローカルDBを比較
  - [ ] `updated_at`の比較で差分検出
  - [ ] `sync_status`の更新

#### 2.4 Presentation層（backend/src/presentation/）

##### 2.4.1 commands.rs - Tauriコマンド
- [ ] プロジェクト関連コマンド
  - [ ] `list_projects()` - プロジェクト一覧
  - [ ] `get_project(code)` - プロジェクト取得
  - [ ] `sync_projects()` - プロジェクト同期
- [ ] テストスイート関連コマンド
  - [ ] `get_suites_tree(project_code)` - ツリー取得
  - [ ] `get_suite(id)` - スイート取得
  - [ ] `sync_suites(project_code)` - スイート同期
- [ ] テストケース関連コマンド
  - [ ] `get_cases_by_suite(suite_id)` - ケース一覧
  - [ ] `get_case(id)` - ケース詳細
  - [ ] `save_case(test_case)` - ケース保存
  - [ ] `sync_cases(project_code)` - ケース同期
- [ ] 設定関連コマンド
  - [ ] `save_api_token(token)` - APIトークン保存
  - [ ] `get_api_token()` - APIトークン取得

---

### 3. フロントエンド実装

#### 3.1 型定義（frontend/src/types/index.ts）
- [ ] `Project` interface
- [ ] `TestSuite` interface
- [ ] `TestCase` interface
- [ ] `TestCaseStep` interface
- [ ] `SyncStatus` type
- [ ] Tauri Commandの戻り値型定義

#### 3.2 Tauriサービス（frontend/src/services/tauri.ts）
- [ ] プロジェクト関連
  - [ ] `listProjects()`
  - [ ] `getProject(code)`
  - [ ] `syncProjects()`
- [ ] テストスイート関連
  - [ ] `getSuitesTree(projectCode)`
  - [ ] `getSuite(id)`
  - [ ] `syncSuites(projectCode)`
- [ ] テストケース関連
  - [ ] `getCasesBySuite(suiteId)`
  - [ ] `getCase(id)`
  - [ ] `saveCase(testCase)`
  - [ ] `syncCases(projectCode)`
- [ ] 設定関連
  - [ ] `saveApiToken(token)`
  - [ ] `getApiToken()`

#### 3.3 Zustand Store（frontend/src/store/index.ts）
- [ ] プロジェクトストア
  - [ ] `projects: Project[]`
  - [ ] `selectedProject: Project | null`
  - [ ] `setProjects()`, `selectProject()`
- [ ] テストスイートストア
  - [ ] `suites: TestSuite[]`
  - [ ] `selectedSuite: TestSuite | null`
  - [ ] `setSuites()`, `selectSuite()`
- [ ] テストケースストア
  - [ ] `cases: TestCase[]`
  - [ ] `selectedCase: TestCase | null`
  - [ ] `setCases()`, `selectCase()`, `updateCase()`
- [ ] UI状態ストア
  - [ ] `loading: boolean`
  - [ ] `error: string | null`
  - [ ] `setLoading()`, `setError()`

#### 3.4 カスタムフック（frontend/src/hooks/）

##### useProjects.ts
- [ ] `useProjects()` - プロジェクト一覧取得
- [ ] `useSyncProjects()` - プロジェクト同期

##### useTestSuites.ts
- [ ] `useTestSuites(projectCode)` - スイート一覧取得
- [ ] `useSyncSuites(projectCode)` - スイート同期

##### useTestCases.ts
- [ ] `useTestCases(suiteId)` - ケース一覧取得
- [ ] `useTestCase(id)` - ケース詳細取得
- [ ] `useSaveTestCase()` - ケース保存
- [ ] `useSyncCases(projectCode)` - ケース同期

#### 3.5 共通コンポーネント（frontend/src/components/common/）

##### Layout.tsx
- [ ] 3ペインレイアウトの実装
- [ ] ヘッダー（プロジェクト選択、同期ボタン）
- [ ] Left, Center, Rightペインの配置
- [ ] ペインのリサイズ機能（オプション）

##### Button.tsx
- [ ] プライマリボタン
- [ ] セカンダリボタン
- [ ] 危険操作ボタン
- [ ] ローディング状態の表示

##### Modal.tsx
- [ ] 汎用モーダルコンポーネント
- [ ] 確認ダイアログ
- [ ] エラー表示ダイアログ

##### Spinner.tsx
- [ ] ローディングスピナー

##### ErrorMessage.tsx
- [ ] エラーメッセージ表示

#### 3.6 機能コンポーネント（frontend/src/components/）

##### ProjectList.tsx
- [ ] プロジェクト一覧表示
- [ ] プロジェクト選択機能
- [ ] 選択中プロジェクトのハイライト
- [ ] 同期ボタン（プロジェクト同期）
- [ ] ローディング状態表示
- [ ] エラー表示

##### TestSuiteTree.tsx
- [ ] ツリー構造でのスイート表示
- [ ] 階層構造の表示（parent_id利用）
- [ ] スイート選択機能
- [ ] 展開/折りたたみ機能
- [ ] ケース数表示（cases_count）
- [ ] 同期ボタン（スイート同期）

##### TestCaseEditor.tsx
- [ ] テストケース詳細の表示
- [ ] タイトル編集
- [ ] 説明編集（テキストエリア）
- [ ] Preconditions編集
- [ ] Postconditions編集
- [ ] ステップ一覧表示
  - [ ] ステップの追加ボタン
  - [ ] ステップの削除ボタン
  - [ ] ステップの並び替え（position）
- [ ] 保存ボタン
- [ ] キャンセルボタン
- [ ] 変更検知（ダーティフラグ）
- [ ] 保存前の確認ダイアログ

##### StepEditor.tsx (TestCaseEditor内で使用)
- [ ] ステップの編集フォーム
  - [ ] Action入力
  - [ ] Expected Result入力
  - [ ] Data入力（オプション）
- [ ] インライン編集機能

##### Header.tsx
- [ ] プロジェクト選択ドロップダウン
- [ ] 同期ボタン（全体同期）
- [ ] 設定ボタン（APIトークン設定）
- [ ] 同期状態のバッジ表示

##### SettingsModal.tsx
- [ ] APIトークン入力フォーム
- [ ] APIトークン保存
- [ ] 検証機能（APIトークンが有効か確認）

#### 3.7 App.tsx
- [ ] ルートコンポーネント
- [ ] Layoutの配置
- [ ] 初期化処理
  - [ ] APIトークンの確認
  - [ ] プロジェクト一覧の取得
- [ ] グローバルエラーハンドリング

---

### 4. 統合とテスト

#### 4.1 開発環境での動作確認
- [ ] `npm run tauri dev` で起動確認
- [ ] Rust側のログ確認
- [ ] フロントエンドのコンソールログ確認

#### 4.2 機能テスト
- [ ] APIトークン設定
- [ ] プロジェクト一覧取得
- [ ] プロジェクト選択
- [ ] テストスイート一覧取得（階層表示）
- [ ] テストケース一覧取得
- [ ] テストケース詳細表示
- [ ] テストケース編集
- [ ] テストケース保存
- [ ] 同期機能（Pull）
  - [ ] プロジェクト同期
  - [ ] テストスイート同期
  - [ ] テストケース同期

#### 4.3 エラーハンドリングの確認
- [ ] APIトークンが無効な場合
- [ ] ネットワークエラー
- [ ] データベースエラー
- [ ] 空のプロジェクト/スイート/ケース

#### 4.4 パフォーマンス確認
- [ ] 大量のテストケース（100件以上）での動作
- [ ] 深い階層のテストスイート（5階層以上）での動作
- [ ] 同期処理の速度

---

## 実装優先順位

### 第1優先: プロジェクト選択までの最小構成
1. プロジェクトセットアップ（タスク1）
2. Domain層のモデル定義（タスク2.1）
3. Infrastructure層のDB初期化（タスク2.2.1の一部）
4. Qase API Client（Projects APIのみ）（タスク2.2.2の一部）
5. ProjectService（タスク2.3.1の一部）
6. Tauriコマンド（プロジェクト関連のみ）（タスク2.4.1の一部）
7. フロントエンド型定義（タスク3.1）
8. Tauriサービス（プロジェクト関連のみ）（タスク3.2の一部）
9. Zustandストア（プロジェクトのみ）（タスク3.3の一部）
10. Layout, Button, Spinner（タスク3.5の一部）
11. ProjectList（タスク3.6の一部）
12. Header, SettingsModal（タスク3.6の一部）
13. App.tsx（タスク3.7）

### 第2優先: テストスイート表示
14. TestSuite関連のDB実装（タスク2.2.1の一部）
15. Qase API Client（Test Suites API）（タスク2.2.2の一部）
16. TestSuiteService（タスク2.3.1の一部）
17. Tauriコマンド（テストスイート関連）（タスク2.4.1の一部）
18. Tauriサービス（テストスイート関連）（タスク3.2の一部）
19. Zustandストア（テストスイート）（タスク3.3の一部）
20. useTestSuites（タスク3.4）
21. TestSuiteTree（タスク3.6）

### 第3優先: テストケース表示・編集
22. TestCase関連のDB実装（タスク2.2.1の一部）
23. Qase API Client（Test Cases API）（タスク2.2.2の一部）
24. TestCaseService（タスク2.3.1の一部）
25. Tauriコマンド（テストケース関連）（タスク2.4.1の一部）
26. Tauriサービス（テストケース関連）（タスク3.2の一部）
27. Zustandストア（テストケース）（タスク3.3の一部）
28. useTestCases（タスク3.4）
29. TestCaseEditor, StepEditor（タスク3.6）

### 第4優先: テスト・改善
30. 統合テスト（タスク4）

---

## 進捗管理

- [ ] 第1優先グループ完了
- [ ] 第2優先グループ完了
- [ ] 第3優先グループ完了
- [ ] 第4優先グループ完了

---

## メモ・課題

### 技術的な課題
- [ ] Qase API v1のレート制限への対応方法
- [ ] SQLiteのマイグレーション戦略
- [ ] Tauri IPCのエラーハンドリング

### UI/UXの課題
- [ ] ペインのリサイズ機能の必要性
- [ ] キーボードショートカットの優先度
- [ ] ローディング状態の表示方法

### 今後の検討事項
- [ ] オフライン動作の考慮
- [ ] データベースのバックアップ機能
- [ ] 複数プロジェクトの同時操作
