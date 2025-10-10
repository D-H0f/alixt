use std::collections::HashMap;
use std::io::Write;

use clap::Parser;
use reqwest::blocking::{self, Response};
use serde_json::Value;
use thiserror::Error;

pub mod template;
mod config;

pub use config::*;
use Method::*;

mod results;

pub use results::*;

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

    #[error("Missing url")]
    MissingUrl,

    #[error("Missing port")]
    MissingPort,
}


#[derive(Parser, Debug)]
pub struct Args {
    /// Generate a test_requests.toml template file to use
    #[arg(long)]
    pub generate_template: bool,

    /// Run requests from a .toml file instead of command-line flags.
    #[arg(short, long)]
    pub file: Option<String>,

    /// displays more detailed output for each request
    #[arg(short, long)]
    pub verbose: bool,

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

pub fn request<W: Write>(
    writer: &mut W,
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

    writeln!(writer, "[TEST]: Sending {:#?} to {url:#?}", &body)?;
    let response = request.send()?;

    Ok(response)
}
pub fn parse_file<W: Write>(writer: &mut W, file: String) -> Result<AllRuns, Error> {
    let content = std::fs::read_to_string(&file)?;
    let config: Config = toml::from_str(&content)?;

    writeln!(writer, "[TEST]: Found {} runs", config.run.len())?;

    let mut all_run_data = AllRuns::new();

    for run in config.run {
        if let Err(e) = execute_run(writer, &run, &mut all_run_data) {
            writeln!(
                writer,
                "[ERROR] run {} encounterded an error: {e:#?}",
                &run.name
            )?;
        }
    }

    all_run_data.display_results(writer)?;
    Ok(all_run_data)
}

pub fn execute_run<W: Write>(
    writer: &mut W,
    run: &Run,
    all_run_data: &mut AllRuns,
) -> Result<(), Error> {
    writeln!(
        writer,
        "\n\n----Run {} starting with {} requests----",
        &run.name,
        run.request.len(),
    )?;


    let mut current_run = RunData::new(run.name.clone());

    for req in &run.request {
        let mut passed = true;
        writeln!(writer, "\n-- Running Test: {} --", req.name)?;

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
            return Err(Error::MissingUrl);
        }

        if let Some(found) = req.port {
            port = format!("{found}");
        } else if let Some(found) = run.port {
            port = format!("{found}");
        } else {
            return Err(Error::MissingPort);
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

        match request(writer, method, &headers, &url, &port, &target, &body) {
            Ok(response) => {
                let status = response.status();
                let body_text = response.text()?;

                writeln!(
                    writer,
                    "[PASS]: '{}' returned a response '{}'",
                    &req.name,
                    status.as_str()
                )?;

                if body_text.is_empty() {
                    writeln!(writer, "[API]: Response body is empty")?;
                } else {
                    writeln!(
                        writer,
                        "[API]: Response body is {:#?}",
                        serde_json::from_str::<Value>(&body_text)?,
                    )?;
                }

                if status.as_u16() != req.assert.status {
                    passed = false;
                    writeln!(
                        writer,
                        "[FAIL]: Expected status {}, got {}",
                        req.assert.status,
                        status.as_u16(),
                    )?;
                    if req.assert.breaking {
                        writeln!(writer, "[TEST]: assertion was breaking, run terminated.")?;
                        current_run.add(&req.name, false, true);
                        break;
                    }
                }

                if let Some(expected) = &req.assert.body {
                    let expected_json: Value = serde_json::from_str(expected)?;

                    if !body_text.is_empty() {
                        let body_json: Value = serde_json::from_str(&body_text)?;

                        if expected_json != body_json {
                            passed = false;
                            writeln!(
                                writer,
                                "[FAIL]: Expected {expected_json:#?}, got {:#?}",
                                &body_json
                            )?;
                            if req.assert.breaking {
                                writeln!(
                                    writer,
                                    "[TEST]: assertion was breaking, run terminated."
                                )?;
                                current_run.add(&req.name, false, true); 
                                break;
                            }
                        } else {
                            writeln!(writer, "[PASS]: recieved correct body\n{:#?}", &body_json)?;
                        }
                    } else if expected.is_empty() {
                        writeln!(writer, "[PASS]: Both bodies are empty as expected")?;
                    } else {
                        passed = false;
                        writeln!(
                            writer,
                            "[FAIL]: Got empty body in response, expected:\n{:#?}",
                            &expected_json
                        )?;
                        if req.assert.breaking {
                            writeln!(writer, "[TEST]: assertion was breaking, run terminated.")?;
                            current_run.add(&req.name, false, true);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                writeln!(writer, "[ERROR] Request '{}' failed {e:#}", &req.name)?;
                writeln!(writer, "[FATAL]: Halting test run due to request error")?;
                current_run.add(&req.name, false, true);
                break;
            }
        }
        current_run.add(&req.name, passed, false);
    }
    writeln!(writer, "---- End of Tests ----")?;
    all_run_data.add(current_run);
    Ok(())
}
