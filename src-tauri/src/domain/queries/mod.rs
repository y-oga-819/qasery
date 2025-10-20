//! Query側（参照系）のドメインモデル
//!
//! CQRSパターンのQuery側を構成するモジュール。
//! 画面表示に最適化されたReadModelと、参照専用のRepositoryトレイトを提供する。

pub mod read_models;
pub mod repository;

// 便利な再エクスポート
pub use read_models::*;
pub use repository::*;
