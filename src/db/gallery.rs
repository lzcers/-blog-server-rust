use chrono::{DateTime, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

#[derive(Deserialize, Serialize)]
pub struct GalleryItem {
    id: i64,
    url: String,
    datetime: String,
    localtion: String,
    description: String,
}

#[derive(Clone)]
pub struct GalleryDB {
    pool: Pool<Sqlite>,
}

fn local_dt_to_utc(dt: &str) -> DateTime<Utc> {
    if let Ok(local_c) = Local::datetime_from_str(&Local, &dt, "%Y/%m/%d %H:%M:%S") {
        DateTime::<Utc>::from_utc(local_c.naive_utc(), Utc)
    } else {
        Utc::now()
    }
}

impl GalleryDB {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(db_url).await?;
        Ok(GalleryDB { pool })
    }

    pub async fn get_items(
        &self,
        page_number: i32,
        page_size: i32,
    ) -> anyhow::Result<Vec<GalleryItem>> {
        todo!()
    }
}
