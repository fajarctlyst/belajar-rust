use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::iter::repeat_with;
use std::sync::{Arc, LazyLock, RwLock};

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

pub fn load_tokens() -> Result<()> {
    let stored_tokens = std::fs::read_to_string(TOKENS_FILE)?;

    let mut tokens = TOKENS.write()?;

    for line in stored_tokens.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let token = line.trim().to_string();
        tokens.insert(token);
    }

    Ok(())
}

pub fn store_token(token: &str) -> Result<()> {
    let mut f = std::fs::File::options()
        .create(true)
        .append(true)
        .open(TOKENS_FILE)?;
    write!(f, "{token}{NEWLINE}")?;

    load_tokens()
}

pub fn revoke_token(token: &str) -> Result<()> {
    {
        let mut tokens = TOKENS.write()?;
        tokens.remove(token);
    }

    {
        let tokens = TOKENS.read()?;
        let mut f = std::fs::File::options().write(true).open(TOKENS_FILE)?;
        for token in tokens.iter() {
            write!(f, "{token}{NEWLINE}")?;
        }
    }

    load_tokens()
}

pub fn is_token_allowed_access(token: &str) -> Result<bool> {
    let tokens = TOKENS.read()?;

    Ok(tokens.contains(token))
}
