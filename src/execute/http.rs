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


use std::{str::FromStr, sync::Arc, time::Instant};

use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue}};
use serde_json::Value;

use crate::models::{context::{Global, RunState}, error::AlixtError, plan::{ExecuteRequest, RunPlan, TestPlan}, test_data::{RequestOutcome, RunData, TestData}};

pub async fn execute_test(client: &Client, plan: TestPlan, global: Arc<Global>) -> Result<TestData, AlixtError> {
    let mut test_outcome = TestData::new();
    for run in plan.runs {
        match execute_run(client, run, RunState::new(global.clone())).await {
            Ok(outcome) => test_outcome.run_data.push(outcome),
            Err(e) => return Err(e),
        };
    }
    Ok(test_outcome)
}

async fn execute_run(client: &Client, run: RunPlan, mut state: RunState) -> Result<RunData, AlixtError> {
    let mut run_outcome = RunData::new(run.name.clone());
    for request in run.requests {
        let outcome = execute_request(client, request, &mut state).await?;
        if !outcome.passing && outcome.breaking {
            run_outcome.outcomes.push(outcome);
            return Ok(run_outcome)
        }
        run_outcome.outcomes.push(outcome);
    }
    Ok(run_outcome)
}

async fn execute_request(client: &Client, request: ExecuteRequest, state: &mut RunState) -> Result<RequestOutcome, AlixtError> {
    let url = state.substitute_values_in_text(&request.url);
    let mut final_headers = HeaderMap::new();
    if let Some(headers) = &request.headers {
        for (key, value) in headers {
            let value = state.substitute_values_in_text(value);

            let header_name = HeaderName::from_str(key).map_err(|e| {
                AlixtError::Config(format!("Invalid Header Name '{}', {:#?}", key, e))
            })?;
            let header_value = HeaderValue::from_str(value.as_str()).map_err(|e| {
                AlixtError::Config(format!("Invalid header value for '{}: {}', {:#?}", key, value, e))
            })?;

            final_headers.insert(header_name, header_value);
        }
    }

    let mut builder = client.request(request.method.clone(), url.clone())
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
    
    if let Some(capture) = request.capture && let Some(json) = &json {
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
        passing: true,
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

        if outcome.status != Some(assert.status) {
            // assertion on expected status failed
            outcome.passing = false;
        }
        if assert.body.is_some() && outcome.response_body.is_none() {
            // asserts on a response body, but one was not returned
            outcome.passing = false;
        } else if let Some(assert_body) = assert.body {
            if let Some(outcome_json) = json {
                if let Ok(assert_json) = serde_json::from_str::<Value>(&assert_body) {
                    if assert_json != outcome_json {
                        // assert_body and outcome_body are both valid JSON, but do not match
                        outcome.passing = false;
                    }
                } else {
                    // outcome_body is valid JSON but assert_body is not
                    outcome.passing = false;
                }
            } else if let Some(outcome_body) = &outcome.response_body {
                // neither the assert body or the response body are valid JSON, but both exist,
                // so fall back to direct String comparison
                outcome.passing = *outcome_body == assert_body;
            }
        }

    }
    Ok(outcome)
}


// in place as a reminder to eventually add an initial capture run, and global_variables inside
// Global. execute_capture_run() will almost certainly be needed, but execute_capture_request may
// be redundant, wont know until we get there.
#[allow(unused)]
pub async fn execute_capture_run() {}
#[allow(unused)]
pub async fn execute_capture_request() {}
