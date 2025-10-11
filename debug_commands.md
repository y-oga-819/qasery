# Shared Step デバッグ手順

## 1. ブラウザのコンソールを開く

アプリケーションで以下を確認してください:

1. プロジェクトを選択
2. テストスイートを選択
3. Shared Stepを含むテストケースを選択
4. ブラウザのコンソール（Developer Tools）を開く

## 2. コンソールログを確認

以下のようなログが表示されるはずです:

```
[useSharedSteps] Fetching shared steps for project: <プロジェクトコード>
[useSharedSteps] Found X shared steps in DB
[useSharedSteps] Sample shared steps (first 3): [...]
[useSharedSteps] Created map with X entries
[TestCaseViewer] Shared Step Group 0: { hash: "...", foundInMap: true/false, title: "...", ... }
```

## 3. データベースを直接確認（オプション）

ブラウザのコンソールで以下を実行:

```javascript
// すべてのShared Stepの数とサンプルを取得
const { invoke } = window.__TAURI__.core;
await invoke('debug_shared_step', {});

// 特定のShared Stepを取得
await invoke('debug_shared_step', { hash: '<ハッシュ値>' });
```

## 4. 期待される結果

- `foundInMap: true` となっているか
- `title` フィールドに正しいタイトルが入っているか（例: "【メール】オプション機能利用料のインボイスメールを確認する"）
- `displayName` に `title` の値が表示されているか

## 問題の診断

### パターン1: mapSize が 0
→ Shared Stepが同期されていない
→ 同期ボタンを押して再取得

### パターン2: foundInMap が false
→ hash値が一致していない
→ test_case_steps.shared_step_hash と shared_steps.hash を確認

### パターン3: title が空または null
→ APIからのデータ取得に問題がある
→ Qase APIのレスポンスを確認

### パターン4: displayName が action フィールドの値
→ title がnullで fallback している
→ データベースの title カラムを確認
