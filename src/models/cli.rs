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


use std::path::PathBuf;

use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
pub struct Args {
    /// Generate a test_requests.toml template file to use
    #[arg(long, conflicts_with = "file")]
    pub generate_template: bool,

    /// Run requests from a .toml file.
    #[arg(short, long, required_unless_present = "generate_template")]
    pub file: Option<String>,

    /// The format of the output
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table, requires = "file")]
    pub mode: OutputFormat,

    /// Write output to a specific file instead of stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Table,
    Json,
}
