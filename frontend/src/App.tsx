// アプリケーションルートコンポーネント

import { useState, useEffect } from 'react';
import { Layout } from './components/common';
import { Header } from './components/Header';
import { ProjectList } from './components/ProjectList';
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

  // 起動時の初期化処理
  useEffect(() => {
    const initialize = async () => {
      // APIトークンが設定されていればプロジェクト一覧を自動同期
      const token = await getApiToken();
      if (token) {
        await syncProjects();
      }
    };

    initialize();
  }, [syncProjects]);

  return (
    <>
      <Layout
        header={<Header onOpenSettings={() => setIsSettingsOpen(true)} />}
        left={<ProjectList />}
        center={<TestSuiteTree />}
        right={<TestCaseViewer />}
      />

      {/* 設定モーダル */}
      <SettingsModal isOpen={isSettingsOpen} onClose={() => setIsSettingsOpen(false)} />
    </>
  );
}

export default App;
