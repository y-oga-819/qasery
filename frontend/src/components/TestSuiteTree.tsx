// テストスイートツリーコンポーネント

import { useEffect } from 'react';
import { useStore } from '../store';
import { useTestSuites, useSyncSuites } from '../hooks/useTestSuites';
import { Button, Spinner, ErrorMessage } from './common';
import type { TestSuite } from '../types';

/**
 * テストスイートツリーコンポーネント
 */
export function TestSuiteTree() {
  const { selectedProject, selectedSuite, selectSuite } = useStore();
  const { suites, loading, error } = useTestSuites(selectedProject?.code || null);
  const { syncSuites, loading: syncing } = useSyncSuites();

  // プロジェクトが変更されたら選択をクリア
  useEffect(() => {
    selectSuite(null);
  }, [selectedProject, selectSuite]);

  const handleSync = async () => {
    await syncSuites();
  };

  const handleSuiteClick = (suite: TestSuite) => {
    selectSuite(suite);
  };

  if (!selectedProject) {
    return (
      <div className="h-full flex items-center justify-center text-gray-500">
        <div className="text-center">
          <p className="text-lg mb-2">プロジェクトを選択してください</p>
          <p className="text-sm">左側のリストからプロジェクトを選択すると、テストスイートが表示されます</p>
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
      <div className="p-4 border-b border-gray-200 flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-800">テストスイート</h2>
        <Button
          onClick={handleSync}
          disabled={syncing}
          variant="secondary"
          size="sm"
        >
          {syncing ? '同期中...' : '同期'}
        </Button>
      </div>

      {/* スイート一覧 */}
      <div className="flex-1 overflow-y-auto p-4">
        {suites.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <p className="mb-2">テストスイートがありません</p>
            <p className="text-sm">「同期」ボタンでQaseからテストスイートを取り込めます</p>
          </div>
        ) : (
          <div className="space-y-1">
            {suites.map((suite) => (
              <TestSuiteItem
                key={suite.id}
                suite={suite}
                selected={selectedSuite?.id === suite.id}
                onClick={() => handleSuiteClick(suite)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * テストスイートアイテムコンポーネント
 */
interface TestSuiteItemProps {
  suite: TestSuite;
  selected: boolean;
  onClick: () => void;
}

function TestSuiteItem({ suite, selected, onClick }: TestSuiteItemProps) {
  return (
    <button
      onClick={onClick}
      className={`
        w-full text-left px-4 py-3 rounded-lg transition-colors
        ${
          selected
            ? 'bg-blue-50 border-2 border-blue-500 text-blue-900'
            : 'bg-white border border-gray-200 hover:bg-gray-50 text-gray-800'
        }
      `}
    >
      <div className="flex items-center justify-between">
        <div className="flex-1 min-w-0">
          <h3 className="font-medium truncate">{suite.title}</h3>
          {suite.description && (
            <p className="text-sm text-gray-600 truncate mt-1">{suite.description}</p>
          )}
        </div>
        <div className="ml-4 flex items-center space-x-3">
          <span className="text-sm text-gray-500">{suite.cases_count} ケース</span>
          {suite.sync_status !== 'clean' && (
            <span className="text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-800">
              未同期
            </span>
          )}
        </div>
      </div>
    </button>
  );
}
