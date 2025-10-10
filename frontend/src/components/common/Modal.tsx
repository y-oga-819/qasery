// モーダルコンポーネント

import React from 'react';

/**
 * モーダルのプロパティ
 */
export interface ModalProps {
  /** モーダルのタイトル */
  title: string;
  /** モーダルの内容 */
  children: React.ReactNode;
  /** 開閉状態 */
  isOpen: boolean;
  /** 閉じるハンドラ */
  onClose: () => void;
  /** モーダルの幅 */
  width?: 'sm' | 'md' | 'lg' | 'xl';
}

/**
 * 汎用モーダルコンポーネント
 */
export const Modal: React.FC<ModalProps> = ({
  title,
  children,
  isOpen,
  onClose,
  width = 'md',
}) => {
  if (!isOpen) return null;

  // 幅ごとのスタイル
  const widthStyles = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
  };

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      {/* 背景オーバーレイ */}
      <div
        className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
        onClick={onClose}
      />

      {/* モーダルコンテンツ */}
      <div className="flex min-h-screen items-center justify-center p-4">
        <div
          className={`relative bg-white rounded-lg shadow-xl ${widthStyles[width]} w-full`}
          onClick={(e) => e.stopPropagation()}
        >
          {/* ヘッダー */}
          <div className="flex items-center justify-between px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-semibold text-gray-800">{title}</h3>
            <button
              type="button"
              className="text-gray-400 hover:text-gray-600 transition-colors"
              onClick={onClose}
            >
              <svg
                className="h-6 w-6"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>

          {/* ボディ */}
          <div className="px-6 py-4">{children}</div>
        </div>
      </div>
    </div>
  );
};
