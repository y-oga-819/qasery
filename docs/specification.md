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

（次のセクションで詳細化予定）

## 3. アーキテクチャ設計

（次のセクションで詳細化予定）

## 4. Qase API統合

（次のセクションで詳細化予定）

## 5. 同期・差分管理

（次のセクションで詳細化予定）

## 6. UI/UX設計

（次のセクションで詳細化予定）

## 7. 開発フェーズ

（次のセクションで詳細化予定）
