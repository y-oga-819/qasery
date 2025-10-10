// プロジェクト関連のカスタムフック

import { useEffect } from 'react';
import { useStore } from '../store';
import * as tauriService from '../services/tauri';

/**
 * プロジェクト一覧を取得して管理するフック
 *
 * @returns プロジェクト一覧とローディング・エラー状態
 */
export function useProjects() {
  const { projects, setProjects, loading, error, setLoading, setError } = useStore();

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        setLoading(true);
        setError(null);
        const projectList = await tauriService.listProjects();
        setProjects(projectList);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      } finally {
        setLoading(false);
      }
    };

    fetchProjects();
  }, [setProjects, setLoading, setError]);

  return { projects, loading, error };
}

/**
 * プロジェクトをQaseから同期するフック
 *
 * @returns 同期関数とローディング・エラー状態
 */
export function useSyncProjects() {
  const { setProjects, loading, error, setLoading, setError } = useStore();

  const syncProjects = async () => {
    try {
      setLoading(true);
      setError(null);

      // Qaseからプロジェクトを同期
      await tauriService.syncProjects();

      // 同期後、ローカルのプロジェクト一覧を再取得
      const projectList = await tauriService.listProjects();
      setProjects(projectList);

      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      return false;
    } finally {
      setLoading(false);
    }
  };

  return { syncProjects, loading, error };
}
