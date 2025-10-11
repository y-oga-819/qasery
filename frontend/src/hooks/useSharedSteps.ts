// Shared Step取得用カスタムフック

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface SharedStep {
  hash: string;
  project_code: string;
  title: string;
  action: string;
  expected_result: string;
  cases_count: number;
  created_at: string;
  updated_at: string;
  local_updated_at: string;
  remote_updated_at: string | null;
  sync_status: string;
  is_deleted: boolean;
}

/**
 * Shared Step取得フック
 * プロジェクトのShared Stepをハッシュでマップ化して返す
 */
export function useSharedSteps(projectCode: string | null) {
  const [sharedStepsMap, setSharedStepsMap] = useState<Map<string, SharedStep>>(new Map());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!projectCode) {
      setSharedStepsMap(new Map());
      return;
    }

    const fetchSharedSteps = async () => {
      setLoading(true);
      setError(null);

      try {
        const steps = await invoke<SharedStep[]>('get_shared_steps', {
          projectCode,
        });

        // ハッシュでマップ化
        const map = new Map<string, SharedStep>();
        steps.forEach((step) => {
          map.set(step.hash, step);
        });

        setSharedStepsMap(map);
      } catch (err) {
        console.error('Failed to fetch shared steps:', err);
        setError(err instanceof Error ? err.message : String(err));
        setSharedStepsMap(new Map());
      } finally {
        setLoading(false);
      }
    };

    fetchSharedSteps();
  }, [projectCode]);

  return { sharedStepsMap, loading, error };
}
