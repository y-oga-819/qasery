//! TestSuite Aggregate用のRepository実装（新しいDDD設計版）
//!
//! 新しい`domain::test_suite::TestSuite`モデルに対する
//! `domain::test_suite::repository::TestSuiteRepository`トレイトの実装

use crate::domain::project::ProjectCode;
use crate::domain::test_suite::repository::TestSuiteRepository as TestSuiteRepositoryTrait;
use crate::domain::test_suite::{TestSuite, TestSuiteId};
use crate::infrastructure::db::Database;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::params;

impl TestSuiteRepositoryTrait for Database {
    fn get_all(&self) -> Result<Vec<TestSuite>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions,
                    parent_id, position, cases_count, created_at, updated_at
             FROM test_suites
             WHERE is_deleted = 0",
        )?;

        let suites = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let project_code_str: String = row.get(1)?;
                let title: String = row.get(2)?;
                let description: Option<String> = row.get(3)?;
                let preconditions: Option<String> = row.get(4)?;
                let parent_id: Option<i64> = row.get(5)?;
                let position: i32 = row.get(6)?;
                let cases_count: i32 = row.get(7)?;
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;

                Ok((
                    id,
                    project_code_str,
                    title,
                    description,
                    preconditions,
                    parent_id,
                    position,
                    cases_count,
                    created_at_str,
                    updated_at_str,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (
            id,
            project_code_str,
            title,
            description,
            preconditions,
            parent_id,
            position,
            cases_count,
            created_at_str,
            updated_at_str,
        ) in suites
        {
            let project_code =
                ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

            let test_suite = TestSuite::from_api(
                TestSuiteId::new(id),
                project_code,
                title,
                description,
                preconditions,
                position,
                cases_count,
                parent_id.map(TestSuiteId::new),
                created_at,
                updated_at,
            );
            result.push(test_suite);
        }

        Ok(result)
    }

    fn get_by_id(&self, id: TestSuiteId) -> Result<Option<TestSuite>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions,
                    parent_id, position, cases_count, created_at, updated_at
             FROM test_suites
             WHERE id = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([id.value()], |row| {
            let id: i64 = row.get(0)?;
            let project_code_str: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: Option<String> = row.get(3)?;
            let preconditions: Option<String> = row.get(4)?;
            let parent_id: Option<i64> = row.get(5)?;
            let position: i32 = row.get(6)?;
            let cases_count: i32 = row.get(7)?;
            let created_at_str: String = row.get(8)?;
            let updated_at_str: String = row.get(9)?;

            Ok((
                id,
                project_code_str,
                title,
                description,
                preconditions,
                parent_id,
                position,
                cases_count,
                created_at_str,
                updated_at_str,
            ))
        });

        match result {
            Ok((
                id,
                project_code_str,
                title,
                description,
                preconditions,
                parent_id,
                position,
                cases_count,
                created_at_str,
                updated_at_str,
            )) => {
                let project_code =
                    ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

                let test_suite = TestSuite::from_api(
                    TestSuiteId::new(id),
                    project_code,
                    title,
                    description,
                    preconditions,
                    position,
                    cases_count,
                    parent_id.map(TestSuiteId::new),
                    created_at,
                    updated_at,
                );
                Ok(Some(test_suite))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_by_project(&self, project_code: &ProjectCode) -> Result<Vec<TestSuite>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_code, title, description, preconditions,
                    parent_id, position, cases_count, created_at, updated_at
             FROM test_suites
             WHERE project_code = ? AND is_deleted = 0
             ORDER BY position ASC",
        )?;

        let suites = stmt
            .query_map([project_code.value()], |row| {
                let id: i64 = row.get(0)?;
                let project_code_str: String = row.get(1)?;
                let title: String = row.get(2)?;
                let description: Option<String> = row.get(3)?;
                let preconditions: Option<String> = row.get(4)?;
                let parent_id: Option<i64> = row.get(5)?;
                let position: i32 = row.get(6)?;
                let cases_count: i32 = row.get(7)?;
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;

                Ok((
                    id,
                    project_code_str,
                    title,
                    description,
                    preconditions,
                    parent_id,
                    position,
                    cases_count,
                    created_at_str,
                    updated_at_str,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (
            id,
            project_code_str,
            title,
            description,
            preconditions,
            parent_id,
            position,
            cases_count,
            created_at_str,
            updated_at_str,
        ) in suites
        {
            let project_code =
                ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

            let test_suite = TestSuite::from_api(
                TestSuiteId::new(id),
                project_code,
                title,
                description,
                preconditions,
                position,
                cases_count,
                parent_id.map(TestSuiteId::new),
                created_at,
                updated_at,
            );
            result.push(test_suite);
        }

        Ok(result)
    }

    fn save(&self, test_suite: &TestSuite) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let sync_metadata = test_suite.sync_metadata();
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
            "INSERT OR REPLACE INTO test_suites
             (id, project_code, title, description, preconditions, parent_id, position,
              cases_count, created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                test_suite.id().value(),
                test_suite.project_code().value(),
                test_suite.title(),
                test_suite.description(),
                test_suite.preconditions(),
                test_suite.parent_id().map(|id| id.value()),
                test_suite.position(),
                test_suite.cases_count(),
                test_suite.created_at().to_rfc3339(),
                test_suite.updated_at().to_rfc3339(),
                local_updated_at,
                remote_updated_at,
                sync_status,
                is_deleted,
            ],
        )?;

        Ok(())
    }

    fn delete(&self, id: TestSuiteId) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute(
            "UPDATE test_suites SET is_deleted = 1 WHERE id = ?",
            params![id.value()],
        )?;

        Ok(())
    }
}
