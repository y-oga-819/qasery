// SQLite database implementation, repository trait implementation

use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::domain::models::{Project, SyncStatus};
use crate::domain::repository::ProjectRepository;

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
        let conn = Connection::open(path)
            .context("データベース接続の初期化に失敗しました")?;

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
        ).context("projectsテーブルの作成に失敗しました")?;

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

        let projects = stmt.query_map([], |row| {
            Ok(Project {
                code: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                local_updated_at: row.get::<_, String>(3)?.parse().unwrap(),
                remote_updated_at: row.get::<_, Option<String>>(4)?
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
                remote_updated_at: row.get::<_, Option<String>>(4)?
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

        conn.execute(
            "UPDATE projects SET is_deleted = 1 WHERE code = ?",
            [code],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let result = db.save(&project);
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

        db.save(&project1).unwrap();
        db.save(&project2).unwrap();

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

        db.save(&project).unwrap();

        // 削除
        db.delete("TEST").unwrap();

        // 削除後は取得できないことを確認
        let retrieved = db.get_by_code("TEST").unwrap();
        assert!(retrieved.is_none());

        // get_allでも取得できない
        let all_projects = db.get_all().unwrap();
        assert_eq!(all_projects.len(), 0);
    }
}
