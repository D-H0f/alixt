use std::{collections::HashMap, sync::Arc};

use regex::Regex;

pub struct Global {
    pub env_variables: HashMap<String, String>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            env_variables: HashMap::new(),
        }
    }
}

pub struct RunState {
    pub run_variables: HashMap<String, String>,
    pub global: Arc<Global>,

    matcher: Regex,
}

impl RunState {
    pub fn new(global: Arc<Global>) -> Self {
        Self {
            run_variables: HashMap::new(),
            global,
            matcher: Regex::new(r"\{\{\s*(.*?)\s*\}\}").expect("Failed to compile regex"),
        }
    }
    fn resolve(&self, key: &str) -> Option<&str> {
        if let Some(identifier) = key.strip_prefix("env.") {
            return self.global.env_variables.get(identifier).map(|v| v.as_str());
        }

        if let Some(identifier) = key.strip_prefix("run.") {
            return self.run_variables.get(identifier).map(|v| v.as_str());
        }

        if let Some(value) = self.run_variables.get(key) {
            Some(value.as_str())
        } else if let Some(value) = self.global.env_variables.get(key) {
            Some(value.as_str())
        } else {
            None
        }
    }

    pub fn substitute_values_in_text(&self, input: &str) -> String {
        self.matcher
            .replace_all(input, |caps: &regex::Captures| {
                let key = &caps[1].trim();

                self.resolve(key).unwrap_or(&caps[0]).to_string()
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitution() {
        let mut global = Global::new();
        global
            .env_variables
            .insert("one".to_string(), "Hello,_".to_string());

        let mut state = RunState::new(Arc::new(global));


        state
            .run_variables
            .insert("two".to_string(), "World!".to_string());

        let input = r"{{one}}{{two}}".to_string();

        let output = state.substitute_values_in_text(&input);

        assert_eq!("Hello,_World!".to_string(), output);
    }
}
