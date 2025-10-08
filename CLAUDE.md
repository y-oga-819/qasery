# Claude開発ガイド

このドキュメントは、Claude Codeを使ってQaseryを開発する際のルールと手順を定めたものです。

## 開発の基本ルール

### 1. ブランチ戦略

#### ブランチ命名規則
```
feature/<task-name>  # 新機能追加
fix/<bug-name>       # バグ修正
refactor/<target>    # リファクタリング
test/<target>        # テスト追加
docs/<target>        # ドキュメント更新
```

#### ブランチ運用ルール
- **必須**: タスク開始時に必ず新しいブランチを作成
- mainブランチでは直接作業しない
- 1ブランチ = 1タスク（小さく分割）
- タスク完了後はPRを作成してマージ

#### ブランチ作成手順
```bash
# 1. mainブランチの最新を取得
git checkout main
git pull origin main

# 2. 新しいブランチを作成
git checkout -b feature/project-repository

# 3. 作業開始
```

### 2. TDD（テスト駆動開発）の手法

#### t_wadaのTDDサイクル
```
Red → Green → Refactor → Commit
```

#### 具体的な手順

##### 1. Red（失敗するテストを書く）
```rust
// tests/project_repository_test.rs
#[test]
fn test_get_all_projects() {
    let repo = ProjectRepository::new();
    let projects = repo.get_all();
    assert_eq!(projects.len(), 0); // まず失敗するテスト
}
```
- **実行**: テストが失敗することを確認
- **Commit**: `test: add failing test for get_all_projects`

##### 2. Green（テストを通す最小限のコードを書く）
```rust
// src/domain/repository.rs
impl ProjectRepository {
    pub fn get_all(&self) -> Vec<Project> {
        Vec::new() // 最小限の実装
    }
}
```
- **実行**: テストが通ることを確認
- **Commit**: `feat: implement get_all_projects (minimal)`

##### 3. Refactor（リファクタリング）
```rust
// より良い実装に改善
impl ProjectRepository {
    pub fn get_all(&self) -> Result<Vec<Project>, Error> {
        self.db.query("SELECT * FROM projects")
    }
}
```
- **実行**: テストが通ることを確認
- **Commit**: `refactor: improve get_all_projects implementation`

##### 4. 次のテストケースへ
- 新しいテストケースを追加して1に戻る
- 小さいステップで進める

#### TDDの原則
1. **テストファースト**: 実装より先にテストを書く
2. **小さいステップ**: 一度に1つの機能だけ実装
3. **リファクタリング**: テストが通ったら改善
4. **継続的な実行**: テストを頻繁に実行

### 3. コミット戦略

#### コミットの粒度
- **超細かく**: 1つの変更 = 1コミット
- Red/Green/Refactorそれぞれでコミット
- ファイル追加だけでもコミット
- 設定変更だけでもコミット

#### コミットメッセージ規則
```
<type>: <subject>

<body>（オプション）
```

##### Type一覧
- `test`: テストコード追加・変更
- `feat`: 新機能実装
- `fix`: バグ修正
- `refactor`: リファクタリング
- `docs`: ドキュメント更新
- `style`: コードフォーマット
- `chore`: ビルド・設定変更

##### 例
```bash
# Red
git commit -m "test: add failing test for project creation"

# Green
git commit -m "feat: implement basic project creation"

# Refactor
git commit -m "refactor: extract validation logic"

# その他
git commit -m "chore: add rusqlite dependency"
git commit -m "docs: update README with setup instructions"
```

#### コミット前のチェック
- [ ] テストが通っているか
- [ ] コンパイルエラーがないか
- [ ] コミットメッセージは明確か

---

## 開発フロー（実践例）

### Phase 1のタスク例: ProjectRepositoryの実装

#### Step 1: ブランチ作成
```bash
git checkout main
git pull origin main
git checkout -b feature/project-repository
```

#### Step 2: テストファイル作成
```bash
# ファイル作成
touch backend/src/domain/repository.rs
touch backend/tests/project_repository_test.rs
```
```bash
git add .
git commit -m "chore: add files for project repository"
```

#### Step 3: TDDサイクル開始

##### Cycle 1: get_all()の実装

**Red**
```rust
// tests/project_repository_test.rs
#[test]
fn test_get_all_returns_empty_list() {
    let repo = ProjectRepository::new();
    let result = repo.get_all();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
```
```bash
cargo test  # 失敗を確認
git add tests/project_repository_test.rs
git commit -m "test: add failing test for get_all returns empty list"
```

**Green**
```rust
// src/domain/repository.rs
pub struct ProjectRepository;

impl ProjectRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn get_all(&self) -> Result<Vec<Project>, Error> {
        Ok(Vec::new())
    }
}
```
```bash
cargo test  # 成功を確認
git add src/domain/repository.rs
git commit -m "feat: implement get_all to return empty list"
```

**Refactor**（必要に応じて）
```rust
// リファクタリングが必要な場合
pub fn get_all(&self) -> Result<Vec<Project>, Error> {
    self.fetch_all_from_db()
}
```
```bash
cargo test  # 成功を確認
git add src/domain/repository.rs
git commit -m "refactor: extract get_all logic to fetch_all_from_db"
```

##### Cycle 2: get_by_code()の実装

**Red**
```rust
#[test]
fn test_get_by_code_returns_project() {
    let repo = ProjectRepository::new();
    let result = repo.get_by_code("DEMO");
    assert!(result.is_ok());
}
```
```bash
cargo test  # 失敗を確認
git commit -am "test: add failing test for get_by_code"
```

**Green → Refactor → 繰り返し**

#### Step 4: PR作成
```bash
# すべてのテストが通ることを確認
cargo test

# mainブランチにプッシュ
git push origin feature/project-repository

# PRを作成（GitHub CLI使用）
gh pr create --title "feat: implement ProjectRepository" \
  --body "- Implement get_all()
- Implement get_by_code()
- Implement save()
- Implement delete()

Closes #1"
```

---

## タスク実行時のチェックリスト

### 開始時
- [ ] mainブランチから最新を取得
- [ ] 新しいブランチを作成
- [ ] タスクの目的を理解

### 開発中
- [ ] Red → Green → Refactorサイクルを守る
- [ ] テストファーストで書く
- [ ] 小さいステップで進める
- [ ] こまめにコミット
- [ ] テストを頻繁に実行

### 完了時
- [ ] すべてのテストが通る
- [ ] コンパイルエラーがない
- [ ] コミットメッセージが適切
- [ ] PRを作成
- [ ] マージ後、ブランチを削除

---

## 推奨する開発環境設定

### Rust
```bash
# テスト自動実行
cargo install cargo-watch
cargo watch -x test

# フォーマッター
rustfmt --check src/**/*.rs

# Linter
cargo clippy
```

### Git
```bash
# コミット前の自動チェック（推奨）
# .git/hooks/pre-commit
#!/bin/sh
cargo test
cargo clippy
```

---

## トラブルシューティング

### テストが失敗する
1. エラーメッセージを読む
2. 最小限のコードで通るようにする
3. 段階的に実装を追加

### コミットを間違えた
```bash
# 直前のコミットを修正
git commit --amend

# コミットを取り消し（変更は残す）
git reset --soft HEAD^
```

### ブランチを間違えた
```bash
# 現在の変更を退避
git stash

# 正しいブランチに切り替え
git checkout correct-branch

# 変更を復元
git stash pop
```

---

## 参考資料

- [t_wadaさんのTDD](https://speakerdeck.com/twada/live-coding-with-tdd)
- [Red-Green-Refactor](https://www.jamesshore.com/v2/blog/2005/red-green-refactor)
- [Conventional Commits](https://www.conventionalcommits.org/)
