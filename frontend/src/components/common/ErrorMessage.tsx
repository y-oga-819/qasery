// エラーメッセージコンポーネント

import React from 'react';

/**
 * エラーメッセージのプロパティ
 */
export interface ErrorMessageProps {
  /** エラーメッセージ */
  message: string;
  /** 閉じるボタンのハンドラ */
  onClose?: () => void;
  /** 追加のCSSクラス */
  className?: string;
}

/**
 * エラーメッセージコンポーネント
 */
export const ErrorMessage: React.FC<ErrorMessageProps> = ({
  message,
  onClose,
  className = '',
}) => {
  return (
    <div
      className={`bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded relative ${className}`}
      role="alert"
    >
      <div className="flex items-start">
        {/* エラーアイコン */}
        <svg
          className="h-5 w-5 text-red-400 mr-2 flex-shrink-0"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fillRule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clipRule="evenodd"
          />
        </svg>

        {/* エラーメッセージ */}
        <span className="flex-1">{message}</span>

        {/* 閉じるボタン */}
        {onClose && (
          <button
            type="button"
            className="ml-2 text-red-400 hover:text-red-600 flex-shrink-0"
            onClick={onClose}
          >
            <svg
              className="h-5 w-5"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </button>
        )}
      </div>
    </div>
  );
};
