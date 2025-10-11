# Qasery Phase 2 準備チェックリスト

**目的**: Phase 2機能開発着手前の品質基準クリア  
**期限**: 3週間（15営業日）  
**最終確認日**: _________

---

## 📋 Critical Tasks（必須完了）

### ✅ タスク1: Value Objectの導入
- [ ] `ProjectCode` Value Objectの実装と単体テスト
- [ ] `TestSuiteId` Value Objectの実装と単体テスト
- [ ] `TestCaseId` Value Objectの実装と単体テスト  
- [ ] `SharedStepHash` Value Objectの実装と単体テスト
- [ ] `Severity` Value Objectの実装と単体テスト
- [ ] `Priority` Value Objectの実装と単体テスト
- [ ] ドメインモデル（models.rs）の更新
- [ ] Repositoryインターフェースの更新
- [ ] Serviceレイヤーの更新
- [ ] 既存テストの修正（全テストパス確認）
- [ ] **完了条件**: `cargo test` が100%パス、Value Object カバレッジ100%

---

### ✅ タスク2: Aggregate境界の明確化
- [ ] `TestCase` Aggregateの実装（steps を private化）
- [ ] `SharedStep` Aggregateの実装
- [ ] `TestCase::add_step()` メソッドの実装
- [ ] `TestCase::remove_step()` メソッドの実装
- [ ] Repository の `save_aggregate()` メソッド実装
- [ ] トランザクション境界のテスト（commit/rollback）
- [ ] **完了条件**: 部分的な状態更新が不可能、すべてAggregate root経由

---

### ✅ タスク3: 層の依存関係修正
- [ ] Presentation→Infrastructure 参照の全削除
- [ ] 必要に応じて Service にメソッド追加
- [ ] `TestStepService` の新規実装
- [ ] `get_case_steps` コマンドの修正
- [ ] 依存関係グラフの生成と確認（`cargo-depgraph`）
- [ ] **完了条件**: `grep -r "use crate::infrastructure" src-tauri/src/presentation/` が空

---

### ✅ タスク4: 構造化エラーハンドリング
- [ ] `src-tauri/src/domain/error.rs` の新規作成
- [ ] `DomainError` enum の実装
- [ ] `src-tauri/src/application/error.rs` の新規作成
- [ ] `ApplicationError` enum の実装
- [ ] `src-tauri/src/infrastructure/error.rs` の新規作成
- [ ] `InfrastructureError` enum の実装
- [ ] Presentation層の `ErrorResponse` 実装
- [ ] すべての `unwrap()` の削除
- [ ] すべての `expect()` の削除  
- [ ] すべての `unwrap_or()` の削除（特に services.rs:301）
- [ ] エラーハンドリングのテスト追加
- [ ] **完了条件**: `grep -r "\.unwrap\|\.expect" src-tauri/src/ | wc -l` が 0

---

### ✅ タスク5: APIトークンの安全な保存
- [ ] `keyring` crateの依存追加（Cargo.toml）
- [ ] `src-tauri/src/infrastructure/keychain.rs` の新規作成
- [ ] `KeychainService::save_token()` の実装
- [ ] `KeychainService::get_token()` の実装
- [ ] `KeychainService::delete_token()` の実装
- [ ] `AppState` からメモリ内 `api_token` の削除
- [ ] `AppState::set_api_token()` の更新
- [ ] `AppState::get_api_token()` の更新
- [ ] macOS での動作確認
- [ ] Windows での動作確認（可能であれば）
- [ ] Linux での動作確認（可能であれば）
- [ ] **完了条件**: 再起動後もトークンが永続化、平文メモリ保存なし

---

### ✅ タスク6: Qase API Mockの実装
- [ ] `mockito` の依存追加（Cargo.toml dev-dependencies）
- [ ] `test_get_projects_with_mock()` の実装
- [ ] `test_get_project_with_mock()` の実装
- [ ] `test_get_suites_with_mock()` の実装
- [ ] `test_get_suite_with_mock()` の実装
- [ ] `test_get_cases_with_mock()` の実装
- [ ] `test_get_case_with_mock()` の実装
- [ ] `test_get_shared_steps_with_mock()` の実装
- [ ] エラーケーステスト（401, 404, 500）の追加
- [ ] 既存の実APIテストの削除または無効化
- [ ] CI/CD設定（GitHub Actions等）
- [ ] **完了条件**: 実APIキーなしで `cargo test` 完全動作、30秒以内に完了

---

### ✅ タスク7: 統合テストの追加
- [ ] `src-tauri/tests/` ディレクトリ作成
- [ ] `tests/common/mod.rs` でテストヘルパー実装
- [ ] `setup_test_app()` ヘルパーの実装
- [ ] テストデータファクトリの実装
- [ ] プロジェクトライフサイクルの統合テスト
- [ ] テストスイート同期の統合テスト
- [ ] テストケース編集の統合テスト
- [ ] Shared Step同期の統合テスト
- [ ] エラーケースの統合テスト
- [ ] Given-When-Then形式への統一
- [ ] **完了条件**: 統合テスト10シナリオ以上、全テストパス

---

## 🎯 マイルストーン確認

### M1: Value Object & Aggregate（Week 1終了時）
- [ ] タスク1が100%完了
- [ ] タスク2が100%完了
- [ ] テストカバレッジ85%以上
- [ ] コードレビュー完了

### M2: アーキテクチャ改善（Week 2終了時）
- [ ] タスク3が100%完了
- [ ] タスク4が100%完了
- [ ] タスク5が100%完了
- [ ] コードレビュー完了

### M3: テスト基盤（Week 3終了時）
- [ ] タスク6が100%完了
- [ ] タスク7が100%完了
- [ ] CI/CDパイプライン動作確認
- [ ] 最終コードレビュー完了

---

## 🚀 Phase 2 着手可否判定

**すべての項目に✅がついていることを確認してください。**

### 必須条件
- [ ] すべてのCriticalタスク（1-7）が完了
- [ ] すべてのマイルストーン（M1-M3）が達成
- [ ] `cargo test` が100%パス（テスト数50以上）
- [ ] `cargo clippy` が警告なし
- [ ] ドキュメント更新完了

### 品質基準
- [ ] Value Objectで型安全性を保証
- [ ] Aggregate境界でトランザクション整合性を保証
- [ ] 層の依存関係が適切（Presentation→Infrastructure参照ゼロ）
- [ ] エラーが構造化され、デバッグが容易
- [ ] APIトークンが安全に保存（OS keychain使用）
- [ ] CI/CDで自動テストが動作（実APIキー不要）
- [ ] 統合テストで主要シナリオをカバー（70%以上）

---

## ⚠️ Phase 2着手前の最終確認

```bash
# 1. すべてのテストがパスするか確認
cd src-tauri
cargo test

# 2. Clippy警告がないか確認
cargo clippy -- -D warnings

# 3. unwrap/expectが残っていないか確認
grep -r "\.unwrap\|\.expect" src/ | grep -v "test" | wc -l
# ↑ この結果が0であること

# 4. Presentation→Infrastructureの依存がないか確認
grep -r "use crate::infrastructure" src/presentation/
# ↑ この結果が空であること

# 5. ドキュメント生成確認
cargo doc --no-deps --open

# 6. フォーマット確認
cargo fmt -- --check

# 7. ビルド確認
cargo build --release
```

---

## 📊 進捗トラッキング

| タスク | 着手日 | 完了日 | 担当者 | ステータス |
|--------|--------|--------|--------|-----------|
| タスク1: Value Object | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク2: Aggregate | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク3: 依存関係 | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク4: エラー処理 | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク5: Keychain | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク6: API Mock | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |
| タスク7: 統合テスト | ______ | ______ | ______ | ⬜ 未着手 / 🔄 進行中 / ✅ 完了 |

---

## 📝 レビューア署名

### Week 1レビュー（M1達成確認）
- [ ] Value Objectの実装を確認
- [ ] Aggregateの実装を確認
- [ ] テストカバレッジを確認
- **レビューア**: ____________  **日付**: __________

### Week 2レビュー（M2達成確認）
- [ ] 層の依存関係を確認
- [ ] エラーハンドリングを確認
- [ ] Keychainの動作を確認
- **レビューア**: ____________  **日付**: __________

### Week 3レビュー（M3達成確認）
- [ ] API Mockの動作を確認
- [ ] 統合テストを確認
- [ ] CI/CD動作を確認
- **レビューア**: ____________  **日付**: __________

### 最終承認（Phase 2着手許可）
- [ ] すべてのCriticalタスクが完了
- [ ] すべての品質基準をクリア
- [ ] ドキュメント更新完了
- **承認者**: ____________  **日付**: __________

---

**Phase 2着手許可が下りるまで、新機能開発には着手しないでください。**
**テクニカルデットの返済が最優先です。**
