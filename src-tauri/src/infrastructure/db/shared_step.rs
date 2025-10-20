// SharedStepRepository の実装
//
// Database構造体に対するSharedStepRepositoryトレイトの実装

use crate::domain::models::{SharedStep, SharedStepStep, SyncStatus};
use crate::domain::repository::SharedStepRepository;
use crate::infrastructure::db::Database;
use anyhow::Result;
use rusqlite::params;

impl SharedStepRepository for Database {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, action, expected_result, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM shared_steps
             WHERE project_code = ? AND is_deleted = 0",
        )?;

        let shared_steps = stmt
            .query_map([project_code], |row| {
                let created_at = row
                    .get::<_, String>(6)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let updated_at = row
                    .get::<_, String>(7)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let local_updated_at = row
                    .get::<_, String>(8)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let sync_status_str = row.get::<_, String>(10)?;
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

                Ok(SharedStep {
                    hash: row.get(0)?,
                    project_code: row.get(1)?,
                    title: row.get(2)?,
                    action: row.get(3)?,
                    expected_result: row.get(4)?,
                    cases_count: row.get(5)?,
                    created_at,
                    updated_at,
                    local_updated_at,
                    remote_updated_at: row
                        .get::<_, Option<String>>(9)?
                        .and_then(|s| s.parse().ok()),
                    sync_status,
                    is_deleted: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(shared_steps)
    }

    fn get_by_hash(&self, hash: &str) -> Result<Option<SharedStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, action, expected_result, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM shared_steps
             WHERE hash = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([hash], |row| {
            let created_at = row
                .get::<_, String>(6)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let updated_at = row
                .get::<_, String>(7)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let local_updated_at = row
                .get::<_, String>(8)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            let sync_status_str = row.get::<_, String>(10)?;
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

            Ok(SharedStep {
                hash: row.get(0)?,
                project_code: row.get(1)?,
                title: row.get(2)?,
                action: row.get(3)?,
                expected_result: row.get(4)?,
                cases_count: row.get(5)?,
                created_at,
                updated_at,
                local_updated_at,
                remote_updated_at: row
                    .get::<_, Option<String>>(9)?
                    .and_then(|s| s.parse().ok()),
                sync_status,
                is_deleted: row.get(11)?,
            })
        });

        match result {
            Ok(shared_step) => Ok(Some(shared_step)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, shared_step: &SharedStep) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let sync_status_str = match shared_step.sync_status {
            SyncStatus::Clean => "clean",
            SyncStatus::LocalModified => "local_modified",
            SyncStatus::RemoteModified => "remote_modified",
            SyncStatus::Conflict => "conflict",
        };

        conn.execute(
            "INSERT OR REPLACE INTO shared_steps
             (hash, project_code, title, action, expected_result, cases_count,
              created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                shared_step.hash,
                shared_step.project_code,
                shared_step.title,
                shared_step.action,
                shared_step.expected_result,
                shared_step.cases_count,
                shared_step.created_at.to_rfc3339(),
                shared_step.updated_at.to_rfc3339(),
                shared_step.local_updated_at.to_rfc3339(),
                shared_step
                    .remote_updated_at
                    .as_ref()
                    .map(|dt| dt.to_rfc3339()),
                sync_status_str,
                shared_step.is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, hash: &str) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute(
            "UPDATE shared_steps SET is_deleted = 1 WHERE hash = ?",
            [hash],
        )?;

        Ok(())
    }

    fn get_steps(&self, shared_step_hash: &str) -> Result<Vec<SharedStepStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, shared_step_hash, hash, position, action, expected_result, data, parent_step_id
             FROM shared_step_steps
             WHERE shared_step_hash = ?
             ORDER BY position",
        )?;

        let steps = stmt
            .query_map([shared_step_hash], |row| {
                Ok(SharedStepStep {
                    id: row.get(0)?,
                    shared_step_hash: row.get(1)?,
                    hash: row.get(2)?,
                    position: row.get(3)?,
                    action: row.get(4)?,
                    expected_result: row.get(5)?,
                    data: row.get(6)?,
                    parent_step_id: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(steps)
    }

    fn save_steps(&self, shared_step_hash: &str, steps: &[SharedStepStep]) -> Result<()> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        // トランザクション開始
        let tx = conn.transaction()?;

        // 既存のステップを削除
        tx.execute(
            "DELETE FROM shared_step_steps WHERE shared_step_hash = ?",
            [shared_step_hash],
        )?;

        // 新しいステップを保存
        for step in steps {
            tx.execute(
                "INSERT INTO shared_step_steps
                 (shared_step_hash, hash, position, action, expected_result, data, parent_step_id)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    shared_step_hash,
                    step.hash,
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
