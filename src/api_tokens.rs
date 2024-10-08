use std::collections::HashSet;
use std::error::Error;
use std::iter::repeat_with;
use std::sync::{Arc, LazyLock, RwLock};

use actix_web::web;

use crate::db;

#[cfg(target_os = "windows")]
const NEWLINE: &str = "\r\n";
#[cfg(not(target_os = "windows"))]
const NEWLINE: &str = "\n";

const TOKENS_FILE: &str = "tokens.txt";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

static TOKENS: LazyLock<Arc<RwLock<HashSet<String>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashSet::new())));

pub fn create_token() -> String {
    repeat_with(fastrand::alphanumeric).take(40).collect()
}

pub fn load_tokens(database: web::Data<db::Pool>) -> Result<()> {
    let conn = database
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut stmt = conn.prepare(
        "
        SELECT  api_key
        FROM    api_keys
        WHERE   revoked_at IS NOT NULL
    ;",
    )?;

    let rows = stmt.query_map((), |row| row.get(0))?;

    let mut tokens: std::sync::RwLockWriteGuard<'_, HashSet<String>> = TOKENS.write()?;

    for row in rows {
        tokens.insert(row?);
    }

    Ok(())
}

pub async fn store_token(database: web::Data<db::Pool>, token: String) -> Result<()> {
    let query = db::Query::RevokeApiKey(token);
    query.execute(database.clone()).await?;

    load_tokens(database.clone())
}

pub async fn revoke_token(database: web::Data<db::Pool>, token: String) -> Result<()> {
    let query = db::Query::RevokeApiKey(token);
    query.execute(database.clone()).await?;

    load_tokens(database.clone())
}

pub fn is_token_allowed_access(token: &str) -> Result<bool> {
    let tokens = TOKENS.read()?;

    Ok(tokens.contains(token))
}
