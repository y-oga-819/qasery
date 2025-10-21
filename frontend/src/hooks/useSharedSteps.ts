// Shared Step取得用カスタムフック

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface SharedStep {
  hash: string;
  project_code: string;
  title: string;
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
        // 1. まずDBから取得を試みる
        let steps = await invoke<SharedStep[]>('get_shared_steps', {
          projectCode,
        });

        // 2. 空の場合は同期してから再取得
        if (steps.length === 0) {
          try {
            await invoke<number>('sync_shared_steps', { projectCode });
            steps = await invoke<SharedStep[]>('get_shared_steps', {
              projectCode,
            });
          } catch (syncErr) {
            console.warn('[useSharedSteps] Failed to sync shared steps:', syncErr);
            // 同期失敗しても続行（空のマップを返す）
          }
        }

        // ハッシュでマップ化
        const map = new Map<string, SharedStep>();
        steps.forEach((step) => {
          map.set(step.hash, step);
        });

        setSharedStepsMap(map);
      } catch (err) {
        console.error('[useSharedSteps] Failed to fetch shared steps:', err);
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
