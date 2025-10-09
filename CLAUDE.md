# Claude開発ガイド

このドキュメントは、Claude Codeを使ってQaseryを開発する際のルールと手順を定めたものです。

## 開発の基本ルール

### 1. コーディング規約

#### コメントは日本語で記述
- **必須**: すべてのコメント（行コメント、ドキュメントコメント）は日本語で記述する
- コード自体は英語（変数名、関数名、型名など）
- READMEなどのドキュメントも日本語

**良い例:**
```rust
/// ユーザー情報を取得する
///
/// # Arguments
/// * `user_id` - ユーザーID
///
/// # Returns
/// ユーザー情報またはエラー
pub fn get_user(user_id: i64) -> Result<User, Error> {
    // データベースから取得
    let user = db.query_user(user_id)?;
    Ok(user)
}
```

**悪い例:**
```rust
/// Get user information
pub fn get_user(user_id: i64) -> Result<User, Error> {
    // Fetch from database
    let user = db.query_user(user_id)?;
    Ok(user)
}
```

### 2. ブランチ戦略

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

### 3. TDD（テスト駆動開発）の手法

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

#### ⚠️ TDDでよくある失敗パターン

##### 失敗例: 一度のGreenフェーズで完全実装してしまう

**❌ 悪い例（Qase API Clientでの実際の失敗）:**
```rust
// Red: get_projects()が存在しないテスト
#[tokio::test]
async fn test_get_projects() {
    let client = QaseClient::new("token").unwrap();
    let result = client.get_projects(None, None).await;
    assert!(result.is_ok());
}

// Green: いきなり完全な実装を書いてしまう
pub async fn get_projects(&self, limit: Option<i32>, offset: Option<i32>)
    -> Result<QaseResponse<ProjectsResult>> {
    let client = reqwest::Client::new();
    let url = format!("{}/project", self.base_url);
    let mut query_params = vec![];
    if let Some(l) = limit {
        query_params.push(("limit", l.to_string()));
    }
    if let Some(o) = offset {
        query_params.push(("offset", o.to_string()));
    }
    let response = client.get(&url)
        .header("Token", &self.api_token)
        .query(&query_params)
        .send()
        .await?;
    let result: QaseResponse<ProjectsResult> = response.json().await?;
    Ok(result)
}
```

**問題点:**
- 一度に多くの機能を実装（HTTP通信、パラメータ処理、JSONパース）
- テストが意味のないアサーション（`assert!(result.is_ok())`）になっている
- デバッグが困難（どこで問題が起きているか分からない）
- TDDの「小さいステップ」という原則に反している

##### ✅ 正しい例: 段階的に実装する

**Step 1: まず固定値を返す（最小限の実装）**
```rust
// Red: 固定値を期待するテスト
#[test]
fn test_get_projects_returns_response() {
    let client = QaseClient::new("token").unwrap();
    let result = client.get_projects_sync(); // 同期版で開始

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, true);
    assert_eq!(response.result.total, 0);
}

// Green: 固定値を返す
pub fn get_projects_sync(&self) -> Result<QaseResponse<ProjectsResult>> {
    Ok(QaseResponse {
        status: true,
        result: ProjectsResult {
            total: 0,
            filtered: 0,
            count: 0,
            entities: vec![],
        }
    })
}
```
```bash
cargo test
git commit -m "feat: get_projectsの固定値実装（Green）"
```

**Step 2: 非同期化する**
```rust
// Red: 非同期版のテスト
#[tokio::test]
async fn test_get_projects_async() {
    let client = QaseClient::new("token").unwrap();
    let result = client.get_projects().await; // asyncに変更

    assert!(result.is_ok());
}

// Green: async化
pub async fn get_projects(&self) -> Result<QaseResponse<ProjectsResult>> {
    Ok(QaseResponse {
        status: true,
        result: ProjectsResult {
            total: 0,
            filtered: 0,
            count: 0,
            entities: vec![],
        }
    })
}
```
```bash
cargo test
git commit -m "feat: get_projectsを非同期化（Green）"
```

**Step 3: モックサーバーを使ったHTTP通信**
```rust
// Red: モックサーバーでHTTP通信をテスト
#[tokio::test]
async fn test_get_projects_makes_http_request() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/project"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": true,
            "result": {
                "total": 1,
                "filtered": 1,
                "count": 1,
                "entities": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = QaseClient::new_with_base_url("token", &mock_server.uri());
    let result = client.get_projects().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.result.total, 1);
}

// Green: 実際のHTTP通信を実装
pub async fn get_projects(&self) -> Result<QaseResponse<ProjectsResult>> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/project", self.base_url))
        .header("Token", &self.api_token)
        .send()
        .await?;
    let result: QaseResponse<ProjectsResult> = response.json().await?;
    Ok(result)
}
```
```bash
cargo test
git commit -m "feat: get_projectsでHTTP通信を実装（Green）"
```

**Step 4: limitパラメータを追加**
```rust
// Red: limitパラメータのテスト
#[tokio::test]
async fn test_get_projects_with_limit() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/project"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": true,
            "result": {
                "total": 1,
                "filtered": 1,
                "count": 1,
                "entities": []
            }
        })))
        .mount(&mock_server)
        .await;

    let client = QaseClient::new_with_base_url("token", &mock_server.uri());
    let result = client.get_projects(Some(10), None).await;

    assert!(result.is_ok());
}

// Green: limitパラメータを追加
pub async fn get_projects(&self, limit: Option<i32>, offset: Option<i32>)
    -> Result<QaseResponse<ProjectsResult>> {
    let client = reqwest::Client::new();
    let mut request = client
        .get(&format!("{}/project", self.base_url))
        .header("Token", &self.api_token);

    if let Some(l) = limit {
        request = request.query(&[("limit", l.to_string())]);
    }

    let response = request.send().await?;
    let result: QaseResponse<ProjectsResult> = response.json().await?;
    Ok(result)
}
```
```bash
cargo test
git commit -m "feat: get_projectsにlimitパラメータを追加（Green）"
```

**Step 5: offsetパラメータを追加**
```rust
// 同様にoffsetパラメータのテストと実装を追加...
```

**このアプローチの利点:**
- ✅ 各ステップでコミットできる（問題があれば戻せる）
- ✅ どこで問題が起きているか特定しやすい
- ✅ テストが具体的で意味がある
- ✅ レビューしやすい（差分が小さい）
- ✅ 本当の「テスト駆動」開発になる

#### TDD実践のチェックリスト

各Greenフェーズの実装前に、以下を自問すること：

- [ ] **これは本当に最小限の実装か？**
  - もっと簡単な実装（固定値を返すなど）で済まないか？

- [ ] **もっと小さく分割できないか？**
  - 複数の機能を一度に実装していないか？
  - 各機能を別々のサイクルに分けられないか？

- [ ] **テストは具体的な値を検証しているか？**
  - `assert!(result.is_ok())` だけになっていないか？
  - 期待する値を明示的に書いているか？

- [ ] **1つのテストで1つの事だけテストしているか？**
  - 複数の機能を1つのテストで検証していないか？

- [ ] **失敗を確認したか？**
  - Redフェーズで本当にテストが失敗することを確認したか？
  - 失敗理由が期待通りか確認したか？

#### ベイビーステップの目安

**1つのメソッドを実装する際の推奨サイクル数:**
- **単純な関数**: 3-5サイクル
- **HTTP通信を含む関数**: 5-10サイクル
- **複雑なビジネスロジック**: 10-20サイクル

**「大きすぎる」サインの例:**
- 1つのコミットで50行以上のコードを追加
- 1つのテストで複数のアサーション（3つ以上）
- テストが失敗した時に原因が複数考えられる
- 実装に10分以上かかる

**「ちょうど良い」サインの例:**
- 1つのコミットで10-20行のコード追加
- 1つのテストで1-2個の関連するアサーション
- テストが失敗した時に原因が明確
- 実装に2-5分程度

### 4. コミット戦略

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
git commit -m "test: プロジェクト作成の失敗するテストを追加"

# Green
git commit -m "feat: プロジェクト作成の基本実装"

# Refactor
git commit -m "refactor: バリデーションロジックを抽出"

# その他
git commit -m "chore: rusqlite依存関係を追加"
git commit -m "docs: READMEにセットアップ手順を追加"
```

#### コミット前のチェック
- [ ] フォーマッターを実行したか（`cargo fmt`）
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
git commit -m "chore: プロジェクトリポジトリ用ファイルを追加"
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
git commit -m "test: get_allが空リストを返すテストを追加（失敗）"
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
git commit -m "feat: get_allを実装して空リストを返す"
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
git commit -m "refactor: get_allのロジックをfetch_all_from_dbに抽出"
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
git commit -am "test: get_by_codeの失敗するテストを追加"
```

**Green → Refactor → 繰り返し**

#### Step 4: PR作成前のチェック
```bash
# 1. フォーマッターを実行
cargo fmt

# 2. すべてのテストが通ることを確認
cargo test

# 3. Lintチェックを実行（CI失敗を防ぐ）
cargo clippy --all-targets --all-features -- -D warnings

# ⚠️ clippyでエラーが出た場合は必ず修正してからプッシュ
# 例: PR #7では dead_code 警告が発生し、CI失敗後に修正が必要だった

# 4. すべてクリアしたらプッシュ
git push origin feature/project-repository

# 5. PRを作成（GitHub CLI使用）
gh pr create --title "feat: implement ProjectRepository" \
  --body "- Implement get_all()
- Implement get_by_code()
- Implement save()
- Implement delete()

Closes #1"
```

#### Step 5: CI結果確認
```bash
# PR番号を確認（例: #4）
gh pr list

# CIの実行状況を確認
gh pr checks 4

# CIが失敗した場合は詳細を確認
gh run view <run-id> --log-failed

# 修正が必要な場合
# 1. 修正してコミット
git add .
git commit -m "fix: CI指摘事項を修正"

# 2. プッシュ（CIが自動で再実行される）
git push origin feature/project-repository

# 3. 再度CI結果を確認
gh pr checks 4
```

#### Step 6: マージ
```bash
# すべてのCIが通ったことを確認してマージ
gh pr merge 4 --squash
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
- [ ] コミット前に`cargo fmt`を実行

### 完了時
- [ ] フォーマッターを実行（`cargo fmt`）
- [ ] すべてのテストが通る（`cargo test`）
- [ ] Lintチェックが通る（`cargo clippy --all-targets --all-features -- -D warnings`）
- [ ] コンパイルエラーがない
- [ ] コミットメッセージが適切
- [ ] PRを作成
- [ ] CIが通ることを確認（`gh pr checks <PR番号>`）
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
