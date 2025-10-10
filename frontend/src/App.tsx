// アプリケーションルートコンポーネント

import { useState } from 'react';
import { Layout } from './components/common';
import { Header } from './components/Header';
import { ProjectList } from './components/ProjectList';
import { TestSuiteTree } from './components/TestSuiteTree';
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
        right={
          <div className="h-full flex items-center justify-center text-gray-500">
            <div className="text-center">
              <p className="text-lg mb-2">テストケースエディタ</p>
              <p className="text-sm">Phase 3で実装予定</p>
            </div>
          </div>
        }
      />

      {/* 設定モーダル */}
      <SettingsModal isOpen={isSettingsOpen} onClose={() => setIsSettingsOpen(false)} />
    </>
  );
}

export default App;
