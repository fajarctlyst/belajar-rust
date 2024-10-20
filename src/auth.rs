use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::iter::repeat_with;
use std::sync::{Arc, LazyLock, RwLock};

const API_KEYS_FILE: &str = "api-keys.txt";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

static API_KEYS: LazyLock<Arc<RwLock<HashSet<String>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashSet::new())));

pub fn create_api_key() -> String {
    repeat_with(fastrand::alphanumeric).take(40).collect()
}

pub fn load_api_keys() -> Result<()> {
    let stored_keys = std::fs::read_to_string(API_KEYS_FILE)?;

    let mut api_keys = API_KEYS.write()?;

    for line in stored_keys.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let token = line.trim().to_string();
        api_keys.insert(token);
    }

    Ok(())
}

pub fn store_api_key(api_key: &str) -> Result<()> {
    let mut f = std::fs::File::options()
        .create(true)
        .append(true)
        .open(API_KEYS_FILE)?;
    writeln!(f, "{api_key}")?;

    load_api_keys()
}

pub fn revoke_api_key(api_key: &str) -> Result<()> {
    {
        let mut api_keys = API_KEYS.write()?;
        api_keys.remove(api_key);
    }

    {
        let api_keys = API_KEYS.read()?;
        let mut f = std::fs::File::options().write(true).open(API_KEYS_FILE)?;
        for api_key in api_keys.iter() {
            writeln!(f, "{api_key}")?;
        }
    }

    load_api_keys()
}

pub fn is_key_allowed_access(token: &str) -> Result<bool> {
    let api_keys = API_KEYS.read()?;

    Ok(api_keys.contains(token))
}
