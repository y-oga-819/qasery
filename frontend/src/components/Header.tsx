// ヘッダーコンポーネント

import React, { useState } from 'react';
import { useStore } from '../store';
import { useSyncProjects } from '../hooks/useProjects';
import { useSyncSuites } from '../hooks/useTestSuites';
import { useSyncCases } from '../hooks/useTestCases';
import { Button } from './common';
import type { Project } from '../types';

/**
 * ヘッダーのプロパティ
 */
export interface HeaderProps {
  /** 設定モーダルを開くハンドラ */
  onOpenSettings: () => void;
}

/**
 * ヘッダーコンポーネント
 */
export const Header: React.FC<HeaderProps> = ({ onOpenSettings }) => {
  const { projects, selectedProject, selectProject } = useStore();
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const { syncProjects, loading: syncingProjects } = useSyncProjects();
  const { syncSuites, loading: syncingSuites } = useSyncSuites();
  const { syncCases, loading: syncingCases } = useSyncCases();

  /**
   * プロジェクトを選択
   */
  const handleSelectProject = (project: Project) => {
    selectProject(project);
    setIsDropdownOpen(false);
  };

  /**
   * 同期ボタンのハンドラ
   */
  const handleSync = async () => {
    if (selectedProject) {
      // 選択中プロジェクトのスイートとケースを再同期
      await syncSuites(selectedProject.code);
      await syncCases(selectedProject.code);
    } else {
      // プロジェクト一覧のみ同期
      await syncProjects();
    }
  };

  // いずれかの同期処理が実行中か
  const isSyncing = syncingProjects || syncingSuites || syncingCases;

  return (
    <div className="px-6 py-4 flex items-center justify-between">
      {/* 左側: アプリ名とプロジェクト選択 */}
      <div className="flex items-center gap-4">
        <h1 className="text-xl font-bold text-gray-800">Qasery</h1>

        {/* プロジェクト選択ドロップダウン */}
        <div className="relative">
          <button
            type="button"
            className="flex items-center gap-2 px-4 py-2 bg-white border border-gray-300 rounded hover:bg-gray-50 transition-colors"
            onClick={() => setIsDropdownOpen(!isDropdownOpen)}
          >
            <span className="text-gray-700">
              {selectedProject ? selectedProject.title : 'プロジェクトを選択'}
            </span>
            <svg
              className={`h-4 w-4 text-gray-500 transition-transform ${
                isDropdownOpen ? 'rotate-180' : ''
              }`}
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </button>

          {/* ドロップダウンメニュー */}
          {isDropdownOpen && (
            <>
              {/* 背景オーバーレイ */}
              <div
                className="fixed inset-0 z-10"
                onClick={() => setIsDropdownOpen(false)}
              />

              {/* メニュー */}
              <div className="absolute left-0 mt-2 w-64 bg-white border border-gray-200 rounded shadow-lg z-20 max-h-80 overflow-y-auto">
                {projects.length === 0 ? (
                  <div className="px-4 py-3 text-sm text-gray-500">
                    プロジェクトがありません
                  </div>
                ) : (
                  <ul>
                    {projects.map((project) => (
                      <li key={project.code}>
                        <button
                          type="button"
                          className={`w-full text-left px-4 py-2 hover:bg-gray-100 transition-colors ${
                            selectedProject?.code === project.code
                              ? 'bg-blue-50 text-blue-700'
                              : 'text-gray-700'
                          }`}
                          onClick={() => handleSelectProject(project)}
                        >
                          <div className="font-medium">{project.title}</div>
                          <div className="text-xs text-gray-500">{project.code}</div>
                        </button>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            </>
          )}
        </div>
      </div>

      {/* 右側: 同期ボタン・設定ボタン */}
      <div className="flex items-center gap-2">
        <Button
          variant="secondary"
          size="sm"
          onClick={handleSync}
          loading={isSyncing}
          disabled={isSyncing}
        >
          {isSyncing ? '同期中...' : '同期'}
        </Button>
        <Button variant="secondary" size="sm" onClick={onOpenSettings}>
          設定
        </Button>
      </div>
    </div>
  );
};
