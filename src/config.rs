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
    pub capture: Option<HashMap<String, String>>,
    pub assert: Assert,
}

#[derive(Deserialize, Debug)]
pub struct Assert {
    pub status: u16,
    pub breaking: bool,
    pub body: Option<String>,
}
