// テストステップデータ取得用カスタムフック

import { useState, useEffect } from 'react';
import { getCaseSteps } from '../services/tauri';
import type { TestCaseStep } from '../types';

/**
 * テストケースのステップ一覧を取得するカスタムフック
 *
 * @param testCaseId テストケースID（nullの場合は取得しない）
 * @returns steps: テストステップ一覧、loading: ローディング中、error: エラーメッセージ
 */
export function useTestSteps(testCaseId: number | null) {
  const [steps, setSteps] = useState<TestCaseStep[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (testCaseId === null) {
      setSteps([]);
      setLoading(false);
      setError(null);
      return;
    }

    let isMounted = true;

    const fetchSteps = async () => {
      setLoading(true);
      setError(null);

      try {
        const result = await getCaseSteps(testCaseId);
        if (isMounted) {
          setSteps(result);
        }
      } catch (err) {
        if (isMounted) {
          setError(err instanceof Error ? err.message : String(err));
        }
      } finally {
        if (isMounted) {
          setLoading(false);
        }
      }
    };

    fetchSteps();

    return () => {
      isMounted = false;
    };
  }, [testCaseId]);

  return { steps, loading, error };
}
