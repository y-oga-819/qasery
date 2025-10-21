//! SharedStep Aggregate用のRepository実装（新しいDDD設計版）
//!
//! 新しい`domain::shared_step::SharedStep`モデルに対する
//! `domain::shared_step::repository::SharedStepRepository`トレイトの実装

use crate::domain::project::ProjectCode;
use crate::domain::shared_step::repository::SharedStepRepository as SharedStepRepositoryTrait;
use crate::domain::shared_step::{SharedStep, SharedStepId, SharedStepStep};
use crate::infrastructure::db::Database;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::params;

impl SharedStepRepositoryTrait for Database {
    fn get_by_id(&self, id: &SharedStepId) -> Result<Option<SharedStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, cases_count, created_at, updated_at
             FROM shared_steps
             WHERE hash = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([id.value()], |row| {
            let hash: String = row.get(0)?;
            let project_code_str: String = row.get(1)?;
            let title: String = row.get(2)?;
            let cases_count: i32 = row.get(3)?;
            let created_at_str: String = row.get(4)?;
            let updated_at_str: String = row.get(5)?;

            Ok((
                hash,
                project_code_str,
                title,
                cases_count,
                created_at_str,
                updated_at_str,
            ))
        });

        match result {
            Ok((hash, project_code_str, title, cases_count, created_at_str, updated_at_str)) => {
                let project_code =
                    ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

                // ステップを取得
                let steps = self.load_shared_step_steps(&hash)?;

                let shared_step = SharedStep::from_api(
                    SharedStepId::new(hash),
                    project_code,
                    title,
                    steps,
                    cases_count,
                    created_at,
                    updated_at,
                );
                Ok(Some(shared_step))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_by_project(&self, project_code: &str) -> Result<Vec<SharedStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT hash, project_code, title, cases_count, created_at, updated_at
             FROM shared_steps
             WHERE project_code = ? AND is_deleted = 0",
        )?;

        let steps_data = stmt
            .query_map([project_code], |row| {
                let hash: String = row.get(0)?;
                let project_code_str: String = row.get(1)?;
                let title: String = row.get(2)?;
                let cases_count: i32 = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                let updated_at_str: String = row.get(5)?;

                Ok((
                    hash,
                    project_code_str,
                    title,
                    cases_count,
                    created_at_str,
                    updated_at_str,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (hash, project_code_str, title, cases_count, created_at_str, updated_at_str) in
            steps_data
        {
            let project_code =
                ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

            // ステップを取得
            let steps = self.load_shared_step_steps(&hash)?;

            let shared_step = SharedStep::from_api(
                SharedStepId::new(hash),
                project_code,
                title,
                steps,
                cases_count,
                created_at,
                updated_at,
            );
            result.push(shared_step);
        }

        Ok(result)
    }

    fn save(&self, shared_step: &SharedStep) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let sync_metadata = shared_step.sync_metadata();
        let local_updated_at = sync_metadata.local_updated_at().to_rfc3339();
        let remote_updated_at = sync_metadata.remote_updated_at().map(|dt| dt.to_rfc3339());
        let sync_status = match sync_metadata.sync_status() {
            crate::domain::common::sync_metadata::SyncStatus::Clean => "clean",
            crate::domain::common::sync_metadata::SyncStatus::LocalModified => "local_modified",
            crate::domain::common::sync_metadata::SyncStatus::RemoteModified => "remote_modified",
            crate::domain::common::sync_metadata::SyncStatus::Conflict => "conflict",
        };
        let is_deleted = if sync_metadata.is_deleted() { 1 } else { 0 };

        // SharedStepを保存
        conn.execute(
            "INSERT OR REPLACE INTO shared_steps
             (hash, project_code, title, cases_count, created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                shared_step.id().value(),
                shared_step.project_code().value(),
                shared_step.title(),
                shared_step.cases_count(),
                shared_step.created_at().to_rfc3339(),
                shared_step.updated_at().to_rfc3339(),
                local_updated_at,
                remote_updated_at,
                sync_status,
                is_deleted,
            ],
        )?;

        // ステップを保存（まず既存のものを削除）
        conn.execute(
            "DELETE FROM shared_step_steps WHERE shared_step_hash = ?",
            params![shared_step.id().value()],
        )?;

        // 新しいステップを挿入
        for step in shared_step.steps() {
            self.save_shared_step_step(&conn, shared_step.id().value(), step)?;
        }

        Ok(())
    }

    fn delete(&self, id: &SharedStepId) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute(
            "UPDATE shared_steps SET is_deleted = 1 WHERE hash = ?",
            params![id.value()],
        )?;

        Ok(())
    }
}

impl Database {
    /// SharedStepの内部ステップを読み込む
    fn load_shared_step_steps(&self, shared_step_hash: &str) -> Result<Vec<SharedStepStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, hash, action, expected_result, data, parent_step_id, position
             FROM shared_step_steps
             WHERE shared_step_hash = ?
             ORDER BY position ASC",
        )?;

        let steps = stmt
            .query_map([shared_step_hash], |row| {
                let id: i64 = row.get(0)?;
                let hash: String = row.get(1)?;
                let action: String = row.get(2)?;
                let expected_result: Option<String> = row.get(3)?;
                let data: Option<String> = row.get(4)?;
                let parent_step_id: Option<i64> = row.get(5)?;
                let position: i32 = row.get(6)?;

                Ok((
                    id,
                    hash,
                    action,
                    expected_result,
                    data,
                    parent_step_id,
                    position,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (id, hash, action, expected_result, data, parent_step_id, position) in steps {
            let mut step = SharedStepStep::new(id, hash, action, expected_result, position);
            step.set_data(data);
            step.set_parent(parent_step_id);
            result.push(step);
        }

        Ok(result)
    }

    /// SharedStepの内部ステップを保存
    fn save_shared_step_step(
        &self,
        conn: &rusqlite::Connection,
        shared_step_hash: &str,
        step: &SharedStepStep,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO shared_step_steps
             (id, shared_step_hash, hash, action, expected_result, data, parent_step_id, position)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                step.id(),
                shared_step_hash,
                step.hash(),
                step.action(),
                step.expected_result(),
                step.data(),
                step.parent_step_id(),
                step.position(),
            ],
        )?;
        Ok(())
    }
}
