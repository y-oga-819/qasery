# ドキュメント命名規則

**策定日**: 2025-10-11  
**適用範囲**: `docs/` 配下のすべてのMarkdownファイル

---

## 📋 命名規則

### 基本原則

1. **ケバブケース（kebab-case）を使用**
   - すべて小文字
   - 単語の区切りはハイフン（`-`）
   - アンダースコア（`_`）は使用しない
   
2. **英語のファイル名を使用**
   - 内容は日本語でOK
   - ファイル名は英語で統一
   
3. **意味のある名前**
   - ファイルの内容が一目でわかる名前
   - 省略形は一般的なもののみ使用（例: `spec`, `doc`, `ref`）

4. **特殊ファイルの例外**
   - `README.md` - 各ディレクトリのインデックス（大文字OK）
   - `CHANGELOG.md`, `LICENSE.md` - 慣習的に大文字（該当する場合）

---

## 🎯 命名パターン

### パターン1: レポート・サマリー系
```
{type}-{subject}.md

例:
- review-summary.md          # レビューサマリー
- review-report.md           # レビュー詳細報告書
- action-plan.md             # アクションプラン
- checklist.md               # チェックリスト
- reorganization-report.md   # 整理報告書
```

### パターン2: Phase固有ドキュメント
```
{phase-name}-{document-type}.md

例:
- phase1-development.md      # Phase 1開発ドキュメント
- phase1-summary.md          # Phase 1サマリー
- phase2-action-plan.md      # Phase 2アクションプラン
- phase2-checklist.md        # Phase 2チェックリスト
```

### パターン3: 仕様・参照系
```
{subject}-{type}.md

例:
- system-specification.md    # システム仕様書
- architecture-design.md     # アーキテクチャ設計書
- api-reference.md           # API リファレンス
- qase-api-reference.md      # Qase API リファレンス（固有名詞含む）
```

### パターン4: ディレクトリインデックス
```
README.md (大文字固定)

各ディレクトリの説明・ナビゲーション
```

---

## 📝 具体的な命名例

### ✅ 良い例
```
✓ review-summary.md
✓ action-plan.md
✓ phase1-development.md
✓ api-reference.md
✓ system-specification.md
✓ README.md
```

### ❌ 悪い例
```
✗ ReviewSummary.md          # キャメルケース
✗ review_summary.md         # アンダースコア
✗ REVIEW-SUMMARY.md         # 全て大文字
✗ rev-sum.md                # 過度な省略
✗ review summary.md         # スペース
✗ レビューサマリー.md         # 日本語ファイル名
```

---

## 🔄 既存ファイルのリネーム対応

### 現状の問題
| 現在のファイル名 | 問題点 |
|-----------------|--------|
| `EXECUTIVE_SUMMARY.md` | 全て大文字、アンダースコア |
| `REORGANIZATION_REPORT.md` | 全て大文字、アンダースコア |
| `code-review-report.md` | OK（そのまま） |
| `specification.md` | OK（そのまま） |
| `qase-api-reference.md` | OK（そのまま） |
| `phase1-development.md` | OK（そのまま） |
| `phase2-action-plan.md` | OK（そのまま） |
| `phase2-checklist.md` | OK（そのまま） |

### リネーム計画

#### reviews/ ディレクトリ
```bash
# 変更前 → 変更後
EXECUTIVE_SUMMARY.md     → review-summary.md
code-review-report.md    → review-report.md (統一のため)
README.md                → README.md (変更なし)
```

#### docs/ ルート
```bash
# 変更前 → 変更後
REORGANIZATION_REPORT.md → reorganization-report.md
specification.md         → system-specification.md (明確化)
qase-api-reference.md    → api-reference-qase.md (統一)
README.md                → README.md (変更なし)
```

#### phase1/ ディレクトリ
```bash
# 変更前 → 変更後
phase1-development.md    → phase1-development.md (変更なし)
```

#### phase2/ ディレクトリ
```bash
# 変更前 → 変更後
phase2-action-plan.md    → phase2-action-plan.md (変更なし)
phase2-checklist.md      → phase2-checklist.md (変更なし)
README.md                → README.md (変更なし)
```

---

## 📚 リネーム後のファイル一覧

```
docs/
├── README.md
├── reorganization-report.md
├── system-specification.md
├── api-reference-qase.md
├── phase1/
│   └── phase1-development.md
├── phase2/
│   ├── README.md
│   ├── phase2-action-plan.md
│   └── phase2-checklist.md
└── reviews/
    ├── README.md
    ├── review-summary.md
    └── review-report.md
```

---

## 🎯 新規ファイル作成時のガイドライン

### ステップ1: ファイルの種類を特定
- レポート・サマリー系 → `{type}-{subject}.md`
- Phase固有 → `{phase-name}-{document-type}.md`
- 仕様・参照系 → `{subject}-{type}.md`
- ディレクトリインデックス → `README.md`

### ステップ2: 命名規則に従う
- すべて小文字
- ハイフンで単語を区切る
- 意味のある英語の名前

### ステップ3: レビュー
- 既存ファイルと命名が一貫しているか確認
- 長すぎないか確認（50文字以内推奨）

---

## 🔍 命名規則チェックリスト

新規ファイル作成時、以下を確認：

- [ ] すべて小文字である
- [ ] ハイフン（`-`）のみを使用（アンダースコア不使用）
- [ ] 英語のファイル名である
- [ ] ファイルの内容が一目でわかる
- [ ] 既存ファイルと命名パターンが一貫している
- [ ] 50文字以内である
- [ ] スペースや特殊文字を含まない

---

## 📅 更新履歴

| 日付 | 変更内容 |
|------|----------|
| 2025-10-11 | 初版策定、既存ファイルのリネーム実施 |

---

**管理者**: Qasery開発チーム
