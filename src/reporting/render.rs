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
use colored::Colorize;

use crate::{models::{error::AlixtError, test_data::TestData}, reporting::table::{BOTTOM_LEFT, BOTTOM_RIGHT, HORIZONTAL, TOP_LEFT, TOP_RIGHT, Table, VERTICAL}};

pub fn generate_text<W: std::io::Write>(writer: &mut W, outcome: TestData) -> Result<(), AlixtError> {
    writeln!(writer, "[TEST RESULTS]")?;
    for run in outcome.run_data {
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
                let body = if let Ok(json) = serde_json::from_str::<Value>(&body) {
                    serde_json::to_string_pretty(&json).unwrap_or(body.clone())
                } else {
                    body
                };
                writeln!(writer, "Body = ```\n{}```", body)?;
            } else {
                writeln!(writer)?;
            }
        }
    }
    Ok(())
}

pub fn generate_table<W: std::io::Write>(writer: &mut W, outcome: TestData) -> Result<(), AlixtError> {
    let mut tables: Vec<Table<5>> = vec![];
    let mut passing: u16 = 0;
    let mut failing: u16 = 0;
    for run in outcome.run_data {
        let mut failed = false;
        let mut table = Table::<5>::new()
            .title(run.name.blue())
            .headers(["".white(), "Result".blue(), "Name".blue(), "Code".blue(), "Duration".blue()])
            .collect()?;
        for request in &run.outcomes {
            let passed = if request.passing {
                "PASS".green()
            } else {
                failed = true;
                "FAIL".red()
            };
            let breaking = if request.breaking && !request.passing {
                "BREAK".red()
            } else {
                "".white()
            };
            let status = if let Some(status) = request.status {
                format!("{status}").yellow()
            } else {
                "".white()
            };
            table.push_row([
                breaking,
                passed,
                request.name.yellow(),
                status,
                format!("{}s", request.duration.as_secs_f32()).yellow(),
            ])?;
        }
        if failed {
            failing += 1;
        } else {
            passing += 1;
        }
        tables.push(table);
    }
    let passing_text = format!("{passing}").green();
    let failing_text = format!("{failing}").red();

    let message = vec![
        "All runs finished. ".blue(),
        passing_text,
        " passing, ".blue(),
        failing_text,
        " failing.".blue(),
    ];
    let message_len: usize = message.iter().fold(0, |acc, w| acc + w.len());
    writeln!(
        writer,
        "{}{}{}",
        TOP_LEFT.blue(),
        HORIZONTAL.repeat(message_len).blue(),
        TOP_RIGHT.blue(),
    )?;
    write!(writer, "{}", VERTICAL.blue())?;
    for word in &message {
        write!(writer, "{word}")?;
    }
    writeln!(
        writer,
        "{}\n{}{}{}",
        VERTICAL.blue(),
        BOTTOM_LEFT.blue(),
        HORIZONTAL.repeat(message_len).blue(),
        BOTTOM_RIGHT.blue()
    )?;
    for table in tables {
        writeln!(writer, "\n")?;
        table.render(writer)?
    }

    if failing == 0 {
        writeln!(
            writer,
            "\n{}{}{}",
            TOP_LEFT.blue(),
            HORIZONTAL.repeat(7).blue(),
            TOP_RIGHT.blue()
        )?;
        writeln!(
            writer,
            "{}{}{}",
            VERTICAL.blue(),
            "SUCCESS".green(),
            VERTICAL.blue()
        )?;
        writeln!(
            writer,
            "{}{}{}",
            BOTTOM_LEFT.blue(),
            HORIZONTAL.repeat(7).blue(),
            BOTTOM_RIGHT.blue()
        )?;
    } else {
        writeln!(
            writer,
            "\n{}{}{}",
            TOP_LEFT.yellow(),
            HORIZONTAL.repeat(6).yellow(),
            TOP_RIGHT.yellow()
        )?;
        writeln!(
            writer,
            "{}{}{}",
            VERTICAL.yellow(),
            "FAILED".red(),
            VERTICAL.yellow()
        )?;
        writeln!(
            writer,
            "{}{}{}",
            BOTTOM_LEFT.yellow(),
            HORIZONTAL.repeat(6).yellow(),
            BOTTOM_RIGHT.yellow()
        )?;
    }
    
    Ok(())
}

pub fn generate_json<W: std::io::Write>(writer: &mut W, outcome: TestData) -> Result<(), AlixtError> {
    serde_json::to_writer_pretty(&mut *writer, &outcome)?;
    writeln!(writer)?;
    Ok(())
}
