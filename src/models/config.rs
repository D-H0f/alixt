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


use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    #[serde(alias = "Get", alias = "GET")]
    Get,
    #[serde(alias = "Post", alias = "POST")]
    Post,
    #[serde(alias = "Put", alias = "PUT")]
    Put,
    #[serde(alias = "Delete", alias = "DELETE")]
    Delete,
    #[serde(alias = "Patch", alias = "PATCH")]
    Patch,
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    #[serde(alias = "Http", alias = "HTTP")]
    Http,
    #[serde(alias = "Https", alias = "HTTPS")]
    Https,
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub capture: Option<Capture>,
    pub run: Vec<Run>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Capture {
    pub env_file: Option<std::path::PathBuf>,
    pub environment_variables: Option<HashMap<String, String>>,
    pub request: Option<Vec<CaptureRequest>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "request")]
pub struct CaptureRequest {
    pub name: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub method: Method,
    pub scheme: Scheme,
    pub host: String,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub body: Option<String>,
    pub capture: Option<HashMap<String, String>>,
}

// holds multiple requests, contents are blocking
#[derive(Serialize, Deserialize, Debug)]
pub struct Run {
    pub name: String,
    // These fields are the defaults for all requests in the run
    pub headers: Option<HashMap<String, String>>,
    pub method: Option<Method>,
    pub scheme: Option<Scheme>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub body: Option<String>,

    pub request: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub headers: Option<HashMap<String, String>>,
    pub method: Option<Method>,
    pub scheme: Option<Scheme>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub body: Option<String>,

    pub capture: Option<HashMap<String, String>>,
    pub assert: Option<Assert>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assert {
    #[serde(default)]
    pub breaking: bool,
    pub status: Option<u16>,
    pub body_matches: Option<HashMap<String, Value>>,
    pub subset_matches: Option<HashMap<String, Value>>,
    pub subset_includes: Option<Vec<String>>,
    pub subset_regex: Option<HashMap<String, Value>>,
}
