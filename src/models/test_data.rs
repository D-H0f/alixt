use std::time::Duration;

pub struct TestData {
    pub run_data: Vec<RunData>,
}

pub struct RunData {
    pub name: String,
    pub outcomes: Vec<RequestOutcome>,
}

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
