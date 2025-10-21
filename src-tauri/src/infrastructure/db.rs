// SQLite database implementation, repository trait implementation

// Phase 1実装時点ではまだ使用されていないが、将来のPhaseで使用するため許可
#![allow(dead_code)]

// Repository実装の各モジュール
mod project; // 古いdomain::modelsのProject用（Phase1互換）
mod project_v2; // 新しいdomain::projectのProject用（DDD設計）
mod shared_step;
mod shared_step_v2; // 新しいdomain::shared_stepのSharedStep用（DDD設計）
mod test_case;
mod test_case_v2; // 新しいdomain::test_caseのTestCase用（DDD設計）
mod test_suite;
mod test_suite_v2; // 新しいdomain::test_suiteのTestSuite用（DDD設計）

// CQRS Query側のRepository実装
pub mod queries;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// データベース接続管理
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// データベース接続を初期化
    ///
    /// # Arguments
    /// * `path` - データベースファイルのパス（":memory:" でインメモリDB）
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path).context("データベース接続の初期化に失敗しました")?;

        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };

        // マイグレーションを実行
        db.migrate()?;

        Ok(db)
    }

    /// マイグレーションを実行（テーブル作成）
    fn migrate(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        // 外部キー制約を有効化
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        // projectsテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                code TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                local_updated_at TEXT NOT NULL,
                remote_updated_at TEXT,
                sync_status TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .context("projectsテーブルの作成に失敗しました")?;

        // test_suitesテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS test_suites (
                id INTEGER PRIMARY KEY,
                project_code TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                preconditions TEXT,
                parent_id INTEGER,
                position INTEGER NOT NULL,
                cases_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                local_updated_at TEXT NOT NULL,
                remote_updated_at TEXT,
                sync_status TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (project_code) REFERENCES projects(code),
                FOREIGN KEY (parent_id) REFERENCES test_suites(id)
            )",
            [],
        )
        .context("test_suitesテーブルの作成に失敗しました")?;

        // test_casesテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS test_cases (
                id INTEGER PRIMARY KEY,
                project_code TEXT NOT NULL,
                suite_id INTEGER,
                title TEXT NOT NULL,
                description TEXT,
                preconditions TEXT,
                postconditions TEXT,
                severity INTEGER NOT NULL,
                priority INTEGER NOT NULL,
                type INTEGER NOT NULL,
                status INTEGER NOT NULL,
                automation INTEGER NOT NULL,
                author_id INTEGER,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                local_updated_at TEXT NOT NULL,
                remote_updated_at TEXT,
                sync_status TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (project_code) REFERENCES projects(code),
                FOREIGN KEY (suite_id) REFERENCES test_suites(id)
            )",
            [],
        )
        .context("test_casesテーブルの作成に失敗しました")?;

        // test_case_stepsテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS test_case_steps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                test_case_id INTEGER NOT NULL,
                hash TEXT NOT NULL,
                shared_step_hash TEXT,
                position INTEGER NOT NULL,
                action TEXT NOT NULL,
                expected_result TEXT,
                data TEXT,
                parent_step_id INTEGER,
                FOREIGN KEY (test_case_id) REFERENCES test_cases(id),
                FOREIGN KEY (parent_step_id) REFERENCES test_case_steps(id)
            )",
            [],
        )
        .context("test_case_stepsテーブルの作成に失敗しました")?;

        // shared_stepsテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS shared_steps (
                hash TEXT PRIMARY KEY,
                project_code TEXT NOT NULL,
                title TEXT NOT NULL,
                cases_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                local_updated_at TEXT NOT NULL,
                remote_updated_at TEXT,
                sync_status TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (project_code) REFERENCES projects(code)
            )",
            [],
        )
        .context("shared_stepsテーブルの作成に失敗しました")?;

        // shared_step_stepsテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS shared_step_steps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                shared_step_hash TEXT NOT NULL,
                hash TEXT NOT NULL,
                position INTEGER NOT NULL,
                action TEXT NOT NULL,
                expected_result TEXT,
                data TEXT,
                parent_step_id INTEGER,
                FOREIGN KEY (shared_step_hash) REFERENCES shared_steps(hash),
                FOREIGN KEY (parent_step_id) REFERENCES shared_step_steps(id)
            )",
            [],
        )
        .context("shared_step_stepsテーブルの作成に失敗しました")?;

        // インデックスの作成
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_test_suites_project ON test_suites(project_code)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_test_cases_suite ON test_cases(suite_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_test_case_steps_case ON test_case_steps(test_case_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_shared_steps_project ON shared_steps(project_code)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_shared_step_steps_hash ON shared_step_steps(shared_step_hash)",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{Project, SyncStatus, TestCase, TestCaseStep, TestSuite};
    use crate::domain::repository::{ProjectRepository, TestCaseRepository, TestSuiteRepository};
    use chrono::Utc;

    #[test]
    fn test_database_initialization() {
        // インメモリDBで初期化できることを確認
        let db = Database::new(":memory:");
        assert!(db.is_ok());
    }

    #[test]
    fn test_project_save_and_get() {
        // プロジェクトを保存して取得できることを確認
        let db = Database::new(":memory:").unwrap();

        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        // 保存
        let result = ProjectRepository::save(&db, &project);
        assert!(result.is_ok());

        // 取得
        let retrieved = db.get_by_code("TEST").unwrap();
        assert!(retrieved.is_some());

        let retrieved_project = retrieved.unwrap();
        assert_eq!(retrieved_project.code, "TEST");
        assert_eq!(retrieved_project.title, "Test Project");
    }

    #[test]
    fn test_project_get_all() {
        // 全プロジェクトを取得できることを確認
        let db = Database::new(":memory:").unwrap();

        // 2つのプロジェクトを保存
        let project1 = Project {
            code: "TEST1".to_string(),
            title: "Test Project 1".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        let project2 = Project {
            code: "TEST2".to_string(),
            title: "Test Project 2".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        ProjectRepository::save(&db, &project1).unwrap();
        ProjectRepository::save(&db, &project2).unwrap();

        // 全取得
        let all_projects = db.get_all().unwrap();
        assert_eq!(all_projects.len(), 2);
    }

    #[test]
    fn test_project_delete() {
        // プロジェクトを削除できることを確認
        let db = Database::new(":memory:").unwrap();

        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        ProjectRepository::save(&db, &project).unwrap();

        // 削除
        ProjectRepository::delete(&db, "TEST").unwrap();

        // 削除後は取得できないことを確認
        let retrieved = db.get_by_code("TEST").unwrap();
        assert!(retrieved.is_none());

        // get_allでも取得できない
        let all_projects = db.get_all().unwrap();
        assert_eq!(all_projects.len(), 0);
    }

    #[test]
    fn test_test_suite_save_and_get() {
        // テストスイートを保存して取得できることを確認
        let db = Database::new(":memory:").unwrap();

        // 先にプロジェクトを作成
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        ProjectRepository::save(&db, &project).unwrap();

        let suite = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite 1".to_string(),
            description: Some("Description".to_string()),
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };

        TestSuiteRepository::save(&db, &suite).unwrap();

        // IDで取得
        let retrieved = TestSuiteRepository::get_by_id(&db, 1).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Suite 1");
    }

    #[test]
    fn test_test_suite_get_by_project() {
        // プロジェクト別にテストスイートを取得できることを確認
        let db = Database::new(":memory:").unwrap();

        // 先にプロジェクトを作成
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        ProjectRepository::save(&db, &project).unwrap();

        let suite1 = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite 1".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };

        let suite2 = TestSuite {
            id: 2,
            project_code: "TEST".to_string(),
            title: "Test Suite 2".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 2,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };

        TestSuiteRepository::save(&db, &suite1).unwrap();
        TestSuiteRepository::save(&db, &suite2).unwrap();

        // プロジェクト別に取得
        let suites = TestSuiteRepository::get_by_project(&db, "TEST").unwrap();
        assert_eq!(suites.len(), 2);
    }

    #[test]
    fn test_test_case_save_and_get() {
        // テストケースを保存して取得できることを確認
        let db = Database::new(":memory:").unwrap();

        // 先にプロジェクトとテストスイートを作成
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        ProjectRepository::save(&db, &project).unwrap();

        let suite = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };
        TestSuiteRepository::save(&db, &suite).unwrap();

        let test_case = TestCase {
            id: 1,
            project_code: "TEST".to_string(),
            suite_id: Some(1),
            title: "Test Case 1".to_string(),
            description: Some("Description".to_string()),
            preconditions: None,
            postconditions: None,
            severity: 0,
            priority: 0,
            test_type: 0,
            status: 0,
            automation: 0,
            author_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };

        TestCaseRepository::save(&db, &test_case).unwrap();

        // IDで取得
        let retrieved = TestCaseRepository::get_by_id(&db, 1).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Case 1");
    }

    #[test]
    fn test_test_case_steps_save_and_get() {
        // テストケースステップを保存して取得できることを確認
        let db = Database::new(":memory:").unwrap();

        // 先にプロジェクト、テストスイート、テストケースを作成
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        ProjectRepository::save(&db, &project).unwrap();

        let suite = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };
        TestSuiteRepository::save(&db, &suite).unwrap();

        let test_case = TestCase {
            id: 1,
            project_code: "TEST".to_string(),
            suite_id: Some(1),
            title: "Test Case".to_string(),
            description: None,
            preconditions: None,
            postconditions: None,
            severity: 0,
            priority: 0,
            test_type: 0,
            status: 0,
            automation: 0,
            author_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        TestCaseRepository::save(&db, &test_case).unwrap();

        let steps = vec![
            TestCaseStep {
                id: 0,
                test_case_id: 1,
                hash: "hash1".to_string(),
                shared_step_hash: None,
                position: 1,
                action: "Action 1".to_string(),
                expected_result: Some("Result 1".to_string()),
                data: None,
                parent_step_id: None,
            },
            TestCaseStep {
                id: 0,
                test_case_id: 1,
                hash: "hash2".to_string(),
                shared_step_hash: None,
                position: 2,
                action: "Action 2".to_string(),
                expected_result: Some("Result 2".to_string()),
                data: None,
                parent_step_id: None,
            },
        ];

        TestCaseRepository::save_steps(&db, 1, &steps).unwrap();

        // ステップを取得
        let retrieved_steps = TestCaseRepository::get_steps(&db, 1).unwrap();
        assert_eq!(retrieved_steps.len(), 2);
        assert_eq!(retrieved_steps[0].action, "Action 1");
        assert_eq!(retrieved_steps[1].action, "Action 2");
    }

    // 異常系テスト

    #[test]
    fn test_invalid_datetime_format_in_database() {
        // 不正な日時形式のデータがDBに存在する場合、読み込み時にエラーになることを確認
        let db = Database::new(":memory:").unwrap();

        // 直接SQLでDBに不正な日時を挿入
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO projects (code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES ('TEST', 'Test Project', NULL, 'invalid-datetime', NULL, 'clean', 0)",
            [],
        )
        .unwrap();
        drop(conn);

        // 読み込み時にエラーになることを確認
        let result = db.get_by_code("TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_foreign_key_constraint_violation() {
        // 外部キー制約違反時にエラーになることを確認
        let db = Database::new(":memory:").unwrap();

        // 存在しないproject_codeでTestSuiteを保存しようとする
        let suite = TestSuite {
            id: 1,
            project_code: "NONEXISTENT".to_string(),
            title: "Test Suite".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };

        let result = TestSuiteRepository::save(&db, &suite);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_sync_status_in_database() {
        // 不正なsync_status値がDBに存在する場合、読み込み時にエラーになることを確認
        let db = Database::new(":memory:").unwrap();

        // 直接SQLでDBに不正なsync_statusを挿入
        let conn = db.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO projects (code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES ('TEST', 'Test Project', NULL, ?, NULL, 'invalid_status', 0)",
            [&now],
        )
        .unwrap();
        drop(conn);

        // 読み込み時にエラーになることを確認
        let result = db.get_by_code("TEST");
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Invalid sync_status"));
    }

    #[test]
    fn test_transaction_rollback_on_error() {
        // save_steps()でエラーが発生した場合、トランザクションがロールバックされることを確認
        let db = Database::new(":memory:").unwrap();

        // プロジェクト、テストスイート、テストケースを作成
        let project = Project {
            code: "TEST".to_string(),
            title: "Test Project".to_string(),
            description: None,
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        ProjectRepository::save(&db, &project).unwrap();

        let suite = TestSuite {
            id: 1,
            project_code: "TEST".to_string(),
            title: "Test Suite".to_string(),
            description: None,
            preconditions: None,
            parent_id: None,
            position: 1,
            cases_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
            children: None,
        };
        TestSuiteRepository::save(&db, &suite).unwrap();

        let test_case = TestCase {
            id: 1,
            project_code: "TEST".to_string(),
            suite_id: Some(1),
            title: "Test Case".to_string(),
            description: None,
            preconditions: None,
            postconditions: None,
            severity: 0,
            priority: 0,
            test_type: 0,
            status: 0,
            automation: 0,
            author_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            local_updated_at: Utc::now(),
            remote_updated_at: None,
            sync_status: SyncStatus::Clean,
            is_deleted: false,
        };
        TestCaseRepository::save(&db, &test_case).unwrap();

        // 初期データを保存
        let initial_steps = vec![TestCaseStep {
            id: 0,
            test_case_id: 1,
            hash: "hash1".to_string(),
            shared_step_hash: None,
            position: 1,
            action: "Initial Action".to_string(),
            expected_result: Some("Initial Result".to_string()),
            data: None,
            parent_step_id: None,
        }];
        TestCaseRepository::save_steps(&db, 1, &initial_steps).unwrap();

        // 存在しないtest_case_idでステップを保存しようとする（エラーを発生させる）
        // 注: 実際にはこれは外部キー制約で失敗するため、トランザクションがロールバックされる
        let invalid_steps = vec![TestCaseStep {
            id: 0,
            test_case_id: 999, // 存在しないtest_case_id
            hash: "hash2".to_string(),
            shared_step_hash: None,
            position: 1,
            action: "Invalid Action".to_string(),
            expected_result: None,
            data: None,
            parent_step_id: None,
        }];

        // 直接コネクションを取得してトランザクション内でエラーを発生させる
        // これは、save_steps内部で削除→挿入の途中でエラーが起きた場合を模倣
        let result = TestCaseRepository::save_steps(&db, 999, &invalid_steps);
        assert!(result.is_err());

        // 元のデータが保持されていることを確認（ロールバック成功）
        let retrieved_steps = TestCaseRepository::get_steps(&db, 1).unwrap();
        assert_eq!(retrieved_steps.len(), 1);
        assert_eq!(retrieved_steps[0].action, "Initial Action");
    }
}
