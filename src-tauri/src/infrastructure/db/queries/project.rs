//! ProjectQueryRepository の実装
//!
//! Database構造体に対するProjectQueryRepositoryトレイトの実装
//! **参照専用**であり、データ取得のみを行う

use crate::domain::queries::{ProjectQueryRepository, ProjectReadModel, SyncStatus};
use crate::infrastructure::db::Database;
use anyhow::Result;

impl ProjectQueryRepository for Database {
    fn get_all(&self) -> Result<Vec<ProjectReadModel>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM projects
             WHERE is_deleted = 0",
        )?;

        let projects = stmt
            .query_map([], |row| {
                let local_updated_at = row
                    .get::<_, String>(3)?
                    .parse()
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let sync_status_str = row.get::<_, String>(5)?;
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

                Ok(ProjectReadModel {
                    code: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    local_updated_at,
                    remote_updated_at: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| s.parse().ok()),
                    sync_status,
                    is_deleted: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    fn get_by_code(&self, code: &str) -> Result<Option<ProjectReadModel>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted
             FROM projects
             WHERE code = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([code], |row| {
            let local_updated_at = row
                .get::<_, String>(3)?
                .parse()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            let sync_status_str = row.get::<_, String>(5)?;
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

            Ok(ProjectReadModel {
                code: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                local_updated_at,
                remote_updated_at: row
                    .get::<_, Option<String>>(4)?
                    .and_then(|s| s.parse().ok()),
                sync_status,
                is_deleted: row.get(6)?,
            })
        });

        match result {
            Ok(project) => Ok(Some(project)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
