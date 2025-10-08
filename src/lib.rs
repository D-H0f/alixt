use std::collections::HashMap;

use clap::{Parser, ValueEnum};
use reqwest::blocking::{self, Response};
use serde_json::Value;
use thiserror::Error;

pub mod template;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse Toml content")]
    Toml(#[from] toml::de::Error),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse JSON body")]
    Json(#[from] serde_json::Error),
}

#[derive(ValueEnum, Clone, Debug, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
use Method::*;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub runs: Vec<Run>,
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

    pub requests: Vec<Request>,
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

#[derive(Parser, Debug)]
pub struct Args {
    /// Generate a test_requests.toml template file to use
    #[arg(long)]
    pub generate_template: bool,

    /// Run requests from a .toml file instead of command-line flags.
    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Method::Get)]
    pub method: Method,

    #[arg(default_value = "0.0.0.0")]
    pub url: String,

    #[arg(short, long, default_value = "7878")]
    pub port: String,

    #[arg(short, long, default_value = "/")]
    pub target: String,

    /// The request body for POST or PUT requests
    #[arg(short, long)]
    pub body: Option<String>,
}

pub fn request(
    method: Method,
    headers: &Option<HashMap<String, String>>,
    url: &String,
    port: &String,
    target: &String,
    body: &Option<String>,
) -> Result<Response, Error> {
    let address = format!("http://{}:{}{}", url, port, target,);

    let client = blocking::Client::new();

    let mut request_builder = match method {
        Get => client.get(&address),
        Post => client.post(&address),
        Put => client.put(&address),
        Delete => client.delete(&address),
    };

    if let Some(hashmap) = headers {
        request_builder = hashmap
            .iter()
            .fold(request_builder, |builder, (key, value)| {
                builder.header(key, value)
            });
    }

    let request = match &body {
        Some(body) => request_builder.body(body.clone()),
        None => request_builder,
    };

    println!("[TEST]: Sending {:#?} to {url:#?}", &body);
    let response = request.send()?;

    Ok(response)
}

/// Holds the results for a un
struct Completed {
    tests: Vec<(String, bool, bool)>,
}

impl Completed {
    fn new() -> Self {
        Self { tests: vec![] }
    }
    fn add(&mut self, name: &str, passing: bool, breaking: bool) {
        let mut name = String::from(name);

        if name.len() > 20 {
            name = format!("{}...", &name[..20]);
        } else if name.is_empty() {
            panic!("test must have a name");
        }

        self.tests.push((name, passing, breaking));
    }
    fn print(&self) {
        println!("[API]: {} tests completed:", self.tests.len());
        self.tests.iter().for_each(|(name, passed, breaking)| {
            let grade = if *passed {
                "[PASS]".to_string()
            } else {
                "[FAIL]".to_string()
            };
            println!("{grade} {name}");
            if *breaking {
                println!("[API]: run stopped due to breaking assertion");
            }
        });
    }
}

enum TestResult {
    Pass(Option<Completed>),
    Fail(Option<Completed>),
}
use TestResult::*;

impl TestResult {
    fn is_pass(&self) -> bool {
        match self {
            Pass(_) => true,
            Fail(_) => false,
        }
    }
    fn is_fail(&self) -> bool {
        !self.is_pass()
    }
    fn unwrap(&mut self) -> Completed {
        match self {
            Pass(test) => test.take().unwrap(),
            Fail(test) => test.take().unwrap(),
        }
    }
}

/// Holds the results for all runs
pub struct RunResults {
    all: Vec<TestResult>,
}

impl RunResults {
    fn new() -> Self {
        Self { all: vec![] }
    }

    fn add(&mut self, test: TestResult) {
        self.all.push(test);
    }

    fn display_results(&mut self) {
        let passing = self
            .all
            .iter()
            .filter(|test| test.is_pass())
            .fold(0u16, |x, _| x + 1);
        let failing = self
            .all
            .iter()
            .filter(|test| test.is_fail())
            .fold(0u16, |x, _| x + 1);
        println!("\n\n\n\n[TEST]: All runs finished. {passing} passing, {failing} failing.");
        self.all.iter_mut().enumerate().for_each(|(index, test)| {
            let test = test.unwrap();

            println!("\n\n----[RUN]-{index}----\n");
            test.print();
        });
    }
}

pub fn parse_file(file: String) -> Result<bool, Error> {
    let content = std::fs::read_to_string(&file)?;
    let config: Config = toml::from_str(&content)?;

    let mut all_runs_passed = true;

    println!("[TEST]: Found {} runs", config.runs.len());

    let mut all_runs = RunResults::new();

    for run in config.runs {
        match execute_run(&run, &mut all_runs) {
            Ok(passed) => {
                if !passed {
                    all_runs_passed = passed;
                }
            }
            Err(e) => {
                println!("[ERROR] run {} encounterded an error: {e:#?}", &run.name);
            }
        }
    }

    all_runs.display_results();
    Ok(all_runs_passed)
}

pub fn execute_run(run: &Run, all_runs: &mut RunResults) -> Result<bool, Error> {
    println!(
        "\n\n----Run {} starting with {} requests----",
        &run.name,
        run.requests.len(),
    );

    let mut all_passed = true;

    let mut completed = Completed::new();

    for req in &run.requests {
        let mut passed = true;
        println!("\n-- Running Test: {} --", req.name);

        // Filtering requests to use specific arguments, or provided defaults.
        let mut method: Method = Get; // Defaults to GET
        #[allow(unused)]
        let mut headers: Option<HashMap<String, String>> = None; // Defaults to None
        #[allow(unused)]
        let mut url: String; // Panics if missing
        #[allow(unused)]
        let mut port: String; // Panics if missing
        let mut target: String = String::from("/"); // Defaults to root
        #[allow(unused)]
        let mut body: Option<String> = None; // Defualts to None

        if let Some(found) = &req.method {
            method = found.clone();
        } else if let Some(found) = &run.method {
            method = found.clone();
        }

        if let Some(found) = &req.headers {
            headers = Some(found.clone());
        } else {
            headers = run.headers.clone();
        }

        if let Some(found) = &req.url {
            url = found.to_string();
        } else if let Some(found) = &run.url {
            url = found.to_string();
        } else {
            panic!("Must have either a request url or default url in the .toml")
        }

        if let Some(found) = req.port {
            port = format!("{found}");
        } else if let Some(found) = run.port {
            port = format!("{found}");
        } else {
            panic!("Must have either a request port or default port in the .toml")
        }

        if let Some(found) = &req.target {
            target = found.to_string();
        } else if let Some(found) = &run.target {
            target = found.to_string();
        }

        if let Some(found) = &req.body {
            body = Some(found.to_string());
        } else {
            body = run.body.clone();
        }

        match request(method, &headers, &url, &port, &target, &body) {
            Ok(response) => {
                let status = response.status();
                let body_text = response.text()?;

                println!("[PASS]: '{}' returned a response '{}'", &req.name, status.as_str());

                if body_text.is_empty() {
                    println!("[API]: Response body is empty")
                } else {
                    println!(
                        "[API]: Response body is {:#?}",
                        serde_json::from_str::<Value>(&body_text)?,
                    );
                }

                if status.as_u16() != req.assert.status {
                    passed = false;
                    println!(
                        "[FAIL]: Expected status {}, got {}",
                        req.assert.status,
                        status.as_u16(),
                    );
                    if req.assert.breaking {
                        println!("[TEST]: assertion was breaking, run terminated.");
                        completed.add(&req.name, false, true);
                        all_passed = false;
                        break;
                    }
                }

                if let Some(expected) = &req.assert.body {
                    let expected_json: Value = serde_json::from_str(&expected)?;

                    if !body_text.is_empty() {
                        let body_json: Value = serde_json::from_str(&body_text)?;

                        if &expected_json != &body_json {
                            passed = false;
                            println!(
                                "[FAIL]: Expected {expected_json:#?}, got {:#?}",
                                &body_json
                            );
                            if req.assert.breaking {
                                println!("[TEST]: assertion was breaking, run terminated.");
                                completed.add(&req.name, false, true);
                                all_passed = false;
                                break;
                            }
                        } else {
                            println!("[PASS]: recieved correct body\n{:#?}", &body_json);
                        }
                    } else {
                        if expected.is_empty() {
                            println!("[PASS]: Both bodies are empty as expected");
                        } else {
                            passed = false;
                            println!(
                                "[FAIL]: Got empty body in response, expected:\n{:#?}",
                                &expected_json
                            );
                            if req.assert.breaking {
                                println!("[TEST]: assertion was breaking, run terminated.");
                                completed.add(&req.name, false, true);
                                all_passed = false;
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("[ERROR] Request '{}' failed {e:#}", &req.name);
                println!("[FATAL]: Halting test run due to request error");
                completed.add(&req.name, false, true);
                all_passed = false;
                break;
            }
        }
        completed.add(&req.name, passed, false);
        if !passed {
            all_passed = passed.clone();
        }
    }

    println!("---- End of Tests ----");

    if all_passed {
        all_runs.add(Pass(Some(completed)));
    } else {
        all_runs.add(Fail(Some(completed)));
    }

    Ok(all_passed)
}
