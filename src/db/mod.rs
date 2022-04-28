use chrono::{DateTime, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

#[derive(Deserialize, Serialize)]
pub struct Note {
    id: i64,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone)]
pub struct BlogDB {
    pool: Pool<Sqlite>,
}

fn local_dt_to_utc(dt: &str) -> DateTime<Utc> {
    if let Ok(local_c) = Local::datetime_from_str(&Local, &dt, "%Y/%m/%d %H:%M:%S") {
        DateTime::<Utc>::from_utc(local_c.naive_utc(), Utc)
    } else {
        Utc::now()
    }
}

impl BlogDB {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(db_url).await?;
        Ok(BlogDB { pool })
    }

    pub async fn get_all_notes(self) -> anyhow::Result<Vec<Note>> {
        let results = sqlx::query!(
            r#"
                SELECT id, content, created_at, updated_at
                FROM notes
            "#
        )
        .map(|row| Note {
            id: row.id,
            content: row.content.expect(""),
            updated_at: Local::from_utc_datetime(&Local, &row.updated_at)
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            created_at: Local::from_utc_datetime(&Local, &row.created_at)
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(results)
    }

    pub async fn add_note(
        self,
        content: String,
        created_at: Option<String>,
        updated_at: Option<String>,
    ) -> anyhow::Result<Note> {
        let id = match (created_at, updated_at) {
            (Some(c_at), Some(u_at)) => {
                let utc_created_at = local_dt_to_utc(&c_at);
                let utc_updated_at = local_dt_to_utc(&u_at);
                sqlx::query!(
                    r#"
                        INSERT INTO notes ( content, created_at, updated_at ) VALUES ( ?1, ?2, ?3 )
                    "#,
                    content,
                    utc_created_at,
                    utc_updated_at
                )
                .execute(&self.pool)
                .await?
                .last_insert_rowid()
            }
            _ => sqlx::query!(
                r#"
                    INSERT INTO notes ( content ) VALUES ( ?1 )
                "#,
                content
            )
            .execute(&self.pool)
            .await?
            .last_insert_rowid(),
        };
        let row = sqlx::query!(
            r#"
                SELECT * FROM notes WHERE id = ?
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(Note {
            id: row.id,
            content: row.content.expect(""),
            updated_at: Local::from_utc_datetime(&Local, &row.updated_at)
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            created_at: Local::from_utc_datetime(&Local, &row.created_at)
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
        })
    }
    pub async fn delete_note(self, id: i64) -> anyhow::Result<u64> {
        Ok(sqlx::query!("DELETE FROM notes WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected())
    }

    pub async fn update_note(self, id: i64, content: String) -> anyhow::Result<u64> {
        let now = Utc::now();
        Ok(sqlx::query!(
            "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
            content,
            now,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
