# Phase 2 品質改善タスク

**期間**: 3週間（15営業日）  
**目的**: Phase 2機能開発前の必須品質改善  
**ステータス**: 🔄 進行中 / ⬜ 未着手 / ✅ 完了

---

## 📋 このディレクトリの資料

### 1. アクションプラン [`action-plan.md`](./action-plan.md)
**対象読者**: 開発者  
**所要時間**: 読むのは30分、実施は3週間

7つのCriticalタスクの具体的な実装方法を、サンプルコード付きで解説。

**内容**:
- タスク1: Value Objectの導入
- タスク2: Aggregate境界の明確化
- タスク3: 層の依存関係修正
- タスク4: 構造化エラーハンドリング
- タスク5: APIトークンの安全な保存
- タスク6: Qase API Mockの実装
- タスク7: 統合テストの追加
- スケジュール、リスク管理

**使い方**: タスク着手時に該当セクションを熟読し、コード例を参考に実装。

---

### 2. チェックリスト [`checklist.md`](./checklist.md)
**対象読者**: 開発チーム全員、PM  
**所要時間**: 毎日5分

Phase 2着手前の必須タスクを管理する日常チェックリスト。

**内容**:
- 7つのCriticalタスクの詳細チェック項目
- マイルストーン確認（Week 1-3）
- Phase 2着手可否判定基準
- 進捗トラッキングテーブル
- レビューア署名欄

**使い方**:
1. 毎朝チームミーティングで前日の進捗を確認
2. 完了した項目に✅を記入
3. 週末にマイルストーン達成状況をレビュー

---

## 🎯 Phase 2の目標

### ミッション
**Phase 2機能開発前に、ミッションクリティカルな品質基準を達成する**

### 成功基準

| メトリクス | 現状 (Phase 1終了時) | 目標 (Phase 2開始前) |
|-----------|---------------------|---------------------|
| テスト数 | 25 | 50+ |
| テストカバレッジ | 60% | 85%+ |
| 統合テスト | 0 | 10+ |
| Clippy警告 | 50+ | 0 |
| `unwrap()`の数 | 20+ | 0 |
| 層違反の数 | 5+ | 0 |
| セキュリティリスク | Critical | Low |

### 完了条件（すべて必須）

- [ ] すべてのCriticalタスク（1-7）が完了
- [ ] `cargo test` が100%パス
- [ ] `cargo clippy -- -D warnings` がパス
- [ ] Value Objectで型安全性を保証
- [ ] Aggregate境界でトランザクション整合性を保証
- [ ] 層の依存関係が適切（Presentation→Infrastructure参照ゼロ）
- [ ] エラーが構造化され、デバッグが容易
- [ ] APIトークンが安全に保存（OS keychain使用）
- [ ] CI/CDで自動テストが動作（実APIキー不要）
- [ ] 統合テストで主要シナリオをカバー（70%以上）

---

## 📅 スケジュール

### Week 1: Value Object & Aggregate
**マイルストーン M1**

- [ ] タスク1: Value Objectの導入（3日）
- [ ] タスク2: Aggregate境界の明確化（2日）
- [ ] M1レビュー: テストカバレッジ85%以上

### Week 2: アーキテクチャ改善
**マイルストーン M2**

- [ ] タスク3: 層の依存関係修正（1日）
- [ ] タスク4: 構造化エラーハンドリング（2日）
- [ ] タスク5: APIトークンの安全な保存（1日）
- [ ] M2レビュー: すべての層違反が解消

### Week 3: テスト基盤
**マイルストーン M3**

- [ ] タスク6: Qase API Mockの実装（2日）
- [ ] タスク7: 統合テストの追加（2日）
- [ ] バッファ・最終レビュー（1日）
- [ ] M3レビュー: CI/CDが完全動作

---

## 🚀 クイックスタート

### 今日やること（Phase 2 Day 1）

1. ✅ チーム全体で[`../reviews/EXECUTIVE_SUMMARY.md`](../reviews/EXECUTIVE_SUMMARY.md)を読み合わせ
2. ✅ Phase 2機能開発の一時停止を正式決定
3. ✅ [`action-plan.md`](./action-plan.md)のタスク1-7をGitHub Issuesに登録
4. ✅ 各タスクに担当者をアサイン
5. ✅ 週次レビューミーティングを設定（毎週金曜17:00）

### 今週やること（Week 1）

1. ✅ タスク1（Value Object導入）に着手
2. ✅ 毎朝9:00にスタンドアップ、[`checklist.md`](./checklist.md)で進捗確認
3. ✅ 金曜17:00に週次レビュー、M1達成状況を確認

### 3週間後のゴール

- ✅ すべてのCriticalタスクが完了
- ✅ [`checklist.md`](./checklist.md)のすべての項目にチェック
- ✅ Phase 2機能開発の着手許可を取得

---

## 🛠️ 開発環境セットアップ

Phase 2タスクを実施する前に、以下を確認：

```bash
# Rustツールチェーンの更新
rustup update stable

# 開発用ツールのインストール
cargo install cargo-watch
cargo install cargo-depgraph  # 依存関係グラフ生成用

# フォーマッタ・リンタの確認
cargo fmt --version
cargo clippy --version

# テスト実行の確認
cd src-tauri
cargo test
```

---

## 📊 進捗確認コマンド

### 毎日実行（開発中）

```bash
# すべてのテストを実行
cargo test

# Clippy警告を確認
cargo clippy

# unwrap/expectの残数を確認
grep -r "\.unwrap\|\.expect" src/ | grep -v "test" | wc -l

# Presentation→Infrastructureの依存を確認
grep -r "use crate::infrastructure" src/presentation/
```

### 週次実行（マイルストーンレビュー時）

```bash
# テストカバレッジを計算（tarpaulin使用）
cargo tarpaulin --out Html

# 依存関係グラフを生成
cargo depgraph | dot -Tpng > dependency-graph.png

# ドキュメント生成
cargo doc --no-deps --open
```

---

## ⚠️ よくある問題と対処法

### 問題1: Value Object導入時に既存テストが大量に壊れる
**対処法**: 型エイリアスを一時的に使用し、段階的に移行
```rust
// 一時的な型エイリアス（後で削除）
pub type ProjectCode = String;
```

### 問題2: Keychain APIがLinuxで動作しない
**対処法**: `keyring` crateはLinuxの`Secret Service API`に対応。`libsecret`をインストール
```bash
sudo apt-get install libsecret-1-dev
```

### 問題3: Mockitoのテストが不安定
**対処法**: テストごとに異なるポートを使用、または`--test-threads=1`でシリアル実行
```bash
cargo test -- --test-threads=1
```

---

## 📞 サポート

### ブロッカーが発生した場合

1. **即座にチームに共有**（Slackなど）
2. **Issue化**してラベル`blocker`を付与
3. **デイリースタンドアップ**で優先議論
4. **必要に応じてペアプログラミング**で解決

### 質問・相談先

- **技術的な実装**: 開発チームリード
- **スケジュール調整**: PM
- **アーキテクチャ判断**: 技術リーダー
- **ドキュメント不明点**: [`action-plan.md`](./action-plan.md)の該当セクションを再読

---

## 📚 参考資料

- [Value Objectパターン（Martin Fowler）](https://martinfowler.com/bliki/ValueObject.html)
- [Aggregateパターン（DDD）](https://martinfowler.com/bliki/DDD_Aggregate.html)
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Tauri Documentation](https://tauri.app/v1/guides/)

---

**最終更新**: 2025-10-11  
**次のレビュー**: Week 1終了時（マイルストーンM1）
