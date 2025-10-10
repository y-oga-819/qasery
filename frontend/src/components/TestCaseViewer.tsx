// テストケース表示・編集コンポーネント

import { useEffect } from 'react';
import { useStore } from '../store';
import { useTestCases, useSyncCases } from '../hooks/useTestCases';
import { Button, Spinner, ErrorMessage } from './common';

/**
 * テストケース表示コンポーネント
 * スイート選択時にテストケース一覧を表示し、選択したケースの詳細を表示
 */
export function TestCaseViewer() {
  const { selectedSuite, selectedCase, selectCase } = useStore();
  const { cases, loading, error } = useTestCases(selectedSuite?.id || null);
  const { syncCases, loading: syncing } = useSyncCases();

  // スイートが変更されたら選択をクリア
  useEffect(() => {
    selectCase(null);
  }, [selectedSuite, selectCase]);

  const handleSync = async () => {
    await syncCases();
  };

  if (!selectedSuite) {
    return (
      <div className="h-full flex items-center justify-center text-gray-500">
        <div className="text-center">
          <p className="text-lg mb-2">テストスイートを選択してください</p>
          <p className="text-sm">中央のリストからテストスイートを選択すると、テストケースが表示されます</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="h-full flex items-center justify-center">
        <Spinner />
      </div>
    );
  }

  if (error) {
    return (
      <div className="h-full flex items-center justify-center p-4">
        <ErrorMessage message={error} />
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      {/* ヘッダー */}
      <div className="p-4 border-b border-gray-200">
        <div className="flex items-center justify-between mb-2">
          <h2 className="text-lg font-semibold text-gray-800">テストケース</h2>
          <Button
            onClick={handleSync}
            disabled={syncing}
            variant="secondary"
            size="sm"
          >
            {syncing ? '同期中...' : '同期'}
          </Button>
        </div>
        <p className="text-sm text-gray-600">
          スイート: <span className="font-medium">{selectedSuite.title}</span>
        </p>
      </div>

      {/* テストケース一覧 */}
      <div className="flex-1 overflow-y-auto">
        {cases.length === 0 ? (
          <div className="h-full flex items-center justify-center text-gray-500">
            <div className="text-center p-8">
              <p className="mb-2">テストケースがありません</p>
              <p className="text-sm">「同期」ボタンでQaseからテストケースを取り込めます</p>
            </div>
          </div>
        ) : selectedCase ? (
          <TestCaseDetail />
        ) : (
          <TestCaseList cases={cases} onSelect={selectCase} />
        )}
      </div>
    </div>
  );
}

/**
 * テストケース一覧コンポーネント
 */
interface TestCaseListProps {
  cases: any[];
  onSelect: (testCase: any) => void;
}

function TestCaseList({ cases, onSelect }: TestCaseListProps) {
  return (
    <div className="p-4 space-y-2">
      {cases.map((testCase) => (
        <button
          key={testCase.id}
          onClick={() => onSelect(testCase)}
          className="w-full text-left p-4 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 hover:border-gray-300 transition-colors"
        >
          <div className="flex items-start justify-between">
            <div className="flex-1 min-w-0">
              <h3 className="font-medium text-gray-900 truncate">{testCase.title}</h3>
              {testCase.description && (
                <p className="text-sm text-gray-600 mt-1 line-clamp-2">{testCase.description}</p>
              )}
            </div>
            {testCase.sync_status !== 'clean' && (
              <span className="ml-4 text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-800 whitespace-nowrap">
                未同期
              </span>
            )}
          </div>
        </button>
      ))}
    </div>
  );
}

/**
 * テストケース詳細表示コンポーネント
 */
function TestCaseDetail() {
  const { selectedCase, selectCase } = useStore();

  if (!selectedCase) {
    return null;
  }

  return (
    <div className="h-full flex flex-col">
      {/* 詳細ヘッダー */}
      <div className="p-4 border-b border-gray-200 flex items-center justify-between bg-gray-50">
        <h3 className="text-lg font-semibold text-gray-900">{selectedCase.title}</h3>
        <Button onClick={() => selectCase(null)} variant="secondary" size="sm">
          一覧に戻る
        </Button>
      </div>

      {/* 詳細内容 */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {/* 説明 */}
        {selectedCase.description && (
          <div>
            <h4 className="text-sm font-semibold text-gray-700 mb-2">説明</h4>
            <p className="text-gray-800 whitespace-pre-wrap">{selectedCase.description}</p>
          </div>
        )}

        {/* 事前条件 */}
        {selectedCase.preconditions && (
          <div>
            <h4 className="text-sm font-semibold text-gray-700 mb-2">事前条件</h4>
            <p className="text-gray-800 whitespace-pre-wrap">{selectedCase.preconditions}</p>
          </div>
        )}

        {/* 事後条件 */}
        {selectedCase.postconditions && (
          <div>
            <h4 className="text-sm font-semibold text-gray-700 mb-2">事後条件</h4>
            <p className="text-gray-800 whitespace-pre-wrap">{selectedCase.postconditions}</p>
          </div>
        )}

        {/* メタデータ */}
        <div>
          <h4 className="text-sm font-semibold text-gray-700 mb-2">情報</h4>
          <dl className="grid grid-cols-2 gap-4">
            <div>
              <dt className="text-xs text-gray-500">重要度</dt>
              <dd className="text-sm text-gray-800 mt-1">{selectedCase.severity}</dd>
            </div>
            <div>
              <dt className="text-xs text-gray-500">優先度</dt>
              <dd className="text-sm text-gray-800 mt-1">{selectedCase.priority}</dd>
            </div>
            <div>
              <dt className="text-xs text-gray-500">タイプ</dt>
              <dd className="text-sm text-gray-800 mt-1">{selectedCase.test_type}</dd>
            </div>
            <div>
              <dt className="text-xs text-gray-500">ステータス</dt>
              <dd className="text-sm text-gray-800 mt-1">{selectedCase.status}</dd>
            </div>
            <div>
              <dt className="text-xs text-gray-500">自動化</dt>
              <dd className="text-sm text-gray-800 mt-1">{selectedCase.automation}</dd>
            </div>
            <div>
              <dt className="text-xs text-gray-500">同期状態</dt>
              <dd className="text-sm text-gray-800 mt-1">
                <span
                  className={`inline-block px-2 py-1 rounded text-xs ${
                    selectedCase.sync_status === 'clean'
                      ? 'bg-green-100 text-green-800'
                      : 'bg-yellow-100 text-yellow-800'
                  }`}
                >
                  {selectedCase.sync_status === 'clean' ? '同期済み' : '未同期'}
                </span>
              </dd>
            </div>
          </dl>
        </div>

        {/* 編集機能は Phase 2 で実装 */}
        <div className="pt-4 border-t border-gray-200">
          <p className="text-sm text-gray-500 text-center">
            編集機能は次のフェーズで実装予定です
          </p>
        </div>
      </div>
    </div>
  );
}
