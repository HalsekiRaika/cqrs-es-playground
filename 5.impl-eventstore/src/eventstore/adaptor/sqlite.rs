use std::collections::BTreeSet;
use std::str::FromStr;

use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{SqliteConnection, SqlitePool};

use crate::errors::{ReadError, WriteError};
use crate::eventstore::adaptor::Adaptor;
use crate::eventstore::io::{Reader, Writer};
use crate::eventstore::payload::SerializedEvent;

#[derive(Debug, Clone)]
pub struct SqliteEventStore {
    pool: SqlitePool
}

impl SqliteEventStore {
    pub async fn setup(url: impl AsRef<str>) -> Self {
        let opts = SqliteConnectOptions::from_str(url.as_ref())
            .expect("URL specified is incorrect.")
            .create_if_missing(true);
        
        let pool = SqlitePoolOptions::new()
            .connect_with(opts)
            .await
            .expect("Cannot connect to the database.");
        
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Cannot run the migrations.");
        
        Self { pool }
    }
}

impl Adaptor for SqliteEventStore {}

#[async_trait]
impl Reader for SqliteEventStore {
    async fn read_to(&self, aggregate_id: &str, from: i64, to: i64) -> Result<BTreeSet<SerializedEvent>, ReadError> {
        let mut con = self.pool.acquire().await?;
        let events = InternalSqliteEventStore::read_to(aggregate_id, from, to, &mut con).await?;
        Ok(events)
    }
}

#[async_trait]
impl Writer for SqliteEventStore {
    async fn append(&self, event: SerializedEvent) -> Result<(), WriteError> {
        let mut con = self.pool.acquire().await?;
        InternalSqliteEventStore::append(event, &mut con).await?;
        Ok(())
    }
}

pub struct InternalSqliteEventStore;

impl InternalSqliteEventStore {
    pub async fn read_to(aggregate_id: &str, from: i64, to: i64, con: &mut SqliteConnection) -> Result<BTreeSet<SerializedEvent>, sqlx::Error> {
        // language=sqlite
        let read = sqlx::query_as::<_, SerializedEvent>(r#"
            SELECT
                aggregate_id,
                sequence,
                registry_key,
                bytes,
                created_at
            FROM
                journal
            WHERE
                id LIKE $1
            AND
                sequence BETWEEN $2 AND $3
        "#)
            .bind(aggregate_id)
            .bind(from)
            .bind(to)
            .fetch_all(&mut *con)
            .await?
            .into_iter()
            .collect::<BTreeSet<SerializedEvent>>();
        Ok(read)
    }
    
    pub async fn append(event: SerializedEvent, con: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO 
              journal (
                aggregate_id, 
                sequence, 
                registry_key, 
                bytes, 
                created_at
              )
            VALUES ($1, $2, $3, $4, $5)
        "#)
            .bind(event.id)
            .bind(event.seq)
            .bind(event.key)
            .bind(event.bytes)
            .bind(event.created_at)
            .execute(&mut *con)
            .await?;
        Ok(())
    }
}
