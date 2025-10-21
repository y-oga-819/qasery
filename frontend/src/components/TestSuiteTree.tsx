// テストスイートツリーコンポーネント

import { useEffect, useState } from 'react';
import { useStore } from '../store';
import { useTestSuites } from '../hooks/useTestSuites';
import { Spinner, ErrorMessage } from './common';
import type { TestSuite } from '../types';

/**
 * テストスイートツリーコンポーネント
 */
export function TestSuiteTree() {
  const selectedProject = useStore((state) => state.selectedProject);
  const selectedSuite = useStore((state) => state.selectedSuite);
  const selectSuite = useStore((state) => state.selectSuite);
  const expandedSuiteIds = useStore((state) => state.expandedSuiteIds);
  const toggleSuiteExpanded = useStore((state) => state.toggleSuiteExpanded);
  const { suites, loading, error } = useTestSuites(selectedProject?.code || null);

  // プロジェクトが変更されたら選択をクリア
  useEffect(() => {
    selectSuite(null);
  }, [selectedProject, selectSuite]);

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
      <div className="p-4 border-b border-gray-200">
        <h2 className="text-lg font-semibold text-gray-800">テストスイート</h2>
      </div>

      {/* スイート一覧（階層表示） */}
      <div className="flex-1 overflow-y-auto p-4">
        {suites.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <p className="mb-2">テストスイートがありません</p>
            <p className="text-sm">ヘッダーの「同期」ボタンでQaseからテストスイートを取り込めます</p>
          </div>
        ) : (
          <div className="space-y-1">
            {suites.map((suite) => (
              <TestSuiteTreeItem
                key={suite.id}
                suite={suite}
                level={0}
                selectedId={selectedSuite?.id || null}
                expandedIds={expandedSuiteIds}
                onToggleExpand={toggleSuiteExpanded}
                onSelect={handleSuiteClick}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * テストスイートツリーアイテムコンポーネント（再帰的に階層表示）
 */
interface TestSuiteTreeItemProps {
  suite: TestSuite & { children?: TestSuite[] };
  level: number;
  selectedId: number | null;
  expandedIds: Set<number>;
  onToggleExpand: (suiteId: number) => void;
  onSelect: (suite: TestSuite) => void;
}

function TestSuiteTreeItem({ suite, level, selectedId, expandedIds, onToggleExpand, onSelect }: TestSuiteTreeItemProps) {
  const isExpanded = expandedIds.has(suite.id) || level === 0; // デフォルトでルートは展開
  const hasChildren = suite.children && suite.children.length > 0;
  const isSelected = selectedId === suite.id;

  return (
    <div>
      {/* スイート本体 */}
      <div className="flex items-center">
        {/* インデントガイドライン */}
        {level > 0 && (
          <div className="flex items-center" style={{ width: `${level * 24}px` }}>
            {[...Array(level)].map((_, i) => (
              <div
                key={i}
                className="w-6 h-full flex items-center justify-center"
              >
                <div className="w-px h-full bg-gray-300" />
              </div>
            ))}
          </div>
        )}
        
        {/* スイートカード */}
        <div
          className={`
            flex-1 flex items-center rounded-lg transition-colors relative
            ${
              isSelected
                ? 'bg-blue-50 border-2 border-blue-500'
                : 'bg-white border border-gray-200 hover:bg-gray-50'
            }
          `}
        >
          {/* ルートスイートの左側アクセント */}
          {level === 0 && (
            <div className="absolute left-0 top-0 bottom-0 w-1 bg-blue-600 rounded-l-lg" />
          )}
        {/* 展開/折りたたみボタン */}
        {hasChildren ? (
          <button
            onClick={() => onToggleExpand(suite.id)}
            className="flex-shrink-0 w-6 h-6 flex items-center justify-center text-gray-500 hover:text-gray-700"
          >
            <svg
              className={`w-4 h-4 transition-transform ${isExpanded ? 'rotate-90' : ''}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        ) : (
          <div className="w-6" />
        )}

{/* スイート情報（クリック可能） */}
        <button
          onClick={() => onSelect(suite)}
          className="flex-1 text-left px-4 py-3 min-w-0"
        >
          <div className="flex items-center justify-between">
            <div className="flex-1 min-w-0">
              <h3
                className={`font-medium truncate ${
                  isSelected ? 'text-blue-900' : 'text-gray-800'
                }`}
              >
                {suite.title}
              </h3>
              {suite.description && (
                <p className="text-sm text-gray-600 truncate mt-1">{suite.description}</p>
              )}
            </div>
            <div className="ml-4 flex items-center space-x-3 flex-shrink-0">
              <span className="text-sm text-gray-500">{suite.cases_count} ケース</span>
              {suite.sync_status !== 'clean' && (
                <span className="text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-800">
                  未同期
                </span>
              )}
            </div>
          </div>
        </button>
        </div>
      </div>

      {/* 子スイート（再帰的に表示） */}
      {hasChildren && isExpanded && (
        <div className="mt-1 space-y-1">
          {suite.children!.map((childSuite) => (
            <TestSuiteTreeItem
              key={childSuite.id}
              suite={childSuite}
              level={level + 1}
              selectedId={selectedId}
              expandedIds={expandedIds}
              onToggleExpand={onToggleExpand}
              onSelect={onSelect}
            />
          ))}
        </div>
      )}
    </div>
  );
}
