// アプリケーションルートコンポーネント

import { useState, useEffect } from 'react';
import { Layout } from './components/common';
import { Header } from './components/Header';
import { TestSuiteTree } from './components/TestSuiteTree';
import { TestCaseViewer } from './components/TestCaseViewer';
import { SettingsModal } from './components/SettingsModal';
import { useSyncProjects } from './hooks/useProjects';
import { getApiToken } from './services/tauri';

/**
 * アプリケーションコンポーネント
 */
function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { syncProjects } = useSyncProjects();

  // 起動時の初期化処理（マウント時に1回だけ実行）
  useEffect(() => {
    const initialize = async () => {
      // APIトークンが設定されていればプロジェクト一覧を自動同期
      const token = await getApiToken();
      if (token) {
        await syncProjects();
      }
    };

    initialize();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // 起動時に1回だけ実行

  return (
    <>
      <Layout
        header={<Header onOpenSettings={() => setIsSettingsOpen(true)} />}
        left={<TestSuiteTree />}
        right={<TestCaseViewer />}
      />

      {/* 設定モーダル */}
      <SettingsModal isOpen={isSettingsOpen} onClose={() => setIsSettingsOpen(false)} />
    </>
  );
}

export default App;
