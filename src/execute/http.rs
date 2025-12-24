// This file is part of alixt.
// Copyright (C) 2025 Devon Harley Offutt
//
// alixt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{collections::HashMap, str::FromStr, sync::Arc, time::Instant};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;

use crate::models::{
    config::Assert,
    context::{Global, RunState},
    error::AlixtError,
    plan::{ExecuteRequest, RunPlan, TestPlan},
    test_data::{AssertionOutcome, FailureType, RequestOutcome, RunData, TestData},
};

pub async fn execute_test(
    client: &Client,
    plan: TestPlan,
    global: Arc<Global>,
) -> Result<TestData, AlixtError> {
    let mut test_outcome = TestData::new();
    for run in plan.runs {
        match execute_run(client, run, RunState::new(global.clone())).await {
            Ok(outcome) => test_outcome.run_data.push(outcome),
            Err(e) => return Err(e),
        };
    }
    Ok(test_outcome)
}

async fn execute_run(
    client: &Client,
    run: RunPlan,
    mut state: RunState,
) -> Result<RunData, AlixtError> {
    let mut run_outcome = RunData::new(run.name.clone());
    for request in run.requests {
        let outcome = execute_request(client, request, &mut state).await?;
        if !outcome.passing.is_passing() && outcome.breaking {
            run_outcome.outcomes.push(outcome);
            return Ok(run_outcome);
        }
        run_outcome.outcomes.push(outcome);
    }
    Ok(run_outcome)
}

async fn execute_request(
    client: &Client,
    request: ExecuteRequest,
    state: &mut RunState,
) -> Result<RequestOutcome, AlixtError> {
    let url = state.substitute_values_in_text(&request.url);
    let mut final_headers = HeaderMap::new();
    if let Some(headers) = &request.headers {
        for (key, value) in headers {
            let value = state.substitute_values_in_text(value);

            let header_name = HeaderName::from_str(key).map_err(|e| {
                AlixtError::Config(format!("Invalid Header Name '{}', {:#?}", key, e))
            })?;
            let header_value = HeaderValue::from_str(value.as_str()).map_err(|e| {
                AlixtError::Config(format!(
                    "Invalid header value for '{}: {}', {:#?}",
                    key, value, e
                ))
            })?;

            final_headers.insert(header_name, header_value);
        }
    }

    let mut builder = client
        .request(request.method.clone(), url.clone())
        .headers(final_headers);

    if let Some(text) = request.body {
        let body = state.substitute_values_in_text(text.as_str());
        builder = builder.body(body);
    }

    let start = Instant::now();
    let response = builder.send().await?;
    let duration = start.elapsed();

    let status = response.status();
    let body_text = response.text().await?;

    let json: Option<Value> = if !body_text.is_empty() {
        serde_json::from_str(&body_text).ok()
    } else {
        None
    };

    if let Some(capture) = request.capture
        && let Some(json) = &json
    {
        for (variable, pattern) in capture {
            let value = if pattern.starts_with('/') {
                json.pointer(&pattern)
            } else {
                json.get(pattern)
            };

            if let Some(value) = value {
                let value_string = match value {
                    Value::String(s) => s.clone(),
                    v => v.to_string(),
                };
                state.run_variables.insert(variable, value_string);
            }
        }
    }

    let mut outcome = RequestOutcome {
        name: request.name,
        method: request.method.to_string(),
        url,
        passing: AssertionOutcome::Passed,
        breaking: false,
        status: Some(status.as_u16()),
        response_body: if !body_text.is_empty() {
            Some(body_text)
        } else {
            None
        },
        duration,
    };

    if let Some(assert) = request.assert {
        outcome.breaking = assert.breaking;
        outcome.passing = assert_response(json.as_ref(), &assert, outcome.status);
    }
    Ok(outcome)
}

fn assert_response(
    body_json: Option<&Value>,
    assertions: &Assert,
    status: Option<u16>,
) -> AssertionOutcome {
    let mut outcome = AssertionOutcome::Passed;
    if let Some(expected) = assertions.status {
        if status.map_or(true, |num| num != expected) {
            outcome.push(FailureType::StatusMismatch {
                expected,
                found: status,
            })
        }
    }

    // if there are no json assertions, early return
    if assertions.body_matches.is_none()
        && assertions.subset_matches.is_none()
        && assertions.subset_includes.is_none()
        && assertions.subset_regex.is_none()
    {
        return outcome;
    }

    // if json is invalid, early return with err
    let Some(json) = body_json else {
        outcome.push(FailureType::InvalidJson());
        return outcome;
    };

    let check_subset = |expected: &HashMap<String, Value>,
                        outcome: &mut AssertionOutcome,
                        regex: bool| {
        for (path, expected_value) in expected {
            // IMMUTABLE REF CLONED FROM ENV SCOPE HERE
            match json.pointer(path) {
                Some(Value::String(value)) if regex => {
                    let Some(pattern) = expected_value.as_str() else {
                        outcome.push(FailureType::JsonRegexMismatch {
                            path: path.clone(),
                            pattern: expected_value.to_string(),
                            found: "Invalid Regex Syntax".to_string(),
                        });
                        continue;
                    };
                    let Ok(re) = regex::Regex::new(pattern) else {
                        outcome.push(FailureType::JsonRegexMismatch {
                            path: path.clone(),
                            pattern: pattern.to_string(),
                            found: "Invalid Regex Syntax".to_string(),
                        });
                        continue;
                    };

                    if !re.is_match(value) {
                        outcome.push(FailureType::JsonRegexMismatch {
                            path: path.clone(),
                            pattern: pattern.to_string(),
                            found: value.clone(),
                        });
                    }
                }
                Some(_) if regex => outcome.push(FailureType::JsonNotString { path: path.clone() }),
                Some(value) => {
                    if expected_value.as_str() == Some("*") {
                        continue;
                    } else {
                        if expected_value != value {
                            outcome.push(FailureType::JsonValueMismatch {
                                path: path.clone(),
                                expected: format!("{expected_value}"),
                                found: format!("{value}"),
                            })
                        }
                    }
                }
                None => outcome.push(FailureType::JsonMissingField { path: path.clone() }),
            }
        }
    };

    if let Some(expected) = assertions.body_matches.as_ref() {
        let found_paths = flatten_json(json);

        for path in &found_paths {
            if !expected.contains_key(path) {
                outcome.push(FailureType::JsonExtraField { path: path.clone() });
            }
        }
        check_subset(expected, &mut outcome, false);
    }
    if let Some(expected) = assertions.subset_matches.as_ref() {
        check_subset(expected, &mut outcome, false);
    }
    if let Some(expected) = assertions.subset_includes.as_ref() {
        for path in expected {
            match json.pointer(path) {
                Some(_) => {}
                None => outcome.push(FailureType::JsonMissingField { path: path.clone() }),
            }
        }
    }
    if let Some(expected) = assertions.subset_regex.as_ref() {
        check_subset(expected, &mut outcome, true);
    }
    outcome
}

fn flatten_json(root: &Value) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    let mut stack: Vec<(&Value, String)> = vec![(root, "".to_string())];

    while let Some((node, prefix)) = stack.pop() {
        match node {
            Value::Object(obj) => {
                for (key, value) in obj {
                    let new_prefix = format!("{}/{}", prefix, key);
                    stack.push((value, new_prefix));
                }
            }
            Value::Array(arr) => {
                for (index, value) in arr.iter().enumerate() {
                    let new_prefix = format!("{}/{}", prefix, index);
                    stack.push((value, new_prefix));
                }
            }
            _ => {
                let key = if prefix.is_empty() {
                    "/".to_string()
                } else {
                    prefix
                };
                paths.push(key);
            }
        }
    }
    paths
}

// in place as a reminder to eventually add an initial capture run, and global_variables inside
// Global. execute_capture_run() will almost certainly be needed, but execute_capture_request may
// be redundant, wont know until we get there.
#[allow(unused)]
pub async fn execute_capture_run() {}

pub async fn execute_capture_request(
    client: &Client,
    request: &ExecuteRequest,
    global: &mut Global,
) -> Result<(), AlixtError> {
    let url = global.substitute_values_in_text(&request.url);
    let mut final_headers = HeaderMap::new();
    if let Some(headers) = &request.headers {
        for (key, value) in headers {
            let value = global.substitute_values_in_text(value);

            let header_name = HeaderName::from_str(key).map_err(|e| {
                AlixtError::Config(format!("Invalid Header Name '{}', {:#?}", key, e))
            })?;
            let header_value = HeaderValue::from_str(value.as_str()).map_err(|e| {
                AlixtError::Config(format!(
                    "Invalid header value for '{}: {}', {:#?}",
                    key, value, e
                ))
            })?;

            final_headers.insert(header_name, header_value);
        }
    }

    let mut builder = client
        .request(request.method.clone(), url.clone())
        .headers(final_headers);

    if let Some(text) = &request.body {
        let body = global.substitute_values_in_text(text.as_str());
        builder = builder.body(body);
    }

    let response = builder.send().await?;

    let Some(capture) = request.capture.as_ref() else {
        return Ok(());
    };

    // if there is no capture, it doesnt matter what the response body is, but if there is a
    // capture block, the response MUST be valid JSON by this point in order to extract values
    let response = response.json::<Value>().await?;

    for (variable, pattern) in capture {
        let value = if pattern.starts_with('/') {
            response.pointer(pattern)
        } else {
            response.get(pattern)
        };

        // unlike with regular requests, capture variables must be resolved in the response body
        let Some(value) = value else {
            return Err(AlixtError::Config(format!(
                "pattern '{}' not found in response body for request '{}'",
                pattern, request.name
            )));
        };

        let value_string = match value {
            Value::String(s) => s.clone(),
            v => v.to_string(),
        };
        global.variables.insert(variable.to_string(), value_string);
    }
    Ok(())
}
