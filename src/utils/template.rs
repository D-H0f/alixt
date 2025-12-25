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


#![allow(unused)]
use std::path::PathBuf;
use std::{collections::HashMap, io::Write};

use serde_json::Value;

use crate::models::config::*;
use crate::models::error::AlixtError;

const PRETTY_TEMPLATE: &str = r#"
[capture]
env_file = "./secrets/.env"

  [capture.environment_variables]
  host = "FORGEJO_HOST"

  [[capture.request]]
  name = "Get Forgejo Version"
  method = "get"
  scheme = "https"
  host = "{{host}}"
  port = 7878
  path = "/api/v1/version"

    [capture.request.headers]
    Accept = "application/json"

    [capture.request.capture]
    forgejo_version = "/version"

[[run]]
name = "Example Test Configuration"
method = "get"
scheme = "https"
host = "{{host}}"
port = 7878

  [run.headers]
  Content-Type = "application/json"

  [[run.request]]
  name = "Confirm Forgejo Version"
  path = "/api/v1/version"

    # you can override the run level defaults, if you want
    [run.request.headers]
    Accept = "application/json"

    [run.request.assert]
    breaking = true
    status = 200
    subset_includes = ["/version"]

    [run.request.assert.body_matches]
    "/version" = "13.0.2+gitea-1.22.0"

    [run.request.assert.subset_matches]
    "/version" = "{{forgejo_version}}"

    [run.request.assert.subset_regex]
    "/version" = '^\d+\..*gitea.*$'

  [[run.request]]
  name = "Some Random Example"
  method = "post"
  scheme = "https"
  path = "/signup"
  body = """
  {
      "name": "Doug Walker",
      "username": "digdug",
      "password": "password123",
      "email": "exapmle@example.com"
  }
  """
"#;


pub fn generate_pretty<W: Write>(writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "{}", PRETTY_TEMPLATE)
}


pub fn generate<W: Write>(writer: &mut W) -> Result<(), AlixtError> {
    let mut capture = Capture {
        env_file: Some(PathBuf::from("./secrets/.env")),
        environment_variables: None,
        request: None,
    };
    let mut environment_variables = HashMap::<String, String>::new();
    environment_variables.insert("host".to_string(), "FORGEJO_HOST".to_string());
    capture.environment_variables = Some(environment_variables);
    let mut capture_request = CaptureRequest {
        name: Some("Get Forgejo Version".to_string()),
        headers: None,
        method: Method::Get,
        scheme: Scheme::Https,
        host: "{{host}}".to_string(),
        port: Some(7878),
        path: Some("/api/v1/version".to_string()),
        body: None,
        capture: None,
    };
    let mut capture_headers = HashMap::<String, String>::new();
    capture_headers.insert("Accept".to_string(), "application/json".to_string());
    capture_request.headers = Some(capture_headers.clone());
    let mut capture_map = HashMap::<String, String>::new();
    capture_map.insert("forgejo_version".to_string(), "/version".to_string());
    capture_request.capture = Some(capture_map);
    capture.request = Some(vec![capture_request]);


    let mut run_headers = HashMap::new();
    run_headers.insert("Content-Type".to_string(), "application/json".to_string());
    let mut login_capture = HashMap::new();
    login_capture.insert("auth_token".to_string(), "token".to_string());

    let login_body = r#"{
    "username": "my_username",
    "password": "my_password"
}
"#;
    let request_body = r#"{
    "name": "Doug Walker",
    "username": "digdug",
    "password": "password123",
    "email": "exapmle@example.com"
}
"#;
    let mut body_matches = HashMap::<String, Value>::new();
    body_matches.insert("/version".to_string(), Value::String("13.0.2+gitea-1.22.0".to_string()));

    let mut subset_matches = HashMap::<String, Value>::new();
    subset_matches.insert("/version".to_string(), Value::String("{{forgejo_version}}".to_string()));

    let mut subset_regex = HashMap::<String, Value>::new();
    subset_regex.insert("/version".to_string(), Value::String(r#"^\d+\..*gitea.*$"#.to_string()));

    let login_run = Run {
        name: "Example Test Configuration".to_string(),
        headers: Some(run_headers.clone()),
        method: Some(Method::Get),
        scheme: Some(Scheme::Http),
        host: Some("{{host}}".to_string()),
        port: Some(7878),
        path: None,
        body: None,
        request: vec![
            Request {
                name: "Confirm Forgejo Version".to_string(),
                headers: Some(capture_headers),
                method: Some(Method::Get),
                scheme: Some(Scheme::Https),
                host: None,
                port: None,
                path: Some("/api/v1/version".to_string()),
                body: None,
                capture: None,
                assert: Some(Assert {
                    breaking: true,
                    status: Some(200),
                    body_matches: Some(body_matches),
                    subset_matches: Some(subset_matches),
                    subset_includes: Some(vec!["/version".to_string()]),
                    subset_regex: Some(subset_regex),
                }),
            },
            Request {
                name: "Some Random Example".to_string(),
                headers: Some(run_headers),
                method: Some(Method::Get),
                scheme: Some(Scheme::Https),
                host: None,
                port: None,
                path: Some("/api".to_string()),
                body: Some(request_body.to_string()),
                capture: None,
                assert: None,
            },
        ],
    };

    let mut environment_variables = HashMap::new();
    environment_variables.insert("API_KEY".to_string(), "SYS_API_KEY_V1".to_string());
    environment_variables.insert("DB_URL".to_string(), "SYS_DATABASE_URL_V1".to_string());
    let environment_variables = Some(environment_variables);

    let config = Config {
        capture: Some(/*Capture {
            env_file: Some(PathBuf::from("./secrets.env")),
            environment_variables,
            request: Some(vec![
                CaptureRequest {
                    name: Some("Get initial login token".to_string()),
                    headers: Some(run_headers),
                    method: Method::Post,
                    scheme: Scheme::Http,
                    host: "0.0.0.0".to_string(),
                    port: Some(7878u16),
                    path: Some("/login".to_string()),
                    body: Some(login_body.to_string()),
                    capture: Some(login_capture),
                }
            ]),
        }*/capture),
        run: vec![login_run],
    };

    let toml_string = toml::to_string_pretty(&config)?;

    write!(writer, "{toml_string}")?;

    Ok(())
}
