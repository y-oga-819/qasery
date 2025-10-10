// レイアウトコンポーネント
// 2ペインレイアウト（Left, Right）

import React from 'react';

/**
 * レイアウトのプロパティ
 */
export interface LayoutProps {
  /** ヘッダー部分 */
  header?: React.ReactNode;
  /** 左ペイン（テストスイートツリー） */
  left: React.ReactNode;
  /** 右ペイン（テストケースエディタ） */
  right: React.ReactNode;
}

/**
 * 2ペインレイアウトコンポーネント
 */
export const Layout: React.FC<LayoutProps> = ({ header, left, right }) => {
  return (
    <div className="h-screen flex flex-col bg-gray-50">
      {/* ヘッダー */}
      {header && (
        <header className="bg-white border-b border-gray-200 shadow-sm">{header}</header>
      )}

      {/* メインコンテンツ（2ペイン） */}
      <main className="flex-1 flex overflow-hidden">
        {/* 左ペイン */}
        <div className="w-1/3 bg-white border-r border-gray-200 overflow-y-auto">{left}</div>

        {/* 右ペイン */}
        <div className="flex-1 bg-white overflow-y-auto">{right}</div>
      </main>
    </div>
  );
};
