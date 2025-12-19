use std::collections::HashMap;

use reqwest::Method;

use crate::models::config::Assert;


pub struct TestPlan {
    pub runs: Vec<RunPlan>,
}

impl TestPlan {
    pub fn new() -> Self {
        Self {
            runs: Vec::new()
        }
    }
}

pub struct RunPlan {
    pub name: String,
    pub requests: Vec<ExecuteRequest>,
}

impl RunPlan {
    pub fn new(name: String) -> Self {
        Self {
            name,
            requests: Vec::new()
        }
    }
}

pub struct ExecuteRequest {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub body: Option<String>,

    pub headers: Option<HashMap<String, String>>,
    pub capture: Option<HashMap<String, String>>,
    pub assert: Option<Assert>
}
