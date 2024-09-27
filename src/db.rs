// Pattern extracted from the official SQLite example
// https://github.com/actix/examples/blob/master/databases/sqlite/src/db.rs
use chrono::{DateTime, Utc};
use std::{path::Display, thread::sleep, time::Duration};

use actix_web::{error, web, Error};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub const DB_FILE: &str = "api-db.sqlite";

use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    Statement, ToSql,
};

/// Deliberately not marked async, because it is not intended to be used while
/// the web API itself is live.
pub fn setup(pool: Pool) {
    let conn = pool.get().expect("unable to connect to the database");

    conn.execute(
        "
  CREATE TABLE IF NOT EXISTS usage (
    id INTEGER PRIMARY KEY,
    api_key TEXT,
    endpoint TEXT,
    called_at TEXT
  );",
        (),
    )
    .expect("unable to create database tables");
}

#[derive(Debug)]
pub enum ApiEndpoint {
    ToCelsius,
    ToFahrenheit,
}

impl ApiEndpoint {
    fn as_str(&self) -> &str {
        match self {
            ApiEndpoint::ToCelsius => "to-celsius",
            ApiEndpoint::ToFahrenheit => "to-fahrenheit",
        }
    }
}

#[derive(Debug)]
pub struct UnknownApiEndpoint(String);

impl std::fmt::Display for UnknownApiEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown API endpoint ({})", self.0)
    }
}

impl std::error::Error for UnknownApiEndpoint {}

impl std::str::FromStr for ApiEndpoint {
    type Err = UnknownApiEndpoint;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "to-celsius" => Ok(ApiEndpoint::ToCelsius),
            "to-fahrenheit" => Ok(ApiEndpoint::ToFahrenheit),
            _ => Err(UnknownApiEndpoint(s.to_string())),
        }
    }
}

impl ToSql for ApiEndpoint {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for ApiEndpoint {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let endpoint: Self = value
            .as_str()?
            .parse()
            .map_err(|err| FromSqlError::Other(Box::new(err)))?;

        Ok(endpoint)
    }
}

pub enum Query {
    RecordApiUsage {
        api_key: String,
        endpoint: ApiEndpoint,
        called_at: DateTime<Utc>,
    },
}

impl Query {
    pub async fn execute(self, connection_pool: &Pool) -> Result<(), Error> {
        let pool = connection_pool.clone();

        let conn = web::block(move || pool.get())
            .await?
            .map_err(error::ErrorInternalServerError)?;

        match self {
            Query::RecordApiUsage {
                api_key,
                endpoint,
                called_at,
            } => {
                let sql = "
                INSERT INTO usage (api_key, endpoint, called_at) 
                VALUES (?1, ?2, ?3);
                ";

                let mut stmt = conn
                    .prepare_cached(sql)
                    .map_err(error::ErrorInternalServerError)?;

                let _n_rows = stmt
                    .execute((api_key, endpoint, called_at))
                    .map_err(error::ErrorInternalServerError)?;

                Ok(())
            }
        }
    }
}
