// テストケース表示・編集コンポーネント

import { useEffect } from 'react';
import { useStore } from '../store';
import { useTestCases } from '../hooks/useTestCases';
import { useTestSteps } from '../hooks/useTestSteps';
import { Button, Spinner, ErrorMessage } from './common';

/**
 * テストケース表示コンポーネント
 * スイート選択時にテストケース一覧を表示し、選択したケースの詳細を表示
 */
export function TestCaseViewer() {
  const { selectedSuite, selectedCase, selectCase } = useStore();
  const { cases, loading, error } = useTestCases(selectedSuite?.id || null);

  // スイートが変更されたら選択をクリア
  useEffect(() => {
    selectCase(null);
  }, [selectedSuite, selectCase]);

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
        <h2 className="text-lg font-semibold text-gray-800 mb-2">テストケース</h2>
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
              <p className="text-sm">ヘッダーの「同期」ボタンでQaseからテストケースを取り込めます</p>
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
  const { steps, loading: stepsLoading, error: stepsError } = useTestSteps(selectedCase?.id || null);

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

        {/* テストステップ */}
        <div>
          <h4 className="text-sm font-semibold text-gray-700 mb-3">テストステップ</h4>
          {stepsLoading ? (
            <div className="flex items-center justify-center py-8">
              <Spinner />
            </div>
          ) : stepsError ? (
            <ErrorMessage message={stepsError} />
          ) : steps.length === 0 ? (
            <p className="text-sm text-gray-500 text-center py-4">テストステップがありません</p>
          ) : (
            <TestStepsList steps={steps} />
          )}
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

/**
 * テストステップ一覧コンポーネント
 * Shared Stepのグループ化とネストされたステップに対応
 */
interface TestStepsListProps {
  steps: any[];
}

function TestStepsList({ steps }: TestStepsListProps) {
  // ステップを親子関係で構造化
  const buildStepTree = (steps: any[]) => {
    const stepMap = new Map<number, any>();
    const rootSteps: any[] = [];

    // まずすべてのステップをMapに登録
    steps.forEach(step => {
      stepMap.set(step.id, { ...step, children: [] });
    });

    // 親子関係を構築
    steps.forEach(step => {
      const node = stepMap.get(step.id);
      if (step.parent_step_id && stepMap.has(step.parent_step_id)) {
        const parent = stepMap.get(step.parent_step_id);
        parent.children.push(node);
      } else {
        rootSteps.push(node);
      }
    });

    return rootSteps;
  };

  // Shared Stepでグループ化
  const groupBySharedStep = (steps: any[]) => {
    const groups: any[][] = [];
    let currentGroup: any[] = [];
    let currentHash: string | null = null;

    steps.forEach(step => {
      if (step.shared_step_hash) {
        if (currentHash === step.shared_step_hash) {
          // 同じShared Stepグループに追加
          currentGroup.push(step);
        } else {
          // 前のグループを保存
          if (currentGroup.length > 0) {
            groups.push(currentGroup);
          }
          // 新しいグループを開始
          currentHash = step.shared_step_hash;
          currentGroup = [step];
        }
      } else {
        // Shared Stepではない場合
        if (currentGroup.length > 0) {
          groups.push(currentGroup);
          currentGroup = [];
          currentHash = null;
        }
        // 単独ステップとして追加
        groups.push([step]);
      }
    });

    // 最後のグループを追加
    if (currentGroup.length > 0) {
      groups.push(currentGroup);
    }

    return groups;
  };

  const treeSteps = buildStepTree(steps);
  const groupedSteps = groupBySharedStep(treeSteps);

  return (
    <div className="space-y-3">
      {groupedSteps.map((group, groupIndex) => {
        const isSharedStepGroup = group.length > 1 || (group.length === 1 && group[0].shared_step_hash);

        if (isSharedStepGroup) {
          // Shared Stepグループの表示
          return (
            <div key={groupIndex} className="border-2 border-purple-300 rounded-lg p-4 bg-purple-50">
              <div className="flex items-center gap-2 mb-3">
                <span className="inline-block px-3 py-1 rounded-full text-xs font-semibold bg-purple-200 text-purple-800">
                  Shared Step
                </span>
                {group[0].shared_step_hash && (
                  <span className="text-xs text-gray-500">
                    ({group[0].shared_step_hash.substring(0, 8)}...)
                  </span>
                )}
              </div>
              <div className="space-y-2">
                {group.map((step) => (
                  <StepItem key={step.id} step={step} level={0} />
                ))}
              </div>
            </div>
          );
        } else {
          // 通常ステップの表示
          return (
            <div key={groupIndex}>
              {group.map((step) => (
                <StepItem key={step.id} step={step} level={0} />
              ))}
            </div>
          );
        }
      })}
    </div>
  );
}

/**
 * 個別ステップアイテムコンポーネント（ネスト対応）
 */
interface StepItemProps {
  step: any;
  level: number;
}

function StepItem({ step, level }: StepItemProps) {
  const indent = level * 24; // 24px per level

  return (
    <div>
      <div
        className="border border-gray-200 rounded-lg p-4 bg-white"
        style={{ marginLeft: `${indent}px` }}
      >
        <div className="flex items-start gap-3">
          <span className="flex-shrink-0 inline-flex items-center justify-center w-8 h-8 rounded-full bg-blue-100 text-blue-800 text-sm font-medium">
            {step.position}
          </span>
          <div className="flex-1 min-w-0">
            <h5 className="text-sm font-medium text-gray-900 mb-1">
              {step.action || 'アクション'}
            </h5>
            {step.expected_result && (
              <p className="text-sm text-gray-600">
                <span className="font-medium text-gray-700">期待結果: </span>
                {step.expected_result}
              </p>
            )}
            {step.data && (
              <p className="text-xs text-gray-500 mt-1">
                <span className="font-medium">データ: </span>
                {step.data}
              </p>
            )}
          </div>
        </div>
      </div>

      {/* 子ステップを再帰的に表示 */}
      {step.children && step.children.length > 0 && (
        <div className="mt-2">
          {step.children.map((child: any) => (
            <StepItem key={child.id} step={child} level={level + 1} />
          ))}
        </div>
      )}
    </div>
  );
}
