// プロジェクト一覧コンポーネント

import React, { useEffect } from 'react';
import { useStore } from '../store';
import { listProjects, syncProjects } from '../services/tauri';
import { Button, Spinner, ErrorMessage } from './common';
import type { Project } from '../types';

/**
 * プロジェクト一覧コンポーネント
 */
export const ProjectList: React.FC = () => {
  const { projects, selectedProject, setProjects, selectProject, loading, error, setLoading, setError } = useStore();

  // 初期化時にプロジェクト一覧を取得
  useEffect(() => {
    loadProjects();
  }, []);

  /**
   * プロジェクト一覧を取得
   */
  const loadProjects = async () => {
    try {
      setLoading(true);
      setError(null);
      const projectList = await listProjects();
      setProjects(projectList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'プロジェクト一覧の取得に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  /**
   * プロジェクトを同期
   */
  const handleSync = async () => {
    try {
      setLoading(true);
      setError(null);
      const count = await syncProjects();
      await loadProjects();
      console.log(`${count}件のプロジェクトを同期しました`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'プロジェクトの同期に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  /**
   * プロジェクトを選択
   */
  const handleSelectProject = (project: Project) => {
    selectProject(project);
  };

  return (
    <div className="h-full flex flex-col">
      {/* ヘッダー */}
      <div className="p-4 border-b border-gray-200">
        <div className="flex items-center justify-between mb-2">
          <h2 className="text-lg font-semibold text-gray-800">プロジェクト</h2>
          <Button onClick={handleSync} variant="secondary" disabled={loading} size="sm">
            同期
          </Button>
        </div>
      </div>

      {/* エラー表示 */}
      {error && (
        <div className="p-4">
          <ErrorMessage message={error} onClose={() => setError(null)} />
        </div>
      )}

      {/* ローディング表示 */}
      {loading && (
        <div className="flex-1 flex items-center justify-center">
          <Spinner size="lg" />
        </div>
      )}

      {/* プロジェクト一覧 */}
      {!loading && (
        <div className="flex-1 overflow-y-auto">
          {projects.length === 0 ? (
            <div className="p-4 text-center text-gray-500">
              <p>プロジェクトがありません</p>
              <p className="text-sm mt-2">「同期」ボタンでQaseから取得してください</p>
            </div>
          ) : (
            <ul>
              {projects.map((project) => (
                <li key={project.code}>
                  <button
                    type="button"
                    className={`w-full text-left px-4 py-3 border-b border-gray-100 hover:bg-gray-50 transition-colors ${
                      selectedProject?.code === project.code ? 'bg-blue-50 border-l-4 border-l-blue-600' : ''
                    }`}
                    onClick={() => handleSelectProject(project)}
                  >
                    <div className="font-medium text-gray-800">{project.title}</div>
                    <div className="text-sm text-gray-500">{project.code}</div>
                    {project.sync_status !== 'clean' && (
                      <div className="mt-1">
                        <span
                          className={`text-xs px-2 py-1 rounded ${
                            project.sync_status === 'local_modified'
                              ? 'bg-yellow-100 text-yellow-800'
                              : project.sync_status === 'remote_modified'
                              ? 'bg-blue-100 text-blue-800'
                              : 'bg-red-100 text-red-800'
                          }`}
                        >
                          {project.sync_status === 'local_modified' && 'ローカル変更'}
                          {project.sync_status === 'remote_modified' && 'リモート変更'}
                          {project.sync_status === 'conflict' && 'コンフリクト'}
                        </span>
                      </div>
                    )}
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
};
