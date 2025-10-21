//! SharedStepQueryRepository の実装
//!
//! Database構造体に対するSharedStepQueryRepositoryトレイトの実装
//! **参照専用**であり、データ取得のみを行う

use crate::domain::queries::{
    SharedStepQueryRepository, SharedStepReadModel, SharedStepStepReadModel, SyncStatus,
};
use crate::infrastructure::db::Database;
use anyhow::Result;

impl SharedStepQueryRepository for Database {
    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStepReadModel>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM shared_steps
             WHERE project_code = ? AND is_deleted = 0",
        )?;

        let shared_steps = stmt
            .query_map([project_code], |row| {
                let created_at = row
                    .get::<_, String>(4)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let updated_at = row
                    .get::<_, String>(5)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                let local_updated_at = row
                    .get::<_, String>(6)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let sync_status_str = row.get::<_, String>(8)?;
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

                Ok(SharedStepReadModel {
                    hash: row.get(0)?,
                    project_code: row.get(1)?,
                    title: row.get(2)?,
                    cases_count: row.get(3)?,
                    created_at,
                    updated_at,
                    local_updated_at,
                    remote_updated_at: row
                        .get::<_, Option<String>>(7)?
                        .and_then(|s| s.parse().ok()),
                    sync_status,
                    is_deleted: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(shared_steps)
    }

    fn get_by_hash(&self, hash: &str) -> Result<Option<SharedStepReadModel>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, cases_count,
                    created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM shared_steps
             WHERE hash = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([hash], |row| {
            let created_at = row
                .get::<_, String>(4)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let updated_at = row
                .get::<_, String>(5)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let local_updated_at = row
                .get::<_, String>(6)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            let sync_status_str = row.get::<_, String>(8)?;
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

            Ok(SharedStepReadModel {
                hash: row.get(0)?,
                project_code: row.get(1)?,
                title: row.get(2)?,
                cases_count: row.get(3)?,
                created_at,
                updated_at,
                local_updated_at,
                remote_updated_at: row
                    .get::<_, Option<String>>(7)?
                    .and_then(|s| s.parse().ok()),
                sync_status,
                is_deleted: row.get(9)?,
            })
        });

        match result {
            Ok(shared_step) => Ok(Some(shared_step)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_steps(&self, shared_step_hash: &str) -> Result<Vec<SharedStepStepReadModel>> {
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
                Ok(SharedStepStepReadModel {
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
}
