// TestSuiteRepository の実装
//
// Database構造体に対するTestSuiteRepositoryトレイトの実装

use crate::domain::models::{SyncStatus, TestSuite};
use crate::domain::repository::TestSuiteRepository;
use crate::infrastructure::db::Database;
use anyhow::Result;
use rusqlite::params;

impl TestSuiteRepository for Database {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<TestSuite>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions, parent_id, position, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_suites
             WHERE project_code = ? AND is_deleted = 0
             ORDER BY position",
        )?;

        let suites = stmt
            .query_map([project_code], |row| {
                let created_at = row
                    .get::<_, String>(8)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let updated_at = row
                    .get::<_, String>(9)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let local_updated_at = row
                    .get::<_, String>(10)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let sync_status_str = row.get::<_, String>(12)?;
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

                Ok(TestSuite {
                    id: row.get(0)?,
                    project_code: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    preconditions: row.get(4)?,
                    parent_id: row.get(5)?,
                    position: row.get(6)?,
                    cases_count: row.get(7)?,
                    created_at,
                    updated_at,
                    local_updated_at,
                    remote_updated_at: row
                        .get::<_, Option<String>>(11)?
                        .and_then(|s| s.parse().ok()),
                    sync_status,
                    is_deleted: row.get(13)?,
                    children: None,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(suites)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<TestSuite>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions, parent_id, position, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM test_suites
             WHERE id = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([id], |row| {
            let created_at = row
                .get::<_, String>(8)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let updated_at = row
                .get::<_, String>(9)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let local_updated_at = row
                .get::<_, String>(10)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            let sync_status_str = row.get::<_, String>(12)?;
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

            Ok(TestSuite {
                id: row.get(0)?,
                project_code: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                preconditions: row.get(4)?,
                parent_id: row.get(5)?,
                position: row.get(6)?,
                cases_count: row.get(7)?,
                created_at,
                updated_at,
                local_updated_at,
                remote_updated_at: row
                    .get::<_, Option<String>>(11)?
                    .and_then(|s| s.parse().ok()),
                sync_status,
                is_deleted: row.get(13)?,
                children: None,
            })
        });

        match result {
            Ok(suite) => Ok(Some(suite)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, suite: &TestSuite) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

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
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute("UPDATE test_suites SET is_deleted = 1 WHERE id = ?", [id])?;

        Ok(())
    }
}
