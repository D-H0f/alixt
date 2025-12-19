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


use std::collections::HashMap;

use reqwest::Method;

use crate::models::config::Assert;


pub struct TestPlan {
    pub runs: Vec<RunPlan>,
}

impl TestPlan {
    pub fn new() -> Self {
        Self {
            runs: Vec::new()
        }
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
