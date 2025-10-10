// アプリケーションルートコンポーネント

import { useState } from 'react';
import { Layout } from './components/common';
import { Header } from './components/Header';
import { ProjectList } from './components/ProjectList';
import { TestSuiteTree } from './components/TestSuiteTree';
import { TestCaseViewer } from './components/TestCaseViewer';
import { SettingsModal } from './components/SettingsModal';

/**
 * アプリケーションコンポーネント
 */
function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

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
