// Tauri コマンドのラッパー
// Rust側のcommands.rsと対応

import { invoke } from '@tauri-apps/api/tauri';
import type { Project, TestSuite, TestCase } from '../types';

/**
 * プロジェクト一覧を取得
 *
 * @returns プロジェクト一覧
 * @throws エラーメッセージ
 */
export async function listProjects(): Promise<Project[]> {
  return await invoke<Project[]>('list_projects');
}

/**
 * プロジェクトを取得
 *
 * @param code プロジェクトコード
 * @returns プロジェクトまたはnull
 * @throws エラーメッセージ
 */
export async function getProject(code: string): Promise<Project | null> {
  return await invoke<Project | null>('get_project', { code });
}

/**
 * Qaseからプロジェクトを同期
 *
 * @returns 取り込んだプロジェクト数
 * @throws エラーメッセージ
 */
export async function syncProjects(): Promise<number> {
  return await invoke<number>('sync_projects');
}

/**
 * プロジェクトのテストスイート一覧を取得（階層構造）
 *
 * @param projectCode プロジェクトコード
 * @returns テストスイート一覧
 * @throws エラーメッセージ
 */
export async function getSuitesTree(projectCode: string): Promise<TestSuite[]> {
  return await invoke<TestSuite[]>('get_suites_tree', { projectCode });
}

/**
 * テストスイートを取得
 *
 * @param id テストスイートID
 * @returns テストスイートまたはnull
 * @throws エラーメッセージ
 */
export async function getSuite(id: number): Promise<TestSuite | null> {
  return await invoke<TestSuite | null>('get_suite', { id });
}

/**
 * Qaseからテストスイートを同期
 *
 * @param projectCode プロジェクトコード
 * @returns 取り込んだテストスイート数
 * @throws エラーメッセージ
 */
export async function syncSuites(projectCode: string): Promise<number> {
  return await invoke<number>('sync_suites', { projectCode });
}

/**
 * スイート別にテストケースを取得
 *
 * @param suiteId テストスイートID
 * @returns テストケース一覧
 * @throws エラーメッセージ
 */
export async function getCasesBySuite(suiteId: number): Promise<TestCase[]> {
  return await invoke<TestCase[]>('get_cases_by_suite', { suiteId });
}

/**
 * テストケースを取得
 *
 * @param id テストケースID
 * @returns テストケースまたはnull
 * @throws エラーメッセージ
 */
export async function getCase(id: number): Promise<TestCase | null> {
  return await invoke<TestCase | null>('get_case', { id });
}

/**
 * テストケースを保存
 *
 * @param testCase テストケース
 * @throws エラーメッセージ
 */
export async function saveCase(testCase: TestCase): Promise<void> {
  return await invoke<void>('save_case', { testCase });
}

/**
 * Qaseからテストケースを同期
 *
 * @param projectCode プロジェクトコード
 * @returns 取り込んだテストケース数
 * @throws エラーメッセージ
 */
export async function syncCases(projectCode: string): Promise<number> {
  return await invoke<number>('sync_cases', { projectCode });
}

/**
 * APIトークンを保存
 *
 * @param token APIトークン
 * @throws エラーメッセージ
 */
export async function saveApiToken(token: string): Promise<void> {
  return await invoke<void>('save_api_token', { token });
}

/**
 * APIトークンを取得
 *
 * @returns APIトークンまたはnull
 * @throws エラーメッセージ
 */
export async function getApiToken(): Promise<string | null> {
  return await invoke<string | null>('get_api_token');
}
