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

pub struct TestData {
    pub run_data: Vec<RunData>,
}

impl TestData {
    pub fn new() -> Self {
        Self {
            run_data: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RunData {
    pub name: String,
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

#[derive(Debug)]
pub struct RequestOutcome {
    pub name: String,
    pub method: String,
    pub url: String,

    pub passing: bool,
    pub breaking: bool,

    pub status: Option<u16>,
    pub response_body: Option<String>,
    pub duration: Duration,
}
