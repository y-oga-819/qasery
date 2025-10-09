// レイアウトコンポーネント
// 3ペインレイアウト（Left, Center, Right）

import React from 'react';

/**
 * レイアウトのプロパティ
 */
export interface LayoutProps {
  /** ヘッダー部分 */
  header?: React.ReactNode;
  /** 左ペイン（プロジェクト一覧） */
  left: React.ReactNode;
  /** 中央ペイン（テストスイートツリー） */
  center: React.ReactNode;
  /** 右ペイン（テストケースエディタ） */
  right: React.ReactNode;
}

/**
 * 3ペインレイアウトコンポーネント
 */
export const Layout: React.FC<LayoutProps> = ({ header, left, center, right }) => {
  return (
    <div className="h-screen flex flex-col bg-gray-50">
      {/* ヘッダー */}
      {header && (
        <header className="bg-white border-b border-gray-200 shadow-sm">{header}</header>
      )}

      {/* メインコンテンツ（3ペイン） */}
      <main className="flex-1 flex overflow-hidden">
        {/* 左ペイン */}
        <div className="w-1/4 bg-white border-r border-gray-200 overflow-y-auto">{left}</div>

        {/* 中央ペイン */}
        <div className="w-1/4 bg-white border-r border-gray-200 overflow-y-auto">{center}</div>

        {/* 右ペイン */}
        <div className="flex-1 bg-white overflow-y-auto">{right}</div>
      </main>
    </div>
  );
};
