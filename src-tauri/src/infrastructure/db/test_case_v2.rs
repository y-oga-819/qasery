//! TestCase Aggregate用のRepository実装（新しいDDD設計版）
//!
//! 新しい`domain::test_case::TestCase`モデルに対する
//! `domain::test_case::repository::TestCaseRepository`トレイトの実装

use crate::domain::project::ProjectCode;
use crate::domain::test_case::repository::TestCaseRepository as TestCaseRepositoryTrait;
use crate::domain::test_case::{OrderedStep, StepId, StepType, TestCase, TestCaseId};
use crate::domain::test_suite::TestSuiteId;
use crate::infrastructure::db::Database;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::params;

impl TestCaseRepositoryTrait for Database {
    fn get_by_id(&self, id: TestCaseId) -> Result<Option<TestCase>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at
             FROM test_cases
             WHERE id = ? AND is_deleted = 0",
        )?;

        let result = stmt.query_row([id.value()], |row| {
            let id: i64 = row.get(0)?;
            let project_code_str: String = row.get(1)?;
            let suite_id: i64 = row.get(2)?;
            let title: String = row.get(3)?;
            let description: Option<String> = row.get(4)?;
            let preconditions: Option<String> = row.get(5)?;
            let postconditions: Option<String> = row.get(6)?;
            let severity: i32 = row.get(7)?;
            let priority: i32 = row.get(8)?;
            let test_type: i32 = row.get(9)?;
            let status: i32 = row.get(10)?;
            let automation: i32 = row.get(11)?;
            let author_id: Option<i64> = row.get(12)?;
            let created_at_str: String = row.get(13)?;
            let updated_at_str: String = row.get(14)?;

            Ok((
                id,
                project_code_str,
                suite_id,
                title,
                description,
                preconditions,
                postconditions,
                severity,
                priority,
                test_type,
                status,
                automation,
                author_id,
                created_at_str,
                updated_at_str,
            ))
        });

        match result {
            Ok((
                id,
                project_code_str,
                suite_id,
                title,
                description,
                preconditions,
                postconditions,
                severity,
                priority,
                test_type,
                status,
                automation,
                author_id,
                created_at_str,
                updated_at_str,
            )) => {
                let project_code =
                    ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

                // ステップを取得
                let steps = self.load_steps(id)?;

                let test_case = TestCase::from_api(
                    TestCaseId::new(id),
                    project_code,
                    TestSuiteId::new(suite_id),
                    title,
                    description,
                    preconditions,
                    postconditions,
                    severity,
                    priority,
                    test_type,
                    status,
                    automation,
                    author_id,
                    steps,
                    created_at,
                    updated_at,
                );
                Ok(Some(test_case))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_by_suite(&self, suite_id: TestSuiteId) -> Result<Vec<TestCase>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_code, suite_id, title, description, preconditions, postconditions,
                    severity, priority, type, status, automation, author_id,
                    created_at, updated_at
             FROM test_cases
             WHERE suite_id = ? AND is_deleted = 0",
        )?;

        let cases = stmt
            .query_map([suite_id.value()], |row| {
                let id: i64 = row.get(0)?;
                let project_code_str: String = row.get(1)?;
                let suite_id: i64 = row.get(2)?;
                let title: String = row.get(3)?;
                let description: Option<String> = row.get(4)?;
                let preconditions: Option<String> = row.get(5)?;
                let postconditions: Option<String> = row.get(6)?;
                let severity: i32 = row.get(7)?;
                let priority: i32 = row.get(8)?;
                let test_type: i32 = row.get(9)?;
                let status: i32 = row.get(10)?;
                let automation: i32 = row.get(11)?;
                let author_id: Option<i64> = row.get(12)?;
                let created_at_str: String = row.get(13)?;
                let updated_at_str: String = row.get(14)?;

                Ok((
                    id,
                    project_code_str,
                    suite_id,
                    title,
                    description,
                    preconditions,
                    postconditions,
                    severity,
                    priority,
                    test_type,
                    status,
                    automation,
                    author_id,
                    created_at_str,
                    updated_at_str,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (
            id,
            project_code_str,
            suite_id,
            title,
            description,
            preconditions,
            postconditions,
            severity,
            priority,
            test_type,
            status,
            automation,
            author_id,
            created_at_str,
            updated_at_str,
        ) in cases
        {
            let project_code =
                ProjectCode::new(project_code_str).map_err(|e| anyhow::anyhow!("{:?}", e))?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

            // ステップを取得
            let steps = self.load_steps(id)?;

            let test_case = TestCase::from_api(
                TestCaseId::new(id),
                project_code,
                TestSuiteId::new(suite_id),
                title,
                description,
                preconditions,
                postconditions,
                severity,
                priority,
                test_type,
                status,
                automation,
                author_id,
                steps,
                created_at,
                updated_at,
            );
            result.push(test_case);
        }

        Ok(result)
    }

    fn save(&self, test_case: &TestCase) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let sync_metadata = test_case.sync_metadata();
        let local_updated_at = sync_metadata.local_updated_at().to_rfc3339();
        let remote_updated_at = sync_metadata.remote_updated_at().map(|dt| dt.to_rfc3339());
        let sync_status = match sync_metadata.sync_status() {
            crate::domain::common::sync_metadata::SyncStatus::Clean => "clean",
            crate::domain::common::sync_metadata::SyncStatus::LocalModified => "local_modified",
            crate::domain::common::sync_metadata::SyncStatus::RemoteModified => "remote_modified",
            crate::domain::common::sync_metadata::SyncStatus::Conflict => "conflict",
        };
        let is_deleted = if sync_metadata.is_deleted() { 1 } else { 0 };

        // TestCaseを保存
        conn.execute(
            "INSERT OR REPLACE INTO test_cases
             (id, project_code, suite_id, title, description, preconditions, postconditions,
              severity, priority, type, status, automation, author_id,
              created_at, updated_at, local_updated_at, remote_updated_at, sync_status, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
            params![
                test_case.id().value(),
                test_case.project_code().value(),
                test_case.suite_id().value(),
                test_case.title(),
                test_case.description(),
                test_case.preconditions(),
                test_case.postconditions(),
                test_case.severity(),
                test_case.priority(),
                test_case.test_type(),
                test_case.status(),
                test_case.automation(),
                test_case.author_id(),
                test_case.created_at().to_rfc3339(),
                test_case.updated_at().to_rfc3339(),
                local_updated_at,
                remote_updated_at,
                sync_status,
                is_deleted,
            ],
        )?;

        // ステップを保存（まず既存のものを削除）
        conn.execute(
            "DELETE FROM test_case_steps WHERE test_case_id = ?",
            params![test_case.id().value()],
        )?;

        // 新しいステップを挿入
        for step in test_case.steps() {
            self.save_step(&conn, test_case.id().value(), step)?;
        }

        Ok(())
    }

    fn delete(&self, id: TestCaseId) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        conn.execute(
            "UPDATE test_cases SET is_deleted = 1 WHERE id = ?",
            params![id.value()],
        )?;

        Ok(())
    }
}

impl Database {
    /// テストケースのステップを読み込む
    fn load_steps(&self, test_case_id: i64) -> Result<Vec<OrderedStep>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, shared_step_hash, position
             FROM test_case_steps
             WHERE test_case_id = ?
             ORDER BY position ASC",
        )?;

        let steps = stmt
            .query_map([test_case_id], |row| {
                let id: i64 = row.get(0)?;
                let shared_step_hash: Option<String> = row.get(1)?;
                let position: i32 = row.get(2)?;

                Ok((id, shared_step_hash, position))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();
        for (id, shared_step_hash, position) in steps {
            let ordered_step = if let Some(hash) = shared_step_hash {
                // 共有ステップ
                OrderedStep::new_shared(hash, position)
            } else {
                // 固有ステップ
                OrderedStep::new_own(StepId::new(id), position)
            };
            result.push(ordered_step);
        }

        Ok(result)
    }

    /// ステップを保存
    fn save_step(
        &self,
        conn: &rusqlite::Connection,
        test_case_id: i64,
        step: &OrderedStep,
    ) -> Result<()> {
        match step.step_type() {
            StepType::Own(step_id) => {
                // 固有ステップ（shared_step_hashはNULL）
                conn.execute(
                    "INSERT INTO test_case_steps
                     (id, test_case_id, hash, shared_step_hash, position, action, expected_result, data)
                     VALUES (?1, ?2, ?3, NULL, ?4, '', NULL, NULL)",
                    params![
                        step_id.value(),
                        test_case_id,
                        format!("step_{}", step_id.value()), // hashは一意性のため
                        step.position(),
                    ],
                )?;
            }
            StepType::Shared(shared_hash) => {
                // 共有ステップ
                conn.execute(
                    "INSERT INTO test_case_steps
                     (test_case_id, hash, shared_step_hash, position, action, expected_result, data)
                     VALUES (?1, ?2, ?3, ?4, '', NULL, NULL)",
                    params![
                        test_case_id,
                        format!("shared_{}", shared_hash), // hashは一意性のため
                        shared_hash,
                        step.position(),
                    ],
                )?;
            }
        }
        Ok(())
    }
}
