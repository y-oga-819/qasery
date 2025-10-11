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
        // 1. まずDBから取得を試みる
        console.log(`[useSharedSteps] Fetching shared steps for project: ${projectCode}`);
        let steps = await invoke<SharedStep[]>('get_shared_steps', {
          projectCode,
        });
        console.log(`[useSharedSteps] Found ${steps.length} shared steps in DB`);

        // 2. 空の場合は同期してから再取得
        if (steps.length === 0) {
          console.log(`[useSharedSteps] No shared steps found for project ${projectCode}, syncing...`);
          try {
            const count = await invoke<number>('sync_shared_steps', { projectCode });
            console.log(`[useSharedSteps] Synced ${count} shared steps`);
            steps = await invoke<SharedStep[]>('get_shared_steps', {
              projectCode,
            });
            console.log(`[useSharedSteps] After sync, found ${steps.length} shared steps`);
          } catch (syncErr) {
            console.warn('[useSharedSteps] Failed to sync shared steps:', syncErr);
            // 同期失敗しても続行（空のマップを返す）
          }
        }

        // デバッグ: 最初の3つのShared Stepを出力
        if (steps.length > 0) {
          console.log(`[useSharedSteps] Sample shared steps (first 3):`, steps.slice(0, 3).map(s => ({
            hash: s.hash,
            title: s.title,
            action: s.action.substring(0, 50) + '...'
          })));
        }

        // ハッシュでマップ化
        const map = new Map<string, SharedStep>();
        steps.forEach((step) => {
          map.set(step.hash, step);
        });
        console.log(`[useSharedSteps] Created map with ${map.size} entries`);

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
