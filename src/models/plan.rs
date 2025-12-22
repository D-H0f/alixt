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


use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::models::{config::{Assert, Config, Request, Run, Scheme}, error::AlixtError};

use crate::models::config::Method as ConfigMethod;
use reqwest::Method;


#[derive(Default)]
pub struct TestPlan {
    pub capture: Option<CapturePlan>,
    pub runs: Vec<RunPlan>,
}

impl TestPlan {
    pub fn new() -> Self {
        Self {
            capture: None,
            runs: Vec::new(),
        }
    }

    pub fn from_config(config: Config, working_dir: &Path) -> Result<Self, AlixtError> {
        let mut config = config;
        let mut plan = TestPlan::new();
        plan.capture = CapturePlan::from_config(&mut config, working_dir)?;
        let config = config;

        for run in config.run {
            plan.runs.push(RunPlan::from_run(run)?);
        }
        Ok(plan)
    }
}

pub struct CapturePlan {
    pub env_file: Option<PathBuf>,
    pub environment_variables: Option<HashMap<String, String>>,
    pub requests: Option<Vec<ExecuteRequest>>,
}

impl CapturePlan {
    fn from_config(config: &mut Config, working_dir: &Path) -> Result<Option<Self>, AlixtError> {
        let Some(mut capture) = config.capture.take() else {
            return Ok(None);
        };

        let resolved_env_path = if let Some(path) = capture.env_file {
            let full_path = working_dir.join(&path);

            if !full_path.exists() {
                return Err(AlixtError::Config(format!("Environment file not found: {:?}\n(Looked in {:?})", path, full_path)));
            }
            Some(full_path)
        } else {
            None
        };

        let requests = if let Some(requests) = capture.request {
            let mut reqs = Vec::new();

            for request in requests {
                reqs.push(ExecuteRequest {
                    name: request.name.unwrap_or("".to_string()),
                    url: ExecuteRequest::_format_url(request.scheme, request.host, request.port, request.path),
                    method: ExecuteRequest::_convert_method(request.method),
                    body: request.body,
                    headers: request.headers,
                    capture: request.capture,
                    assert: None,
                });
            }
            Some(reqs)
        } else {
            None
        };
        Ok(Some(CapturePlan {
            env_file: resolved_env_path,
            environment_variables: capture.environment_variables.take(),
            requests,
        }))
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
    
    fn from_run(run: Run) -> Result<RunPlan, AlixtError> {
        let mut run_plan = RunPlan::new(run.name);

        for mut request in run.request {
            // inheritance checks
            if request.headers.is_none() && run.headers.is_some() {
                request.headers = run.headers.clone();
            }
            if request.method.is_none() {
                if run.method.is_none() {
                    return Err(AlixtError::Config(format!(
                        "No default or explicit method present for {}",
                        request.name
                    )));
                }
                request.method = run.method.clone();
            }
            if request.scheme.is_none() {
                if run.scheme.is_none() {
                    return Err(AlixtError::Config(format!(
                        "No default or explicit scheme present for {}",
                        request.name
                    )));
                }
                request.scheme = run.scheme.clone();
            }
            if request.host.is_none() {
                if run.host.is_none() {
                    return Err(AlixtError::Config(format!(
                        "No default or explicit host present for {}",
                        request.name
                    )));
                }
                request.host = run.host.clone();
            }
            if request.port.is_none() {
                request.port = run.port;
            }
            if request.path.is_none() {
                // if neither the request nor run specify a path, it defaults to "/"
                request.path = run.path.clone();
            }
            if request.body.is_none() {
                request.body = run.body.clone();
            }

            run_plan.requests.push(ExecuteRequest::from_request(request)?);
        }

        Ok(run_plan)
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

impl ExecuteRequest {
    fn from_request(request: Request) -> Result<ExecuteRequest, AlixtError> {
        let Some(host) = request.host else {
            return Err(AlixtError::Config(format!("Internal Error: Request {} missing host, got past checks", request.name)));
        };
        let Some(method) = request.method else {
            return Err(AlixtError::Config(format!("Internal Error: Request {} missing method, got past checks", request.name)));
        };

        let method = Self::_convert_method(method);

        let request_plan = ExecuteRequest {
            name: request.name,
            url: Self::_format_url(request.scheme.unwrap_or(Scheme::Http), host, request.port, request.path),
            method,
            body: request.body,
            headers: request.headers,
            capture: request.capture,
            assert: request.assert,
        };
        Ok(request_plan)
    }
    fn _convert_method(method: ConfigMethod) -> Method {
        match method {
            ConfigMethod::Get => Method::GET,
            ConfigMethod::Put => Method::PUT,
            ConfigMethod::Post => Method::POST,
            ConfigMethod::Patch => Method::PATCH,
            ConfigMethod::Delete => Method::DELETE,
        }
    }

    fn _format_url(scheme: Scheme, host: String, port: Option<u16>, path: Option<String>) -> String {
        let mut url = format!("{}://{}", scheme, host);
        if let Some(port) = port {
            url = format!("{}:{}", url, port);
        }
        let path = if let Some(path) = path {
            path
        } else {
            "/".to_string()
        };
        format!("{url}{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_inheritance() {
        let toml_input = r#"
        [[run]]
        name = "Base Run"
        method = "Get"
        scheme = "Http"
        host = "0.0.0.0"
        port = 4848
        path = "/hello/world"

        [[run.request]]
        name = "Inherit from run"

        [[run.request]]
        name = "Overrides everything"
        method = "Post"
        scheme = "Https"
        host = "api.example.com"
        port = 80
        path = "/a/new/path"
        "#;
        let config: Config = match toml::from_str(toml_input) {
            Ok(config) => config,
            Err(e) => panic!("ERROR: {e:#?}"),
        };

        let plan = match TestPlan::from_config(config, Path::new(".")) {
            Ok(plan) => plan,
            Err(e) => panic!("ERROR: {e:#?}"),
        };

        let req_inherit = &plan.runs[0].requests[0];
        assert_eq!(req_inherit.url, "http://0.0.0.0:4848/hello/world");
        assert_eq!(req_inherit.method, reqwest::Method::GET);

        let req_override = &plan.runs[0].requests[1];
        assert_eq!(req_override.url, "https://api.example.com:80/a/new/path");
        assert_eq!(req_override.method, reqwest::Method::POST);
    }
}
