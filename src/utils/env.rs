use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::models::error::AlixtError;

pub fn load_env_file(path: &Path) -> Result<HashMap<String, String>, AlixtError> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut vars: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?.trim().to_string();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let mut value = value.trim();

            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value = &value[1..value.len() - 1];
            }

            vars.insert(key.to_string(), value.to_string());
        }
    }
    Ok(vars)
}

pub fn capture_system_environment(
    env_map: &HashMap<String, String>,
    dot_env_vars: Option<HashMap<String, String>>,
) -> Result<HashMap<String, String>, AlixtError> {
    let mut captured_variables: HashMap<String, String> = HashMap::new();

    let local_vars = dot_env_vars.unwrap_or_default();

    for (internal_key, system_key) in env_map {
        let value = if let Some(v) = local_vars.get(system_key) {
            v.clone()
        } else {
            match std::env::var(system_key) {
                Ok(v) => v,
                Err(e) => {
                    return Err(AlixtError::Config(format!(
                        "Capture Failed: Required variable '{}', mapped to '{}', was not found in .env file or system environment\nERROR: {:#?}",
                        system_key, internal_key, e
                    )));
                }
            }
        };

        captured_variables.insert(internal_key.clone(), value);
    }

    Ok(captured_variables)
}
