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

use crate::Error;

use colored::{ColoredString, Colorize};

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
    pub fn print<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
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

    pub fn display_results<W: Write>(&mut self, writer: &mut W) -> Result<(), Error> {
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
        let mut tables: Vec<Table> = vec![];
        for run in self.all.iter() {
            let mut table = Table::new(4);
            table.title = run.name.blue();
            table.push_header(vec![
                "".white(),
                "Result".blue(),
                "Name".blue(),
                "Code".blue(),
            ]);
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
                table.push_row(vec![
                    breaking,
                    passed,
                    test.name.yellow(),
                    format!("{}", test.status).yellow(),
                ]);
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
            table.print(true, true);
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

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Position {
    Column(i16),
    Row(i16),
    Header(i16),
    Meta,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coord {
    x: Position,
    y: Position,
}

impl Coord {
    fn item(col: i16, row: i16) -> Self {
        Self {
            x: Position::Column(col),
            y: Position::Row(row),
        }
    }

    fn meta(x: Position) -> Self {
        Self {
            x,
            y: Position::Meta,
        }
    }
}

pub struct TestData {
    pub name: String,
    pub status: u16,
    pub passing: bool,
    pub breaking: bool,
}

struct Table {
    title: ColoredString,
    columns: usize,
    rows: usize,
    data: HashMap<Coord, ColoredString>,
}

impl Table {
    /// fixed size table
    fn new(width: usize) -> Self {
        if width > i16::MAX as usize {
            panic!("Width cannot be larger than {}", i16::MAX);
        }
        let mut data = HashMap::new();
        // insert starter values for column length measurment
        for i in 0..width {
            data.insert(Coord::meta(Position::Column(i as i16)), "".white());
        }
        Self {
            title: "".white(),
            columns: width,
            rows: 0,
            data,
        }
    }
    fn push_header(&mut self, headers: Vec<ColoredString>) {
        if headers.len() != self.columns {
            panic!(
                "wrong number of columns, must be {}, got {}",
                self.columns,
                headers.len()
            );
        }
        for (i, item) in headers.into_iter().enumerate() {
            if item.len() > self.col_width(i as i16) {
                self.data
                    .insert(Coord::meta(Position::Column(i as i16)), item.clone());
            }
            if self
                .data
                .insert(Coord::meta(Position::Header(i as i16)), item)
                .is_some()
            {
                panic!("cannot overwrite existing header");
            }
        }
    }
    fn push_row(&mut self, row: Vec<ColoredString>) {
        if row.len() != self.columns {
            panic!(
                "wrong number of columns, must be {}, got {}",
                self.columns,
                row.len()
            );
        }
        for (i, item) in row.into_iter().enumerate() {
            let column_key = Coord::meta(Position::Column(i as i16));
            if item.len() > self.col_width(i as i16) {
                self.data.insert(column_key, item.clone());
            }
            let key = Coord::item(i as i16, self.rows as i16);
            if self.data.insert(key, item).is_some() {
                panic!("cannot overwrite existing row");
            }
        }
        self.rows += 1;
    }
    fn spacer(&self, width: &Coord, minus: &Coord) -> String {
        " ".repeat(self.data.get(width).unwrap().len() - self.data.get(minus).unwrap().len())
    }
    fn max_width(&self) -> usize {
        (0..self.columns).fold(0, |acc, n| acc + self.col_width(n as i16))
    }
    fn col_width(&self, i: i16) -> usize {
        self.data
            .get(&Coord::meta(Position::Column(i)))
            .unwrap()
            .len()
    }
    fn get(&self, key: &Coord) -> &ColoredString {
        self.data.get(key).unwrap()
    }
    fn _combine(&self, c_string: ColoredString, spacer: &str) -> ColoredString {
        if let Some(color) = c_string.fgcolor {
            format!("{c_string}{spacer}").color(color)
        } else {
            format!("{c_string}{spacer}").white()
        }
    }

    fn print(&self, border: bool, divider: bool) {
        let table_width = self.max_width() + self.columns - 1;
        let title = if self.title.len() > self.max_width() {
            format!("{}...", self.title[..table_width - 5].to_string())
                .color(self.title.fgcolor.unwrap())
        } else {
            self.title.clone()
        };
        if border {
            print!("{}", TOP_LEFT.blue());
            print!("{}", HORIZONTAL.repeat(self.col_width(0)).blue());
            for col in 1..self.columns {
                print!(
                    "{}",
                    HORIZONTAL.repeat(self.col_width(col as i16) + 1).blue()
                );
            }
            print!("{}\n", TOP_RIGHT.blue());
            print!("{}", VERTICAL.blue());
        }
        let pad_l = " ".repeat((table_width - title.len()) / 2);
        let pad_r = " ".repeat(table_width - title.len() - (table_width - title.len()) / 2);
        print!("{}{}{}", pad_l, title, pad_r);

        if border {
            print!("{}\n", VERTICAL.blue());
            print!("{}", LEFT_T.blue());
            print!("{}", HORIZONTAL.repeat(self.col_width(0)).blue());
            for col in 1..self.columns {
                print!(
                    "{}{}",
                    TOP_T.blue(),
                    HORIZONTAL.repeat(self.col_width(col as i16)).blue()
                );
            }
            print!("{}\n", RIGHT_T.blue());
            print!("{}", VERTICAL.blue());
        } else {
            print!("\n")
        }
        print!(
            "{}",
            self.get(&Coord::meta(Position::Header(0)))
        );
        print!(
            "{}",
            self.spacer(
                &Coord::meta(Position::Column(0)),
                &Coord::meta(Position::Header(0))
            )
        );
        for col in 1..self.columns {
            if divider {
                print!("{}", VERTICAL.blue());
            }
            print!(
                "{}",
                self.get(&Coord::meta(Position::Header(col as i16)))
            );
            print!("{}", self.spacer(
                &Coord::meta(Position::Column(col as i16)),
                &Coord::meta(Position::Header(col as i16))
            ));
        }
        if border {
            print!("{}\n", VERTICAL.blue());
            print!("{}", LEFT_T.blue());
            print!("{}", HORIZONTAL.repeat(self.col_width(0)).blue());
            for col in 1..self.columns {
                print!(
                    "{}{}",
                    CROSS.blue(),
                    HORIZONTAL.repeat(self.col_width(col as i16)).blue()
                );
            }
            print!("{}\n", RIGHT_T.blue());
        } else {
            print!("\n")
        }
        for row in 0..self.rows {
            if border {
                print!("{}", VERTICAL.blue());
            }
            print!("{}", self.get(&Coord::item(row as i16, 0)));
            print!("{}", self.spacer(
                &Coord::meta(Position::Column(0)),
                &Coord::item(row as i16, 0)
            ));
            for col in 1..self.columns {
                if divider {
                    print!("{}", VERTICAL.blue());
                }
                print!("{}", self.get(&Coord::item(col as i16, row as i16)));
                print!("{}", self.spacer(
                    &Coord::meta(Position::Column(col as i16)),
                    &Coord::item(col as i16, row as i16)
                ));
            }
            if border {
                print!("{}", VERTICAL.blue());
            }
            print!("\n");
        }
        if border {
            print!("{}", BOTTOM_LEFT.blue());
            print!("{}", HORIZONTAL.repeat(self.col_width(0)).blue());
            for col in 1..self.columns {
                print!(
                    "{}{}",
                    BOTTOM_T.blue(),
                    HORIZONTAL.repeat(self.col_width(col as i16)).blue()
                );
            }
            print!("{}", BOTTOM_RIGHT.blue());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use colored::Colorize;
    #[test]
    fn make_a_table() {
        let mut table = Table::new(4);
        table.push_header(vec![
            "my".white(),
            "cool".white(),
            "header".white(),
            "wow".white(),
        ]);
        table.push_row(vec!["a".white(), "b".white(), "c".white(), "d".white()]);
        assert_eq!(15, table.max_width());
        assert_eq!(6, table.col_width(2));
    }
}
