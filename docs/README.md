# Qasery ドキュメント

Qaseryプロジェクトの技術ドキュメント、設計資料、レビュー報告書を管理するディレクトリです。

---

## 📁 ディレクトリ構成

```
docs/
├── README.md                    # このファイル（ドキュメントの目次）
├── architecture/                # アーキテクチャ・設計資料
│   └── specification.md         # Qaseryの全体仕様書
├── api-reference/               # API リファレンス
│   └── qase-api-reference.md    # Qase API の仕様・エンドポイント情報
├── phase1/                      # Phase 1 関連ドキュメント
│   └── phase1-development.md    # Phase 1 開発計画・実装記録
├── phase2/                      # Phase 2 関連ドキュメント
│   ├── action-plan.md           # Phase 2 アクションプラン（実装ガイド付き）
│   └── checklist.md             # Phase 2 進捗管理チェックリスト
└── reviews/                     # コードレビュー報告書
    ├── README.md                # レビュー資料の読み方ガイド
    ├── EXECUTIVE_SUMMARY.md     # エグゼクティブサマリー（必読）
    └── code-review-report.md    # 詳細レビュー報告書
```

---

## 🚀 クイックスタート

### 新しくプロジェクトに参加した方

1. **まず読む**: [`architecture/specification.md`](./architecture/specification.md)
   - Qaseryの全体像、技術スタック、アーキテクチャを理解

2. **次に読む**: [`reviews/README.md`](./reviews/README.md)
   - 現在の品質状況と次にやるべきことを把握

### Phase 2 開発に着手する方

1. **必読**: [`reviews/EXECUTIVE_SUMMARY.md`](./reviews/EXECUTIVE_SUMMARY.md)
   - Phase 2 着手前の必須対応を確認

2. **実装時の参照**: [`phase2/action-plan.md`](./phase2/action-plan.md)
   - 具体的な実装方法をコード例付きで確認

3. **日常の進捗管理**: [`phase2/checklist.md`](./phase2/checklist.md)
   - タスク完了状況を毎日チェック

### コードレビューの内容を詳しく知りたい方

1. **サマリー**: [`reviews/EXECUTIVE_SUMMARY.md`](./reviews/EXECUTIVE_SUMMARY.md) - 5分
2. **詳細**: [`reviews/code-review-report.md`](./reviews/code-review-report.md) - 60分

---

## 📚 各ディレクトリの詳細

### 🏗️ architecture/

**目的**: システムの設計思想とアーキテクチャを文書化

**対象読者**: 全員（特に新規参加者）

**ファイル**:
- `specification.md`: Qasery全体仕様書
  - 技術スタック（Rust/Tauri/React/TypeScript）
  - Clean Architectureの4層構造
  - ディレクトリ構成
  - ドメインモデル
  - 機能要件

---

### 🔌 api-reference/

**目的**: 外部APIの仕様を参照資料として保存

**対象読者**: バックエンド開発者

**ファイル**:
- `qase-api-reference.md`: Qase APIの仕様
  - エンドポイント一覧
  - リクエスト/レスポンス形式
  - 認証方法
  - レート制限

---

### 1️⃣ phase1/

**目的**: Phase 1の開発計画と実装記録

**対象読者**: Phase 1実装者、後続フェーズの参考として

**ファイル**:
- `phase1-development.md`: Phase 1開発ドキュメント
  - 実装スコープ
  - タスク分解
  - 実装メモ

---

### 2️⃣ phase2/

**目的**: Phase 2の品質改善タスクを管理

**対象読者**: Phase 2開発者全員

**ファイル**:
- `action-plan.md`: 実装ガイド付きアクションプラン
  - 7つのCriticalタスクの詳細
  - 実装コード例
  - テスト例
  - スケジュール（3週間）

- `checklist.md`: 進捗管理チェックリスト
  - タスクごとのチェック項目
  - マイルストーン確認（Week 1-3）
  - Phase 2着手可否判定

**使い方**:
1. 毎朝、チームで`checklist.md`を開いて進捗確認
2. タスク着手時に`action-plan.md`の該当セクションを熟読
3. 週末にマイルストーン達成状況をレビュー

---

### 🔍 reviews/

**目的**: Phase 1完了時点のコードレビュー結果を保存

**対象読者**: 全員（特にPhase 2着手前に必読）

**ファイル**:
- `README.md`: レビュー資料の読み方ガイド
  - 各資料の対象読者と所要時間
  - 推奨される読む順番
  - よくある質問

- `EXECUTIVE_SUMMARY.md`: エグゼクティブサマリー（5分）
  - 総合評価: C+
  - 6つの重大な問題
  - 7つのCriticalタスク
  - ビジネスインパクト
  - 推奨アクション

- `code-review-report.md`: 詳細レビュー報告書（60分）
  - 7つの観点からの詳細分析
  - 良好な点・重大な問題・改善点
  - 具体的なコード例と推奨実装

**重要**: Phase 2の機能開発に進む前に、**必ず**レビュー資料を読んでください。

---

## 🎯 状況別・推奨読書パス

### 📖 ケース1: 新規参加者（初日）
```
1. docs/README.md（このファイル）           - 5分
2. architecture/specification.md          - 30分
3. reviews/EXECUTIVE_SUMMARY.md           - 5分
4. phase2/checklist.md（現在の進捗確認）    - 5分
```

### 🛠️ ケース2: Phase 2タスクを実装する開発者
```
1. reviews/EXECUTIVE_SUMMARY.md           - 5分
2. phase2/action-plan.md（担当タスク部分） - 30分
3. phase2/checklist.md（チェック項目確認）  - 5分
4. reviews/code-review-report.md（詳細理解）- 必要に応じて
```

### 🎓 ケース3: 技術リーダー・レビューア
```
1. reviews/EXECUTIVE_SUMMARY.md           - 5分
2. reviews/code-review-report.md          - 60分
3. phase2/action-plan.md                  - 30分
4. architecture/specification.md          - 30分
5. phase2/checklist.md（レビュー観点確認）  - 5分
```

### 💼 ケース4: PM・意思決定者
```
1. reviews/EXECUTIVE_SUMMARY.md                    - 5分
2. reviews/code-review-report.md（ビジネス影響部分）- 10分
3. phase2/checklist.md（マイルストーン部分）         - 5分
```

---

## 📅 ドキュメント更新履歴

| 日付 | 更新内容 | 担当者 |
|------|----------|--------|
| 2025-10-11 | Phase 1コードレビュー実施、reviews/配下を作成 | Code Review AI |
| 2025-10-11 | ディレクトリ構造の整理、README作成 | Code Review AI |
| 2025-10-10 | Phase 1開発計画を作成 | - |
| 2025-10-08 | 全体仕様書（specification.md）を作成 | - |

---

## 🔄 ドキュメント管理ルール

### 新しいドキュメントを追加する時

1. **適切なディレクトリを選択**
   - アーキテクチャ設計 → `architecture/`
   - Phase固有の資料 → `phase{N}/`
   - レビュー・評価 → `reviews/`
   - API仕様 → `api-reference/`

2. **このREADMEを更新**
   - 新規ファイルの説明を追加
   - 更新履歴に記録

3. **命名規則**
   - ケバブケース（例: `action-plan.md`）
   - 英語ファイル名（内容は日本語OK）
   - 日付を含める場合: `YYYY-MM-DD-` prefix

### Phase が進むとき

- 新しい`phase{N}/`ディレクトリを作成
- 該当Phaseの計画・実装記録・レビューをまとめる
- このREADMEに新Phaseのセクションを追加

---

## ❓ よくある質問

### Q: ドキュメントが多すぎて何から読めばいいかわからない
**A**: 立場に応じて以下を推奨します：
- **開発者**: `reviews/EXECUTIVE_SUMMARY.md` → `phase2/action-plan.md`
- **新規参加**: `architecture/specification.md` → `reviews/README.md`
- **PM**: `reviews/EXECUTIVE_SUMMARY.md` のみ

### Q: コードと資料の内容が矛盾している場合は？
**A**: コードが正。資料をIssueで報告し、更新を依頼してください。

### Q: 英語と日本語、どちらで書くべき？
**A**: プロジェクト規約に従い、**日本語推奨**。技術用語は英語OK。

---

## 📞 サポート

- **ドキュメントの内容に関する質問**: 開発チームリード
- **資料の更新・追加依頼**: GitHub Issue
- **アーキテクチャ判断の相談**: 技術リーダー

---

**最終更新**: 2025-10-11  
**管理者**: Qasery開発チーム
