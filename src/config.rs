use serde::Deserialize;
use std::collections::HashMap;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub run: Vec<Run>,
}

// holds multiple requests, contents are blocking
#[derive(Deserialize, Debug)]
pub struct Run {
    pub name: String,
    // These fields are the defaults for all requests in the run
    pub method: Option<Method>,
    pub url: Option<String>,
    pub port: Option<u16>,
    pub target: Option<String>,
    pub body: Option<String>,

    pub request: Vec<Request>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub method: Option<Method>,
    pub url: Option<String>,
    pub port: Option<u16>,
    pub target: Option<String>,
    pub body: Option<String>,

    pub headers: Option<HashMap<String, String>>,
    pub assert: Assert,
}

#[derive(Deserialize, Debug)]
pub struct Assert {
    pub status: u16,
    pub breaking: bool,
    pub body: Option<String>,
}
