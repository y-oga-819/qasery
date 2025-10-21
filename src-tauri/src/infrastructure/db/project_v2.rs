//! Project Aggregate用のRepository実装（新しいDDD設計版）
//!
//! 新しい`domain::project::Project`モデルに対する
//! `domain::project::repository::ProjectRepository`トレイトの実装

use crate::domain::project::repository::ProjectRepository as ProjectRepositoryTrait;
use crate::domain::project::{Project, ProjectCode};
use crate::infrastructure::db::Database;
use anyhow::Result;
use rusqlite::params;

impl ProjectRepositoryTrait for Database {
    fn get_all(&self) -> Result<Vec<Project>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT code, title, description
             FROM projects
             WHERE is_deleted = 0",
        )?;

        let projects = stmt
            .query_map([], |row| {
                let code_str: String = row.get(0)?;
                let title: String = row.get(1)?;
                let description: Option<String> = row.get(2)?;

                Ok((code_str, title, description))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (code_str, title, description) in projects {
            let project_code =
                ProjectCode::new(code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
            let project = Project::new(project_code, title, description);
            result.push(project);
        }

        Ok(result)
    }

    fn get_by_code(&self, code: &ProjectCode) -> Result<Option<Project>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT code, title, description
             FROM projects
             WHERE code = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([code.value()], |row| {
            let code_str: String = row.get(0)?;
            let title: String = row.get(1)?;
            let description: Option<String> = row.get(2)?;

            Ok((code_str, title, description))
        });

        match result {
            Ok((code_str, title, description)) => {
                let project_code =
                    ProjectCode::new(code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
                let project = Project::new(project_code, title, description);
                Ok(Some(project))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self, project: &Project) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let sync_metadata = project.sync_metadata();
        let local_updated_at = sync_metadata.local_updated_at().to_rfc3339();
        let remote_updated_at = sync_metadata.remote_updated_at().map(|dt| dt.to_rfc3339());
        let sync_status = match sync_metadata.sync_status() {
            crate::domain::common::sync_metadata::SyncStatus::Clean => "clean",
            crate::domain::common::sync_metadata::SyncStatus::LocalModified => "local_modified",
            crate::domain::common::sync_metadata::SyncStatus::RemoteModified => "remote_modified",
            crate::domain::common::sync_metadata::SyncStatus::Conflict => "conflict",
        };
        let is_deleted = if sync_metadata.is_deleted() { 1 } else { 0 };

        conn.execute(
            "INSERT OR REPLACE INTO projects
             (code, title, description, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                project.code().value(),
                project.title(),
                project.description(),
                local_updated_at,
                remote_updated_at,
                sync_status,
                is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, code: &ProjectCode) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute(
            "UPDATE projects SET is_deleted = 1 WHERE code = ?",
            params![code.value()],
        )?;

        Ok(())
    }
}
