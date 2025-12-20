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


use std::time::Duration;

use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct TestData {
    #[serde(rename = "runs")]
    pub run_data: Vec<RunData>,
}

impl TestData {
    pub fn new() -> Self {
        Self {
            run_data: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RunData {
    pub name: String,
    #[serde(rename = "requests")]
    pub outcomes: Vec<RequestOutcome>,
}

impl RunData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            outcomes: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RequestOutcome {
    pub name: String,
    pub method: String,
    pub url: String,

    pub passing: bool,
    pub breaking: bool,

    pub status: Option<u16>,
    pub response_body: Option<String>,
    #[serde(serialize_with = "serialize_duration_as_seconds", rename = "duration_seconds")]
    pub duration: Duration,
}


fn serialize_duration_as_seconds<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(duration.as_secs_f64())
}
