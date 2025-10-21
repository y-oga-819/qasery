// 設定モーダルコンポーネント

import React, { useState, useEffect } from 'react';
import { Modal, Button, ErrorMessage } from './common';
import { saveApiToken, getApiToken, deleteApiToken } from '../services/tauri';

/**
 * 設定モーダルのプロパティ
 */
export interface SettingsModalProps {
  /** 開閉状態 */
  isOpen: boolean;
  /** 閉じるハンドラ */
  onClose: () => void;
}

/**
 * 設定モーダルコンポーネント
 */
export const SettingsModal: React.FC<SettingsModalProps> = ({ isOpen, onClose }) => {
  const [apiToken, setApiToken] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  // モーダルが開かれた時にAPIトークンを取得
  useEffect(() => {
    if (isOpen) {
      loadApiToken();
    }
  }, [isOpen]);

  /**
   * APIトークンを取得
   */
  const loadApiToken = async () => {
    try {
      setLoading(true);
      setError(null);
      const token = await getApiToken();
      if (token) {
        setApiToken(token);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'APIトークンの取得に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  /**
   * APIトークンを保存
   */
  const handleSave = async () => {
    if (!apiToken.trim()) {
      setError('APIトークンを入力してください');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(false);
      await saveApiToken(apiToken);
      setSuccess(true);
      setTimeout(() => {
        onClose();
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'APIトークンの保存に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  /**
   * APIトークンを削除
   */
  const handleDelete = async () => {
    if (!confirm('APIトークンを削除してもよろしいですか？')) {
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(false);
      await deleteApiToken();
      setApiToken('');
      setSuccess(true);
      setTimeout(() => {
        onClose();
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'APIトークンの削除に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  /**
   * モーダルを閉じる
   */
  const handleClose = () => {
    setApiToken('');
    setError(null);
    setSuccess(false);
    onClose();
  };

  return (
    <Modal title="設定" isOpen={isOpen} onClose={handleClose} width="md">
      <div className="space-y-4">
        {/* APIトークン入力 */}
        <div>
          <label htmlFor="api-token" className="block text-sm font-medium text-gray-700 mb-2">
            Qase APIトークン
          </label>
          <input
            id="api-token"
            type="password"
            className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="APIトークンを入力"
            value={apiToken}
            onChange={(e) => setApiToken(e.target.value)}
            disabled={loading}
          />
          <p className="mt-1 text-xs text-gray-500">
            QaseのSettings → API Tokensから取得できます
          </p>
        </div>

        {/* エラー表示 */}
        {error && <ErrorMessage message={error} onClose={() => setError(null)} />}

        {/* 成功メッセージ */}
        {success && (
          <div className="bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded">
            <div className="flex items-center">
              <svg
                className="h-5 w-5 text-green-400 mr-2"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              APIトークンを保存しました
            </div>
          </div>
        )}

        {/* ボタン */}
        <div className="flex justify-between pt-4">
          <div>
            {apiToken && (
              <Button
                variant="secondary"
                onClick={handleDelete}
                disabled={loading}
                className="text-red-600 hover:text-red-700 hover:bg-red-50"
              >
                削除
              </Button>
            )}
          </div>
          <div className="flex gap-2">
            <Button variant="secondary" onClick={handleClose} disabled={loading}>
              キャンセル
            </Button>
            <Button onClick={handleSave} loading={loading} disabled={loading}>
              保存
            </Button>
          </div>
        </div>
      </div>
    </Modal>
  );
};
