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
  /** テストスイート一覧を設定 */
  setSuites: (suites: TestSuite[]) => void;
  /** テストスイートを選択 */
  selectSuite: (suite: TestSuite | null) => void;
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
  setSuites: (suites) => set({ suites }),
  selectSuite: (suite) => set({ selectedSuite: suite }),

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
