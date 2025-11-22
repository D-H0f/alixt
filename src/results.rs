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
use std::io::Write;

use crate::{AlixtError, table::{Table, TableResult}};

use colored::Colorize;

const TOP_LEFT: &str = "╭";
const TOP_RIGHT: &str = "╮";
const BOTTOM_LEFT: &str = "╰";
const BOTTOM_RIGHT: &str = "╯";
const HORIZONTAL: &str = "─";
const VERTICAL: &str = "│";
const CROSS: &str = "┼";
const TOP_T: &str = "┬";
const BOTTOM_T: &str = "┴";
const LEFT_T: &str = "├";
const RIGHT_T: &str = "┤";

/// Holds the results for a request
#[derive(Default)]
pub struct RunData {
    name: String,
    /// test name, pass/fail, breaking
    //tests: Vec<(String, bool, bool)>,
    tests: Vec<TestData>,
}

impl RunData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tests: vec![],
        }
    }
    pub fn add(&mut self, name: &str, status: u16, passing: bool, breaking: bool) {
        let mut name = String::from(name);

        if name.len() > 20 {
            name = format!("{}...", &name[..20]);
        } else if name.is_empty() {
            panic!("test must have a name");
        }

        self.tests.push(TestData {
            name,
            status,
            passing,
            breaking,
        });
    }
    pub fn print<W: Write>(&self, writer: &mut W) -> Result<(), AlixtError> {
        writeln!(writer, "[API]: {} tests completed:", self.tests.len())?;
        for test in &self.tests {
            let grade = if test.passing {
                "[PASS]".to_string()
            } else {
                "[FAIL]".to_string()
            };
            writeln!(writer, "{grade} {}", test.name)?;
            if test.breaking {
                writeln!(writer, "[API]: run stopped due to breaking assertion")?;
            }
        }
        Ok(())
    }
    pub fn is_pass(&self) -> bool {
        self.tests.iter().all(|test| test.passing)
    }
}

/// Holds the results for all runs
#[derive(Default)]
pub struct AllRuns {
    all: Vec<RunData>,
}

impl AllRuns {
    pub fn new() -> Self {
        Self { all: vec![] }
    }

    pub fn add(&mut self, test: RunData) {
        self.all.push(test);
    }

    pub fn display_results<W: Write>(&mut self, writer: &mut W) -> Result<(), AlixtError> {
        let passing = self
            .all
            .iter()
            .filter(|test| test.is_pass())
            .fold(0u16, |x, _| x + 1);
        let failing = self
            .all
            .iter()
            .filter(|test| !test.is_pass())
            .fold(0u16, |x, _| x + 1);
        writeln!(
            writer,
            "\n\n\n\n[TEST]: All runs finished. {passing} passing, {failing} failing."
        )?;

        for (index, test) in self.all.iter().enumerate() {
            writeln!(writer, "\n\n----[RUN]-{index}----\n")?;
            test.print(writer)?;
        }
        Ok(())
    }

    pub fn pretty_print(&self) {
        let mut tables: Vec<Table<4>> = vec![];
        for run in self.all.iter() {
            let mut table = Table::<4>::new()
                .title(run.name.blue())
                .headers(["".white(), "Result".blue(), "Name".blue(), "Code".blue()])
                .collect()
                .unwrap();
            for test in &run.tests {
                let passed = if test.passing {
                    "PASS".green()
                } else {
                    "FAIL".red()
                };
                let breaking = if test.breaking {
                    "BREAK".red()
                } else {
                    "".white()
                };
                if let Err(e) = table.push_row([
                    breaking,
                    passed,
                    test.name.yellow(),
                    format!("{}", test.status).yellow(),
                ]) {
                    eprintln!("ERROR: '{e}'");
                }
            }
            tables.push(table);
        }
        let passing = format!(
            "{}",
            self.all
                .iter()
                .filter(|test| test.is_pass())
                .fold(0u16, |x, _| x + 1)
        );
        let failing = format!(
            "{}",
            self.all
                .iter()
                .filter(|test| !test.is_pass())
                .fold(0u16, |x, _| x + 1)
        );
        let message = vec![
            "All runs finished. ".blue(),
            passing.green(),
            " passing, ".blue(),
            failing.red(),
            " failing.".blue(),
        ];
        let messege_len = message.iter().fold(0, |acc, w| acc + w.len());
        print!(
            "{}{}{}\n",
            TOP_LEFT.blue(),
            HORIZONTAL.repeat(messege_len).blue(),
            TOP_RIGHT.blue()
        );
        print!("{}", VERTICAL.blue());
        for word in &message {
            print!("{word}");
        }
        print!(
            "{}\n{}{}{}\n",
            VERTICAL.blue(),
            BOTTOM_LEFT.blue(),
            HORIZONTAL.repeat(messege_len).blue(),
            BOTTOM_RIGHT.blue()
        );
        for table in tables {
            println!("\n");
            if let Err(e) = table.render(&mut std::io::stdout()) {
                eprintln!("ERROR: '{e}'");
            }
        }

        if self.all_passed() {
            println!(
                "\n{}{}{}",
                TOP_LEFT.blue(),
                HORIZONTAL.repeat(7).blue(),
                TOP_RIGHT.blue()
            );
            println!(
                "{}{}{}",
                VERTICAL.blue(),
                "SUCCESS".green(),
                VERTICAL.blue()
            );
            println!(
                "{}{}{}",
                BOTTOM_LEFT.blue(),
                HORIZONTAL.repeat(7).blue(),
                BOTTOM_RIGHT.blue()
            );
        } else {
            println!(
                "\n{}{}{}",
                TOP_LEFT.yellow(),
                HORIZONTAL.repeat(6).yellow(),
                TOP_RIGHT.yellow()
            );
            println!(
                "{}{}{}",
                VERTICAL.yellow(),
                "FAILED".red(),
                VERTICAL.yellow()
            );
            println!(
                "{}{}{}",
                BOTTOM_LEFT.yellow(),
                HORIZONTAL.repeat(6).yellow(),
                BOTTOM_RIGHT.yellow()
            )
        }
    }

    pub fn all_passed(&self) -> bool {
        self.all.iter().all(|rslt| rslt.is_pass())
    }
}


pub struct TestData {
    pub name: String,
    pub status: u16,
    pub passing: bool,
    pub breaking: bool,
}
