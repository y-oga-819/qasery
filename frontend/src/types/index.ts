// Qaseryアプリケーションの型定義
// Rust側のmodels.rsと対応

/**
 * 同期状態
 */
export type SyncStatus = 'clean' | 'local_modified' | 'remote_modified' | 'conflict';

/**
 * プロジェクト
 */
export interface Project {
  /** プロジェクトコード (2-10文字の識別子) */
  code: string;
  /** プロジェクト名 */
  title: string;
  /** 説明 */
  description: string | null;
  /** ローカル更新日時 */
  local_updated_at: string;
  /** リモート更新日時（前回同期時） */
  remote_updated_at: string | null;
  /** 同期状態 */
  sync_status: SyncStatus;
  /** 削除フラグ */
  is_deleted: boolean;
}

/**
 * テストスイート
 */
export interface TestSuite {
  /** テストスイートID */
  id: number;
  /** プロジェクトコード */
  project_code: string;
  /** テストスイート名 */
  title: string;
  /** 説明 */
  description: string | null;
  /** 事前条件 */
  preconditions: string | null;
  /** 親テストスイートID（階層構造） */
  parent_id: number | null;
  /** 表示順序 */
  position: number;
  /** このスイート内のテストケース数 */
  cases_count: number;
  /** 作成日時（Qase API） */
  created_at: string;
  /** 更新日時（Qase API） */
  updated_at: string;
  /** ローカル更新日時 */
  local_updated_at: string;
  /** リモート更新日時（前回同期時） */
  remote_updated_at: string | null;
  /** 同期状態 */
  sync_status: SyncStatus;
  /** 削除フラグ */
  is_deleted: boolean;
}

/**
 * テストケース
 */
export interface TestCase {
  /** テストケースID */
  id: number;
  /** プロジェクトコード */
  project_code: string;
  /** 所属するテストスイートID */
  suite_id: number | null;
  /** テストケース名 */
  title: string;
  /** 説明 */
  description: string | null;
  /** 事前条件 */
  preconditions: string | null;
  /** 事後条件 */
  postconditions: string | null;
  /** 重要度 */
  severity: number;
  /** 優先度 */
  priority: number;
  /** テストタイプ */
  test_type: number;
  /** ステータス */
  status: number;
  /** 自動化レベル */
  automation: number;
  /** 作成者ID */
  author_id: number | null;
  /** 作成日時（Qase API） */
  created_at: string;
  /** 更新日時（Qase API） */
  updated_at: string;
  /** ローカル更新日時 */
  local_updated_at: string;
  /** リモート更新日時（前回同期時） */
  remote_updated_at: string | null;
  /** 同期状態 */
  sync_status: SyncStatus;
  /** 削除フラグ */
  is_deleted: boolean;
}

/**
 * テストケースステップ
 */
export interface TestCaseStep {
  /** ステップID（ローカルDB用） */
  id: number;
  /** 所属するテストケースID */
  test_case_id: number;
  /** ステップハッシュ（Qase API） */
  hash: string;
  /** Shared StepハッシュID（参照している場合） */
  shared_step_hash: string | null;
  /** 表示順序 */
  position: number;
  /** アクション（実行内容） */
  action: string;
  /** 期待される結果 */
  expected_result: string | null;
  /** データ（入力データなど） */
  data: string | null;
  /** 親ステップID（ネストされたステップ用） */
  parent_step_id: number | null;
}
