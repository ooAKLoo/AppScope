use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::error::AppError;
use crate::models::*;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_id TEXT NOT NULL,
                event TEXT NOT NULL,
                user_id TEXT NOT NULL,
                properties TEXT DEFAULT '{}',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                event_date DATE DEFAULT (date('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS feedbacks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_id TEXT NOT NULL,
                content TEXT NOT NULL,
                user_id TEXT,
                contact TEXT,
                properties TEXT DEFAULT '{}',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes if not exist
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_events_app_date ON events(app_id, event_date)",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_events_user ON events(app_id, user_id, event_date)",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_feedbacks_app ON feedbacks(app_id, created_at DESC)",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_event(
        &self,
        app_id: &str,
        event: &str,
        user_id: &str,
        properties: &serde_json::Value,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO events (app_id, event, user_id, properties)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(app_id)
        .bind(event)
        .bind(user_id)
        .bind(properties.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_feedback(
        &self,
        app_id: &str,
        content: &str,
        user_id: Option<&str>,
        contact: Option<&str>,
        properties: &serde_json::Value,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO feedbacks (app_id, content, user_id, contact, properties)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(app_id)
        .bind(content)
        .bind(user_id)
        .bind(contact)
        .bind(properties.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_apps(&self) -> Result<Vec<AppInfo>, AppError> {
        let app_ids: Vec<AppIdRow> = sqlx::query_as(
            "SELECT DISTINCT app_id FROM events ORDER BY app_id",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut apps = Vec::new();
        for row in app_ids {
            let dau_today: CountRow = sqlx::query_as(
                r#"
                SELECT COUNT(DISTINCT user_id) as count
                FROM events
                WHERE app_id = ? AND event = '$open' AND event_date = date('now')
                "#,
            )
            .bind(&row.app_id)
            .fetch_one(&self.pool)
            .await?;

            let total_installs: CountRow = sqlx::query_as(
                r#"
                SELECT COUNT(*) as count
                FROM events
                WHERE app_id = ? AND event = '$install'
                "#,
            )
            .bind(&row.app_id)
            .fetch_one(&self.pool)
            .await?;

            apps.push(AppInfo {
                app_id: row.app_id,
                dau_today: dau_today.count,
                total_installs: total_installs.count,
            });
        }

        Ok(apps)
    }

    pub async fn get_dau(&self, app_id: &str, days: i32) -> Result<Vec<DauData>, AppError> {
        let rows: Vec<DauRow> = sqlx::query_as(
            r#"
            SELECT event_date, COUNT(DISTINCT user_id) as dau
            FROM events
            WHERE app_id = ?
              AND event = '$open'
              AND event_date >= date('now', ? || ' days')
            GROUP BY event_date
            ORDER BY event_date
            "#,
        )
        .bind(app_id)
        .bind(format!("-{}", days))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| DauData {
                date: r.event_date,
                dau: r.dau,
            })
            .collect())
    }

    pub async fn get_installs(&self, app_id: &str, days: i32) -> Result<(i64, Vec<InstallData>), AppError> {
        let total: CountRow = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM events
            WHERE app_id = ? AND event = '$install'
            "#,
        )
        .bind(app_id)
        .fetch_one(&self.pool)
        .await?;

        let rows: Vec<InstallRow> = sqlx::query_as(
            r#"
            SELECT event_date, COUNT(*) as installs
            FROM events
            WHERE app_id = ?
              AND event = '$install'
              AND event_date >= date('now', ? || ' days')
            GROUP BY event_date
            ORDER BY event_date
            "#,
        )
        .bind(app_id)
        .bind(format!("-{}", days))
        .fetch_all(&self.pool)
        .await?;

        Ok((
            total.count,
            rows.into_iter()
                .map(|r| InstallData {
                    date: r.event_date,
                    installs: r.installs,
                })
                .collect(),
        ))
    }

    pub async fn get_retention(&self, app_id: &str, _start_date: Option<&str>) -> Result<Vec<RetentionData>, AppError> {
        // Get cohorts (first appearance of each user)
        let rows: Vec<RetentionRow> = sqlx::query_as(
            r#"
            WITH cohort AS (
                SELECT user_id, MIN(event_date) as first_date
                FROM events
                WHERE app_id = ? AND event = '$open'
                GROUP BY user_id
            ),
            retention AS (
                SELECT
                    c.first_date as cohort_date,
                    COUNT(DISTINCT c.user_id) as day0_users,
                    COUNT(DISTINCT CASE WHEN e.event_date = date(c.first_date, '+1 day') THEN e.user_id END) as day1_users,
                    COUNT(DISTINCT CASE WHEN e.event_date = date(c.first_date, '+7 days') THEN e.user_id END) as day7_users,
                    COUNT(DISTINCT CASE WHEN e.event_date = date(c.first_date, '+30 days') THEN e.user_id END) as day30_users
                FROM cohort c
                LEFT JOIN events e ON c.user_id = e.user_id
                    AND e.app_id = ?
                    AND e.event = '$open'
                WHERE c.first_date >= date('now', '-60 days')
                GROUP BY c.first_date
            )
            SELECT
                cohort_date,
                day0_users,
                ROUND(day1_users * 100.0 / NULLIF(day0_users, 0), 1) as retention_day1,
                ROUND(day7_users * 100.0 / NULLIF(day0_users, 0), 1) as retention_day7,
                ROUND(day30_users * 100.0 / NULLIF(day0_users, 0), 1) as retention_day30
            FROM retention
            ORDER BY cohort_date DESC
            LIMIT 30
            "#,
        )
        .bind(app_id)
        .bind(app_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| RetentionData {
                cohort_date: r.cohort_date,
                day0: r.day0_users,
                day1: r.retention_day1,
                day7: r.retention_day7,
                day30: r.retention_day30,
            })
            .collect())
    }

    pub async fn get_feedbacks(&self, app_id: &str, limit: i32) -> Result<Vec<FeedbackData>, AppError> {
        let rows: Vec<FeedbackData> = sqlx::query_as(
            r#"
            SELECT id, content, user_id, contact, datetime(created_at) as created_at
            FROM feedbacks
            WHERE app_id = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(app_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct RetentionRow {
    cohort_date: String,
    day0_users: i64,
    retention_day1: Option<f64>,
    retention_day7: Option<f64>,
    retention_day30: Option<f64>,
}
