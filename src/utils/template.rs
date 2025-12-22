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


use std::path::PathBuf;
use std::{collections::HashMap, io::Write};

use crate::models::config::*;
use crate::models::error::AlixtError;

const PRETTY_TEMPLATE: &str = r#"
# --- Initial Capture Run ---
[capture]
  env_file = "./secrets.env"

  [capture.environment_variables]
    API_KEY = "SYS_API_KEY_V1"
    DB_URL = "SYS_DATABASE_URL_V1"

  [[capture.request]]
    name = "Get initial login token"
    method = "post"
    scheme = "http"
    host = "0.0.0.0"
    port = 7878
    path = "/login"
    body = """
{
    "username": "my_username",
    "password": "my_password"
}
"""

  [capture.request.headers]
    Content-Type = "application/json"

  [capture.request.capture]
    auth_token = "token"

# --- Run Tests ---
[[run]]
  name = "Example Test Configuration"
  method = "get"
  scheme = "http"
  host = "0.0.0.0"
  port = 7878

  [run.headers]
    Content-Type = "application/json"

  [[run.request]]
    name = "Get Authentication Token"
    method = "post"
    path = "/login"
    body = """
{
    "username": "my_username",
    "password": "my_password"
}
"""

  [run.request.capture]
    auth_token = "token"

  [[run.request]]
    name = "Use Captured Auth Token"
    method = "post"
    scheme = "https"
    path = "/accounts"
    body = """
{
    "name": "Doug Walker",
    "username": "digdug",
    "password": "password123",
    "email": "exapmle@example.com"
}
"""

  [run.request.headers]
    Authorization = "Bearer {{auth_token}}"
    Content-Type = "application/json"

  [run.request.assert]
    status = 200
    breaking = true
    body = """
{
    "id": 2
}
"""
"#;
pub fn generate_pretty<W: Write>(writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "{}", PRETTY_TEMPLATE)
}


pub fn generate<W: Write>(writer: &mut W) -> Result<(), AlixtError> {
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

    let mut request_headers = HashMap::new();
    request_headers.insert("Content-Type".to_string(), "application/json".to_string());
    request_headers.insert(
        "Authorization".to_string(),
        "Bearer {{auth_token}}".to_string(),
    );

    let login_run = Run {
        name: "Example Test Configuration".to_string(),
        headers: Some(run_headers.clone()),
        method: Some(Method::Get),
        scheme: Some(Scheme::Http),
        host: Some("0.0.0.0".to_string()),
        port: Some(7878),
        path: None,
        body: None,
        request: vec![
            Request {
                name: "Get Authentication Token".to_string(),
                headers: None,
                method: Some(Method::Post),
                scheme: None,
                host: None,
                port: None,
                path: Some("/login".to_string()),
                body: Some(login_body.to_string()),
                capture: Some(login_capture.clone()),
                assert: None,
            },
            Request {
                name: "Use Captured Auth Token".to_string(),
                headers: Some(request_headers),
                method: Some(Method::Post),
                scheme: Some(Scheme::Https),
                host: None,
                port: None,
                path: Some("/accounts".to_string()),
                body: Some(request_body.to_string()),
                capture: None,
                assert: Some(Assert {
                    status: 200,
                    breaking: true,
                    body: Some(
r#"{
    "id": 2
}
"#
                        .to_string(),
                    ),
                }),
            },
        ],
    };

    let mut environment_variables = HashMap::new();
    environment_variables.insert("API_KEY".to_string(), "SYS_API_KEY_V1".to_string());
    environment_variables.insert("DB_URL".to_string(), "SYS_DATABASE_URL_V1".to_string());
    let environment_variables = Some(environment_variables);

    let config = Config {
        capture: Some(Capture {
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
        }),
        run: vec![login_run],
    };

    let toml_string = toml::to_string_pretty(&config)?;

    write!(writer, "{toml_string}")?;
    Ok(())
}
