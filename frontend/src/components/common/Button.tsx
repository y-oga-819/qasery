// ボタンコンポーネント

import React from 'react';

/**
 * ボタンのバリアント
 */
export type ButtonVariant = 'primary' | 'secondary' | 'danger';

/**
 * ボタンのプロパティ
 */
export interface ButtonProps {
  /** ボタンのラベル */
  children: React.ReactNode;
  /** クリック時のハンドラ */
  onClick?: () => void;
  /** ボタンのバリアント */
  variant?: ButtonVariant;
  /** 無効化フラグ */
  disabled?: boolean;
  /** ローディング中フラグ */
  loading?: boolean;
  /** 追加のCSSクラス */
  className?: string;
  /** ボタンのタイプ */
  type?: 'button' | 'submit' | 'reset';
}

/**
 * ボタンコンポーネント
 */
export const Button: React.FC<ButtonProps> = ({
  children,
  onClick,
  variant = 'primary',
  disabled = false,
  loading = false,
  className = '',
  type = 'button',
}) => {
  // バリアントごとのスタイル
  const variantStyles = {
    primary: 'bg-blue-600 hover:bg-blue-700 text-white',
    secondary: 'bg-gray-600 hover:bg-gray-700 text-white',
    danger: 'bg-red-600 hover:bg-red-700 text-white',
  };

  // 無効化またはローディング中のスタイル
  const disabledStyles = 'opacity-50 cursor-not-allowed';

  // 基本スタイル
  const baseStyles = 'px-4 py-2 rounded font-medium transition-colors duration-200';

  // 最終的なクラス名
  const buttonClassName = [
    baseStyles,
    variantStyles[variant],
    (disabled || loading) && disabledStyles,
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <button
      type={type}
      className={buttonClassName}
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading ? (
        <span className="flex items-center gap-2">
          <svg
            className="animate-spin h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          {children}
        </span>
      ) : (
        children
      )}
    </button>
  );
};
