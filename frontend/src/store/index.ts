// Zustand ストア
// アプリケーション全体の状態管理

import { create } from 'zustand';
import type { Project, TestSuite, TestCase } from '../types';

/**
 * プロジェクトストアの状態
 */
interface ProjectState {
  /** プロジェクト一覧 */
  projects: Project[];
  /** 選択中のプロジェクト */
  selectedProject: Project | null;
  /** プロジェクト一覧を設定 */
  setProjects: (projects: Project[]) => void;
  /** プロジェクトを選択 */
  selectProject: (project: Project | null) => void;
}

/**
 * テストスイートストアの状態
 */
interface TestSuiteState {
  /** テストスイート一覧 */
  suites: TestSuite[];
  /** 選択中のテストスイート */
  selectedSuite: TestSuite | null;
  /** 展開されているスイートのIDセット */
  expandedSuiteIds: Set<number>;
  /** テストスイート一覧を設定 */
  setSuites: (suites: TestSuite[]) => void;
  /** テストスイートを選択 */
  selectSuite: (suite: TestSuite | null) => void;
  /** スイートの展開状態をトグル */
  toggleSuiteExpanded: (suiteId: number) => void;
}

/**
 * テストケースストアの状態
 */
interface TestCaseState {
  /** テストケース一覧 */
  cases: TestCase[];
  /** 選択中のテストケース */
  selectedCase: TestCase | null;
  /** テストケース一覧を設定 */
  setCases: (cases: TestCase[]) => void;
  /** テストケースを選択 */
  selectCase: (testCase: TestCase | null) => void;
  /** テストケースを更新 */
  updateCase: (testCase: TestCase) => void;
}

/**
 * UI状態ストアの状態
 */
interface UIState {
  /** ローディング中かどうか */
  loading: boolean;
  /** エラーメッセージ */
  error: string | null;
  /** ローディング状態を設定 */
  setLoading: (loading: boolean) => void;
  /** エラーを設定 */
  setError: (error: string | null) => void;
}

/**
 * アプリケーション全体の状態
 */
type AppState = ProjectState & TestSuiteState & TestCaseState & UIState;

/**
 * Zustandストア
 */
export const useStore = create<AppState>((set) => ({
  // プロジェクト関連
  projects: [],
  selectedProject: null,
  setProjects: (projects) => set({ projects }),
  selectProject: (project) => set({ selectedProject: project }),

  // テストスイート関連
  suites: [],
  selectedSuite: null,
  expandedSuiteIds: new Set(),
  setSuites: (suites) => {
    // すべてのスイートIDを収集（再帰的に）
    const collectAllIds = (suites: TestSuite[]): number[] => {
      const ids: number[] = [];
      suites.forEach((suite: any) => {
        ids.push(suite.id);
        if (suite.children && suite.children.length > 0) {
          ids.push(...collectAllIds(suite.children));
        }
      });
      return ids;
    };
    const allIds = collectAllIds(suites);
    set({ suites, expandedSuiteIds: new Set(allIds) });
  },
  selectSuite: (suite) => set({ selectedSuite: suite }),
  toggleSuiteExpanded: (suiteId) =>
    set((state) => {
      const newSet = new Set(state.expandedSuiteIds);
      if (newSet.has(suiteId)) {
        newSet.delete(suiteId);
      } else {
        newSet.add(suiteId);
      }
      return { expandedSuiteIds: newSet };
    }),

  // テストケース関連
  cases: [],
  selectedCase: null,
  setCases: (cases) => set({ cases }),
  selectCase: (testCase) => set({ selectedCase: testCase }),
  updateCase: (testCase) =>
    set((state) => ({
      cases: state.cases.map((c) => (c.id === testCase.id ? testCase : c)),
      selectedCase: state.selectedCase?.id === testCase.id ? testCase : state.selectedCase,
    })),

  // UI状態関連
  loading: false,
  error: null,
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
}));
