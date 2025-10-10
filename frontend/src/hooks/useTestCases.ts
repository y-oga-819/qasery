// テストケース関連のカスタムフック

import { useEffect } from 'react';
import { useStore } from '../store';
import * as tauriService from '../services/tauri';
import type { TestCase } from '../types';

/**
 * テストケース一覧を取得して管理するフック
 *
 * @param suiteId テストスイートID
 * @returns テストケース一覧とローディング・エラー状態
 */
export function useTestCases(suiteId: number | null) {
  const { cases, setCases, loading, error, setLoading, setError } = useStore();

  useEffect(() => {
    if (!suiteId) {
      setCases([]);
      return;
    }

    const fetchCases = async () => {
      try {
        setLoading(true);
        setError(null);
        const caseList = await tauriService.getCasesBySuite(suiteId);
        setCases(caseList);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      } finally {
        setLoading(false);
      }
    };

    fetchCases();
  }, [suiteId, setCases, setLoading, setError]);

  return { cases, loading, error };
}

/**
 * テストケース詳細を取得するフック
 *
 * @param id テストケースID
 * @returns テストケースとローディング・エラー状態
 */
export function useTestCase(id: number | null) {
  const { selectedCase, selectCase, loading, error, setLoading, setError } = useStore();

  useEffect(() => {
    if (!id) {
      selectCase(null);
      return;
    }

    const fetchCase = async () => {
      try {
        setLoading(true);
        setError(null);
        const testCase = await tauriService.getCase(id);
        selectCase(testCase);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      } finally {
        setLoading(false);
      }
    };

    fetchCase();
  }, [id, selectCase, setLoading, setError]);

  return { testCase: selectedCase, loading, error };
}

/**
 * テストケースを保存するフック
 *
 * @returns 保存関数とローディング・エラー状態
 */
export function useSaveTestCase() {
  const { updateCase, loading, error, setLoading, setError } = useStore();

  const saveCase = async (testCase: TestCase) => {
    try {
      setLoading(true);
      setError(null);

      // Tauriコマンドでテストケースを保存
      await tauriService.saveCase(testCase);

      // ストアを更新
      updateCase(testCase);

      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      return false;
    } finally {
      setLoading(false);
    }
  };

  return { saveCase, loading, error };
}

/**
 * テストケースをQaseから同期するフック
 *
 * @returns 同期関数とローディング・エラー状態
 */
export function useSyncCases() {
  const { selectedProject, setCases, loading, error, setLoading, setError } = useStore();

  const syncCases = async (projectCode?: string) => {
    const code = projectCode || selectedProject?.code;

    if (!code) {
      setError('プロジェクトが選択されていません');
      return false;
    }

    try {
      setLoading(true);
      setError(null);

      // Qaseからテストケースを同期
      await tauriService.syncCases(code);

      // 同期後、現在のsuiteIdがあれば再取得（ここでは全体の同期のみ）
      // 実際の再取得は、useTestCasesフックで行われる

      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      return false;
    } finally {
      setLoading(false);
    }
  };

  return { syncCases, loading, error };
}
