// TestCaseRepository の実装
//
// Database構造体に対するTestCaseRepositoryトレイトの実装

use crate::domain::models::{SyncStatus, TestCase, TestCaseStep};
use crate::domain::repository::TestCaseRepository;
use crate::infrastructure::db::Database;
use anyhow::Result;
use rusqlite::params;

impl TestCaseRepository for Database {
    fn get_by_suite(&self, suite_id: i64) -> Result<Vec<TestCase>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_cases
             WHERE suite_id = ? AND is_deleted = 0",
        )?;

        let cases = stmt
            .query_map([suite_id], |row| {
                let created_at = row
                    .get::<_, String>(13)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let updated_at = row
                    .get::<_, String>(14)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let local_updated_at = row
                    .get::<_, String>(15)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let sync_status_str = row.get::<_, String>(17)?;
                let sync_status = match sync_status_str.as_str() {
                    "clean" => SyncStatus::Clean,
                    "local_modified" => SyncStatus::LocalModified,
                    "remote_modified" => SyncStatus::RemoteModified,
                    "conflict" => SyncStatus::Conflict,
                    _ => {
                        return Err(rusqlite::Error::ToSqlConversionFailure(
                            format!("Invalid sync_status: {}", sync_status_str).into(),
                        ))
                    }
                };

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
                    created_at,
                    updated_at,
                    local_updated_at,
                    remote_updated_at: row
                        .get::<_, Option<String>>(16)?
                        .and_then(|s| s.parse().ok()),
                    sync_status,
                    is_deleted: row.get(18)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(cases)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<TestCase>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_cases
             WHERE id = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([id], |row| {
            let created_at = row
                .get::<_, String>(13)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let updated_at = row
                .get::<_, String>(14)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let local_updated_at = row
                .get::<_, String>(15)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            let sync_status_str = row.get::<_, String>(17)?;
            let sync_status = match sync_status_str.as_str() {
                "clean" => SyncStatus::Clean,
                "local_modified" => SyncStatus::LocalModified,
                "remote_modified" => SyncStatus::RemoteModified,
                "conflict" => SyncStatus::Conflict,
                _ => {
                    return Err(rusqlite::Error::ToSqlConversionFailure(
                        format!("Invalid sync_status: {}", sync_status_str).into(),
                    ))
                }
            };

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
                created_at,
                updated_at,
                local_updated_at,
                remote_updated_at: row
                    .get::<_, Option<String>>(16)?
                    .and_then(|s| s.parse().ok()),
                sync_status,
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
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

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
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute("UPDATE test_cases SET is_deleted = 1 WHERE id = ?", [id])?;

        Ok(())
    }

    fn get_steps(&self, test_case_id: i64) -> Result<Vec<TestCaseStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, test_case_id, hash, shared_step_hash, position, action, expected_result, data, parent_step_id
             FROM test_case_steps
             WHERE test_case_id = ?
             ORDER BY position",
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
        let mut conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        // トランザクション開始
        let tx = conn.transaction()?;

        // 既存のステップを削除
        tx.execute(
            "DELETE FROM test_case_steps WHERE test_case_id = ?",
            [test_case_id],
        )?;

        // 新しいステップを保存
        for step in steps {
            tx.execute(
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

        // トランザクションをコミット
        tx.commit()?;

        Ok(())
    }
}
