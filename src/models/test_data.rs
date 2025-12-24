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

#[derive(Default, Debug, Serialize)]
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

    pub passing: AssertionOutcome,
    pub breaking: bool,

    pub status: Option<u16>,
    pub response_body: Option<String>,
    #[serde(serialize_with = "serialize_duration_as_seconds", rename = "duration_seconds")]
    pub duration: Duration,
}


fn serialize_duration_as_seconds<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(duration.as_secs_f64())
}

#[derive(Debug, Serialize)]
pub enum AssertionOutcome {
    Passed,
    Failed(Vec<FailureType>),
}
impl AssertionOutcome {
    pub fn push(&mut self, failure: FailureType) {
        match self {
            Self::Passed => *self = Self::Failed(vec![failure]),
            Self::Failed(fails) => fails.push(failure),
        }
    }
    pub fn is_passing(&self) -> bool {
        match self {
            Self::Passed => true,
            Self::Failed(_) => false,
        }
    }
    pub fn take(&mut self) -> Self {
        match self {
            Self::Passed => {
                std::mem::replace(self, AssertionOutcome::Passed)
            },
            Self::Failed(_) => {
                std::mem::replace(self, AssertionOutcome::Failed(Vec::new()))
            },
        }
    }
}
#[derive(Debug, Serialize)]
pub enum FailureType {
    StatusMismatch{ expected: u16, found: Option<u16> },
    InvalidJson(),
    JsonMissingField { path: String },
    JsonExtraField { path: String },
    JsonValueMismatch { path: String, expected: String, found: String },
    JsonRegexMismatch { path: String, pattern: String, found: String },
    JsonNotString { path: String },
}
