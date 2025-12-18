use std::collections::HashMap;

use regex::Regex;

pub struct TestState {
    pub run_variables: HashMap<String, String>,
    pub env_variables: HashMap<String, String>,

    matcher: Regex,
}

impl TestState {
    pub fn new() -> Self {
        Self {
            run_variables: HashMap::new(),
            env_variables: HashMap::new(),
            matcher: Regex::new(r"\{\{\s*(.*?)\s*\}\}").expect("Failed to compile regex"),
        }
    }
    fn resolve(&self, key: &str) -> Option<&str> {
        if let Some(identifier) = key.strip_prefix("env.") {
            return self.env_variables.get(identifier).map(|v| v.as_str());
        }

        if let Some(identifier) = key.strip_prefix("run.") {
            return self.run_variables.get(identifier).map(|v| v.as_str());
        }

        if let Some(value) = self.run_variables.get(key) {
            Some(value.as_str())
        } else if let Some(value) = self.env_variables.get(key) {
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
        let mut state = TestState::new();

        state
            .env_variables
            .insert("one".to_string(), "Hello,_".to_string());

        state
            .run_variables
            .insert("two".to_string(), "World!".to_string());

        let input = r"{{one}}{{two}}".to_string();

        let output = state.substitute_values_in_text(&input);

        assert_eq!("Hello,_World!".to_string(), output);
    }
}
