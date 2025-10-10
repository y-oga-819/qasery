// テストスイート関連のカスタムフック

import { useEffect } from 'react';
import { useStore } from '../store';
import * as tauriService from '../services/tauri';

/**
 * テストスイート一覧を取得して管理するフック
 *
 * @param projectCode プロジェクトコード
 * @returns テストスイート一覧とローディング・エラー状態
 */
export function useTestSuites(projectCode: string | null) {
  const { suites, setSuites, loading, error, setLoading, setError } = useStore();

  useEffect(() => {
    if (!projectCode) {
      setSuites([]);
      return;
    }

    const fetchSuites = async () => {
      try {
        setLoading(true);
        setError(null);
        const suiteList = await tauriService.getSuitesTree(projectCode);
        setSuites(suiteList);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      } finally {
        setLoading(false);
      }
    };

    fetchSuites();
  }, [projectCode, setSuites, setLoading, setError]);

  return { suites, loading, error };
}

/**
 * テストスイートをQaseから同期するフック
 *
 * @returns 同期関数とローディング・エラー状態
 */
export function useSyncSuites() {
  const { selectedProject, setSuites, loading, error, setLoading, setError } = useStore();

  const syncSuites = async (projectCode?: string) => {
    const code = projectCode || selectedProject?.code;

    if (!code) {
      setError('プロジェクトが選択されていません');
      return false;
    }

    try {
      setLoading(true);
      setError(null);

      // Qaseからテストスイートを同期
      await tauriService.syncSuites(code);

      // 同期後、ローカルのテストスイート一覧を再取得
      const suiteList = await tauriService.getSuitesTree(code);
      setSuites(suiteList);

      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      return false;
    } finally {
      setLoading(false);
    }
  };

  return { syncSuites, loading, error };
}
