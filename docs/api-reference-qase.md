# Qase API v1 リファレンス

## 基本情報

### ベースURL
```
https://api.qase.io/v1
```

### 認証
- ヘッダー: `Token: API_TOKEN`
- APIトークンは[API tokens pages](https://app.qase.io/user/api/token)で管理
- すべてのリクエストはHTTPS必須

### レート制限
- 600 リクエスト/分
- 超過時: HTTP 429 + `Retry-After: 60` ヘッダー

### エラーコード
| コード | 意味 |
|--------|------|
| 400 | Bad Request - リクエストが無効 |
| 401 | Unauthorized - APIキーが間違っている |
| 403 | Forbidden - 権限不足 |
| 404 | Not Found - リソースが見つからない |
| 422 | Unprocessable Entity - バリデーションエラー |
| 429 | Too Many Requests - レート制限超過 |
| 500 | Internal Server Error - サーバーエラー |
| 503 | Service Unavailable - メンテナンス中 |

---

## Projects API

### GET /v1/project
全プロジェクトを取得

#### Query Parameters
- `limit` (integer, optional): 取得件数 (1-100, デフォルト: 10)
- `offset` (integer, optional): スキップ件数 (0-100000, デフォルト: 0)

#### Request Example
```bash
curl --request GET \
  --url 'https://api.qase.io/v1/project?limit=10&offset=0' \
  --header 'Token: YOUR_API_TOKEN' \
  --header 'accept: application/json'
```

#### Response 200 (Success)
```json
{
  "status": true,
  "result": {
    "total": 0,
    "filtered": 0,
    "count": 0,
    "entities": [
      {
        "title": "string",
        "code": "string",
        "counts": {
          "cases": 0,
          "suites": 0,
          "milestones": 0,
          "runs": {
            "total": 0,
            "active": 0
          },
          "defects": {
            "total": 0,
            "open": 0
          }
        }
      }
    ]
  }
}
```

#### Response Fields
- `status` (boolean): リクエストの成功/失敗
- `result.total` (integer): 全体の件数
- `result.filtered` (integer): フィルター後の件数
- `result.count` (integer): 現在のページの件数
- `result.entities` (array): プロジェクトのリスト
  - `title` (string): プロジェクト名
  - `code` (string): プロジェクトコード (2-10文字)
  - `counts` (object): 各種カウント情報

---

### GET /v1/project/{code}
特定のプロジェクトを取得

#### Path Parameters
- `code` (string, required): プロジェクトコード (2-10文字)

#### Request Example
```bash
curl --request GET \
  --url https://api.qase.io/v1/project/DEMO \
  --header 'Token: YOUR_API_TOKEN' \
  --header 'accept: application/json'
```

#### Response 200 (Success)
```json
{
  "status": true,
  "result": {
    "title": "string",
    "code": "string",
    "counts": {
      "cases": 0,
      "suites": 0,
      "milestones": 0,
      "runs": {
        "total": 0,
        "active": 0
      },
      "defects": {
        "total": 0,
        "open": 0
      }
    }
  }
}
```

#### Response Fields
- `status` (boolean): リクエストの成功/失敗
- `result` (object): プロジェクト情報
  - `title` (string): プロジェクト名
  - `code` (string): プロジェクトコード
  - `counts` (object): 各種カウント情報

---

## 実装メモ

### Phase 1で実装するエンドポイント
- [x] GET /v1/project - 全プロジェクト取得
- [x] GET /v1/project/{code} - プロジェクト詳細取得
- [ ] GET /v1/suite/{code} - スイート一覧取得
- [ ] GET /v1/suite/{code}/{id} - スイート詳細取得
- [ ] GET /v1/case/{code} - ケース一覧取得
- [ ] GET /v1/case/{code}/{id} - ケース詳細取得

### エラーハンドリング
- レート制限: 429エラー時に60秒待機してリトライ
- ネットワークエラー: 指数バックオフでリトライ
- 認証エラー: ユーザーにAPIトークンの再設定を促す
- 404エラー: リソースが存在しないことを通知

### テスト戦略
- モックサーバーを使用した統合テスト
- エラーレスポンスのハンドリングテスト
- レート制限のシミュレーション
