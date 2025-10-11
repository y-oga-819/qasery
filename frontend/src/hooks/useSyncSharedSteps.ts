// Shared Step同期用カスタムフック

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Shared Step同期フック
 */
export function useSyncSharedSteps() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Shared Stepを同期
   */
  const syncSharedSteps = async (projectCode: string): Promise<number> => {
    setLoading(true);
    setError(null);

    try {
      const count = await invoke<number>('sync_shared_steps', {
        projectCode,
      });
      console.log(`Synced ${count} shared steps for project ${projectCode}`);
      return count;
    } catch (err) {
      console.error('Failed to sync shared steps:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return { syncSharedSteps, loading, error };
}
