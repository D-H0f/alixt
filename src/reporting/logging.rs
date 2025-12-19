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


use serde_json::Value;

use crate::models::{error::AlixtError, test_data::TestData};

pub fn standard_out<W: std::io::Write>(writer: &mut W, test_outcome: TestData) -> Result<(), AlixtError> {
    writeln!(writer, "[TEST RESULTS]")?;
    for run in test_outcome.run_data {
        writeln!(writer, "\n[RUN]: '{}'", run.name)?;
        for req in run.outcomes {
            writeln!(
                writer,
                "\n[REQUEST]: '{}',\nTarget: '{}',\nPassed: {},\nBreaking: {},\nDuration: {} seconds,",
                req.name,
                req.url,
                req.passing,
                req.breaking,
                req.duration.as_secs_f64(),
            )?;
            if let Some(body) = req.response_body {
                let body = if let Some(json) = serde_json::from_str::<Value>(&body).ok() {
                    format!("{}", serde_json::to_string_pretty(&json).unwrap_or(body.clone()))
                } else {
                    body
                };
                writeln!(writer, "Body = ```\n{}```", body)?;
            } else {
                writeln!(writer, "")?;
            }
        }
    }
    Ok(())
}
