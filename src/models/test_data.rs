use std::time::Duration;

pub struct TestData {
    pub run_data: Vec<RunData>,
}

impl TestData {
    pub fn new() -> Self {
        Self {
            run_data: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RunData {
    pub name: String,
    pub outcomes: Vec<RequestOutcome>,
}

impl RunData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            outcomes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RequestOutcome {
    pub name: String,
    pub method: String,
    pub url: String,

    pub passing: bool,
    pub breaking: bool,

    pub status: Option<u16>,
    pub response_body: Option<String>,
    pub duration: Duration,
}
