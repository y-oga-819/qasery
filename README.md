# Qasery

An open-source desktop IDE for [Qase](https://qase.io). Design, manage, and sync your test assets as living specifications.

[![Release](https://img.shields.io/github/v/release/y-oga-819/qasery)](https://github.com/y-oga-819/qasery/releases)
[![License](https://img.shields.io/github/license/y-oga-819/qasery)](LICENSE)

## 🎯 概要

Qaseryは、QaseのテストケースをローカルPCで効率的に管理・編集できるデスクトップアプリケーションです。オフラインでもテストケースを閲覧でき、必要に応じてQaseと同期できます。

### 主な特徴

- 🔄 **Qaseとの双方向同期**: プロジェクト、テストスイート、テストケースをQaseと同期
- 📱 **オフライン対応**: ローカルデータベースでオフラインアクセス可能
- 🌳 **階層構造表示**: テストスイートを見やすい階層構造で表示
- 🔗 **Shared Steps対応**: 共有ステップの表示と参照
- 🎨 **HTML表示**: テストステップのHTMLフォーマットをサポート
- 🖥️ **クロスプラットフォーム**: macOS、Windows、Linux対応（現在はmacOS版のみリリース）

## 📦 インストール

### macOS (Apple Silicon)

1. [Releases](https://github.com/y-oga-819/qasery/releases)ページから最新の`Qasery_x.x.x_aarch64.dmg`をダウンロード
2. DMGファイルをダブルクリックしてマウント
3. Qasery.appをApplicationsフォルダにドラッグ&ドロップ
4. Launchpadまたはアプリケーションフォルダから起動

**注意**: 初回起動時、macOSのGatekeeperが「開発元が未確認」の警告を表示する場合があります。その場合は、右クリック → 「開く」で起動してください。

## 🚀 使い方

### 初期設定

1. アプリケーションを起動
2. Qase API Tokenを入力
   - Qaseの[API Tokens](https://app.qase.io/user/api/token)ページで取得
3. プロジェクトを同期

### 基本操作

#### プロジェクトの同期
- サイドバーの「Sync Projects」ボタンをクリック
- Qaseから最新のプロジェクト情報を取得

#### テストスイートの表示
- プロジェクトを選択すると、テストスイートが階層構造で表示されます
- フォルダアイコンでスイートの階層を確認可能

#### テストケースの表示
- テストスイートを選択すると、そのスイート内のテストケースが表示されます
- テストケースをクリックすると、詳細とステップが表示されます

#### Shared Stepsの表示
- Shared Stepsタブから共有ステップを確認
- テストケース内で参照されている共有ステップも表示

## 🛠️ 開発

### 必要な環境

- Node.js 18以上
- Rust 1.70以上
- npm または yarn

### セットアップ

```bash
# リポジトリをクローン
git clone https://github.com/y-oga-819/qasery.git
cd qasery

# 依存関係をインストール
npm install
cd frontend && npm install && cd ..

# 開発モードで起動
npm run tauri dev
```

### ビルド

```bash
# プロダクションビルド
npm run tauri build

# ビルド成果物は以下に生成されます
# macOS: src-tauri/target/release/bundle/dmg/
# Windows: src-tauri/target/release/bundle/msi/
# Linux: src-tauri/target/release/bundle/deb/
```

### アーキテクチャ

Qaseryは以下の技術スタックで構築されています：

#### フロントエンド
- **React** - UIフレームワーク
- **TypeScript** - 型安全性
- **Zustand** - 状態管理
- **Tailwind CSS** - スタイリング
- **Vite** - ビルドツール

#### バックエンド (Rust)
- **Tauri** - デスクトップアプリフレームワーク
- **SQLite** - ローカルデータベース
- **reqwest** - HTTP クライアント
- **tokio** - 非同期ランタイム

#### プロジェクト構成

```
qasery/
├── frontend/              # Reactフロントエンド
│   ├── src/
│   │   ├── components/   # UIコンポーネント
│   │   ├── store/        # Zustand状態管理
│   │   └── types/        # TypeScript型定義
│   └── package.json
├── src-tauri/            # Rustバックエンド
│   ├── src/
│   │   ├── application/  # アプリケーション層
│   │   ├── domain/       # ドメイン層
│   │   ├── infrastructure/ # インフラ層
│   │   └── presentation/ # プレゼンテーション層
│   └── Cargo.toml
└── package.json
```

## 🤝 コントリビューション

プルリクエストを歓迎します！大きな変更の場合は、まずIssueを開いて変更内容を議論してください。

### 開発フロー

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'feat: add amazing feature'`)
4. ブランチをプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

詳細な開発ガイドラインは[CLAUDE.md](CLAUDE.md)を参照してください。

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 🐛 バグ報告・機能要望

バグ報告や機能要望は[Issues](https://github.com/y-oga-819/qasery/issues)までお願いします。

## 📝 ロードマップ

- [ ] テストケースの編集機能
- [ ] テストケースの作成機能
- [ ] テストケースの削除機能
- [ ] Intel Mac対応
- [ ] Windows版リリース
- [ ] Linux版リリース
- [ ] GitHub Actionsでの自動リリース
- [ ] コード署名とnotarization

## 🙏 謝辞

このプロジェクトは[Qase.io](https://qase.io)のAPIを使用しています。

---

Made with ❤️ by [y-oga-819](https://github.com/y-oga-819)
