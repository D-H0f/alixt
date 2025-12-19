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


pub mod execute;
pub mod models;
pub mod preprocess;
pub mod reporting;
pub mod utils;

use std::sync::Arc;

use reqwest::Client;

use crate::{
    models::{cli::OutputFormat, context::Global, error::AlixtError},
    reporting::render::{generate_table, generate_text},
};

pub async fn run<W: std::io::Write>(
    writer: &mut W,
    args: models::cli::Args,
) -> Result<(), AlixtError> {
    let content = std::fs::read_to_string(args.file.ok_or(AlixtError::InternalError(
        "Somehow an arg.file containing None got into run()".to_string(),
    ))?)?;
    let plan = preprocess::parse(content)?;

    #[allow(unused)]
    let mut global = Global::new();
    // add logic to grab env and global variables after it is created, but before it is frozen in
    // Arc

    let global = Arc::new(global);
    let client = Client::new();

    let outcome = execute::http::execute_test(&client, plan, global).await?;

    match args.mode {
        OutputFormat::Text => {
            generate_text(writer, outcome)?;
        }
        OutputFormat::Table => {
            generate_table(writer, outcome)?;
        }
        OutputFormat::Json => {}
    }
    Ok(())
}
