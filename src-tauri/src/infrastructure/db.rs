// SQLite database implementation, repository trait implementation

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::domain::models::{Project, SyncStatus, TestCase, TestCaseStep, TestSuite};
use crate::domain::repository::{ProjectRepository, TestCaseRepository, TestSuiteRepository};

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
        let conn = self.conn.lock().unwrap();

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

        Ok(())
    }
}

/// ProjectRepository traitの実装
impl ProjectRepository for Database {
    fn get_all(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM projects
             WHERE is_deleted = 0"
        )?;

        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    code: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    local_updated_at: row.get::<_, String>(3)?.parse().unwrap(),
                    remote_updated_at: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| s.parse().ok()),
                    sync_status: match row.get::<_, String>(5)?.as_str() {
                        "clean" => SyncStatus::Clean,
                        "local_modified" => SyncStatus::LocalModified,
                        "remote_modified" => SyncStatus::RemoteModified,
                        "conflict" => SyncStatus::Conflict,
                        _ => SyncStatus::Clean,
                    },
                    is_deleted: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    fn get_by_code(&self, code: &str) -> Result<Option<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM projects
             WHERE code = ? AND is_deleted = 0"
        )?;

        let result = stmt.query_row([code], |row| {
            Ok(Project {
                code: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                local_updated_at: row.get::<_, String>(3)?.parse().unwrap(),
                remote_updated_at: row
                    .get::<_, Option<String>>(4)?
                    .and_then(|s| s.parse().ok()),
                sync_status: match row.get::<_, String>(5)?.as_str() {
                    "clean" => SyncStatus::Clean,
                    "local_modified" => SyncStatus::LocalModified,
                    "remote_modified" => SyncStatus::RemoteModified,
                    "conflict" => SyncStatus::Conflict,
                    _ => SyncStatus::Clean,
                },
                is_deleted: row.get(6)?,
            })
        });

        match result {
            Ok(project) => Ok(Some(project)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, project: &Project) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let sync_status_str = match project.sync_status {
            SyncStatus::Clean => "clean",
            SyncStatus::LocalModified => "local_modified",
            SyncStatus::RemoteModified => "remote_modified",
            SyncStatus::Conflict => "conflict",
        };

        conn.execute(
            "INSERT OR REPLACE INTO projects
             (code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                project.code,
                project.title,
                project.description,
                project.local_updated_at.to_rfc3339(),
                project.remote_updated_at.as_ref().map(|dt| dt.to_rfc3339()),
                sync_status_str,
                project.is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, code: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("UPDATE projects SET is_deleted = 1 WHERE code = ?", [code])?;

        Ok(())
    }
}

/// TestSuiteRepository traitの実装
impl TestSuiteRepository for Database {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuite>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions, parent_id, position, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_suites
             WHERE project_code = ? AND is_deleted = 0
             ORDER BY position"
        )?;

        let suites = stmt
            .query_map([project_code], |row| {
                Ok(TestSuite {
                    id: row.get(0)?,
                    project_code: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    preconditions: row.get(4)?,
                    parent_id: row.get(5)?,
                    position: row.get(6)?,
                    cases_count: row.get(7)?,
                    created_at: row.get::<_, String>(8)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(9)?.parse().unwrap(),
                    local_updated_at: row.get::<_, String>(10)?.parse().unwrap(),
                    remote_updated_at: row
                        .get::<_, Option<String>>(11)?
                        .and_then(|s| s.parse().ok()),
                    sync_status: match row.get::<_, String>(12)?.as_str() {
                        "clean" => SyncStatus::Clean,
                        "local_modified" => SyncStatus::LocalModified,
                        "remote_modified" => SyncStatus::RemoteModified,
                        "conflict" => SyncStatus::Conflict,
                        _ => SyncStatus::Clean,
                    },
                    is_deleted: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(suites)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<TestSuite>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions, parent_id, position, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_suites
             WHERE id = ? AND is_deleted = 0"
        )?;

        let result = stmt.query_row([id], |row| {
            Ok(TestSuite {
                id: row.get(0)?,
                project_code: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                preconditions: row.get(4)?,
                parent_id: row.get(5)?,
                position: row.get(6)?,
                cases_count: row.get(7)?,
                created_at: row.get::<_, String>(8)?.parse().unwrap(),
                updated_at: row.get::<_, String>(9)?.parse().unwrap(),
                local_updated_at: row.get::<_, String>(10)?.parse().unwrap(),
                remote_updated_at: row
                    .get::<_, Option<String>>(11)?
                    .and_then(|s| s.parse().ok()),
                sync_status: match row.get::<_, String>(12)?.as_str() {
                    "clean" => SyncStatus::Clean,
                    "local_modified" => SyncStatus::LocalModified,
                    "remote_modified" => SyncStatus::RemoteModified,
                    "conflict" => SyncStatus::Conflict,
                    _ => SyncStatus::Clean,
                },
                is_deleted: row.get(13)?,
            })
        });

        match result {
            Ok(suite) => Ok(Some(suite)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, suite: &TestSuite) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let sync_status_str = match suite.sync_status {
            SyncStatus::Clean => "clean",
            SyncStatus::LocalModified => "local_modified",
            SyncStatus::RemoteModified => "remote_modified",
            SyncStatus::Conflict => "conflict",
        };

        conn.execute(
            "INSERT OR REPLACE INTO test_suites
             (id, project_code, title, description, preconditions, parent_id, position, cases_count,
              created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                suite.id,
                suite.project_code,
                suite.title,
                suite.description,
                suite.preconditions,
                suite.parent_id,
                suite.position,
                suite.cases_count,
                suite.created_at.to_rfc3339(),
                suite.updated_at.to_rfc3339(),
                suite.local_updated_at.to_rfc3339(),
                suite.remote_updated_at.as_ref().map(|dt| dt.to_rfc3339()),
                sync_status_str,
                suite.is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("UPDATE test_suites SET is_deleted = 1 WHERE id = ?", [id])?;

        Ok(())
    }
}

/// TestCaseRepository traitの実装
impl TestCaseRepository for Database {
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_cases
             WHERE suite_id = ? AND is_deleted = 0"
        )?;

        let cases = stmt
            .query_map([suite_id], |row| {
                Ok(TestCase {
                    id: row.get(0)?,
                    project_code: row.get(1)?,
                    suite_id: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                    preconditions: row.get(5)?,
                    postconditions: row.get(6)?,
                    severity: row.get(7)?,
                    priority: row.get(8)?,
                    test_type: row.get(9)?,
                    status: row.get(10)?,
                    automation: row.get(11)?,
                    author_id: row.get(12)?,
                    created_at: row.get::<_, String>(13)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(14)?.parse().unwrap(),
                    local_updated_at: row.get::<_, String>(15)?.parse().unwrap(),
                    remote_updated_at: row
                        .get::<_, Option<String>>(16)?
                        .and_then(|s| s.parse().ok()),
                    sync_status: match row.get::<_, String>(17)?.as_str() {
                        "clean" => SyncStatus::Clean,
                        "local_modified" => SyncStatus::LocalModified,
                        "remote_modified" => SyncStatus::RemoteModified,
                        "conflict" => SyncStatus::Conflict,
                        _ => SyncStatus::Clean,
                    },
                    is_deleted: row.get(18)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(cases)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_cases
             WHERE id = ? AND is_deleted = 0"
        )?;

        let result = stmt.query_row([id], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                project_code: row.get(1)?,
                suite_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                preconditions: row.get(5)?,
                postconditions: row.get(6)?,
                severity: row.get(7)?,
                priority: row.get(8)?,
                test_type: row.get(9)?,
                status: row.get(10)?,
                automation: row.get(11)?,
                author_id: row.get(12)?,
                created_at: row.get::<_, String>(13)?.parse().unwrap(),
                updated_at: row.get::<_, String>(14)?.parse().unwrap(),
                local_updated_at: row.get::<_, String>(15)?.parse().unwrap(),
                remote_updated_at: row
                    .get::<_, Option<String>>(16)?
                    .and_then(|s| s.parse().ok()),
                sync_status: match row.get::<_, String>(17)?.as_str() {
                    "clean" => SyncStatus::Clean,
                    "local_modified" => SyncStatus::LocalModified,
                    "remote_modified" => SyncStatus::RemoteModified,
                    "conflict" => SyncStatus::Conflict,
                    _ => SyncStatus::Clean,
                },
                is_deleted: row.get(18)?,
            })
        });

        match result {
            Ok(case) => Ok(Some(case)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, test_case: &TestCase) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let sync_status_str = match test_case.sync_status {
            SyncStatus::Clean => "clean",
            SyncStatus::LocalModified => "local_modified",
            SyncStatus::RemoteModified => "remote_modified",
            SyncStatus::Conflict => "conflict",
        };

        conn.execute(
            "INSERT OR REPLACE INTO test_cases
             (id, project_code, suite_id, title, description, preconditions, postconditions,
              severity, priority, type, status, automation, author_id,
              created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                test_case.id,
                test_case.project_code,
                test_case.suite_id,
                test_case.title,
                test_case.description,
                test_case.preconditions,
                test_case.postconditions,
                test_case.severity,
                test_case.priority,
                test_case.test_type,
                test_case.status,
                test_case.automation,
                test_case.author_id,
                test_case.created_at.to_rfc3339(),
                test_case.updated_at.to_rfc3339(),
                test_case.local_updated_at.to_rfc3339(),
                test_case
                    .remote_updated_at
                    .as_ref()
                    .map(|dt| dt.to_rfc3339()),
                sync_status_str,
                test_case.is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("UPDATE test_cases SET is_deleted = 1 WHERE id = ?", [id])?;

        Ok(())
    }

    fn get_steps(&self, test_case_id: i64) -> Result<Vec<TestCaseStep>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, test_case_id, hash, shared_step_hash, position, action, expected_result, data, parent_step_id
             FROM test_case_steps
             WHERE test_case_id = ?
             ORDER BY position"
        )?;

        let steps = stmt
            .query_map([test_case_id], |row| {
                Ok(TestCaseStep {
                    id: row.get(0)?,
                    test_case_id: row.get(1)?,
                    hash: row.get(2)?,
                    shared_step_hash: row.get(3)?,
                    position: row.get(4)?,
                    action: row.get(5)?,
                    expected_result: row.get(6)?,
                    data: row.get(7)?,
                    parent_step_id: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(steps)
    }

    fn save_steps(&self, test_case_id: i64, steps: &[TestCaseStep]) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // 既存のステップを削除
        conn.execute(
            "DELETE FROM test_case_steps WHERE test_case_id = ?",
            [test_case_id],
        )?;

        // 新しいステップを保存
        for step in steps {
            conn.execute(
                "INSERT INTO test_case_steps
                 (test_case_id, hash, shared_step_hash, position, action, expected_result, data, parent_step_id)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    test_case_id,
                    step.hash,
                    step.shared_step_hash,
                    step.position,
                    step.action,
                    step.expected_result,
                    step.data,
                    step.parent_step_id,
                ],
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        };

        TestSuiteRepository::save(&db, &suite1).unwrap();
        TestSuiteRepository::save(&db, &suite2).unwrap();

        // プロジェクト別に取得
        let suites = db.get_by_project("TEST").unwrap();
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
}
