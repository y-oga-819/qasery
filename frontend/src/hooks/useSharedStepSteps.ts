// Shared Stepの個別ステップ取得用カスタムフック

import { useState, useEffect } from 'react';
import { getSharedStepSteps } from '../services/tauri';
import type { SharedStepStep } from '../types';

/**
 * Shared Stepの個別ステップ取得フック
 *
 * @param sharedStepHash Shared StepハッシュID
 * @returns ステップ一覧、ローディング状態、エラー
 */
export function useSharedStepSteps(sharedStepHash: string | null) {
  const [steps, setSteps] = useState<SharedStepStep[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!sharedStepHash) {
      setSteps([]);
      return;
    }

    const fetchSteps = async () => {
      setLoading(true);
      setError(null);

      try {
        const fetchedSteps = await getSharedStepSteps(sharedStepHash);
        setSteps(fetchedSteps);
      } catch (err) {
        console.error('[useSharedStepSteps] Failed to fetch steps:', err);
        setError(err instanceof Error ? err.message : String(err));
        setSteps([]);
      } finally {
        setLoading(false);
      }
    };

    fetchSteps();
  }, [sharedStepHash]);

  return { steps, loading, error };
}
