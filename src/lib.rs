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
pub mod reporting;
pub mod utils;

use std::{path::Path, sync::Arc};

use reqwest::Client;

use crate::{
    models::{
        cli::OutputFormat, config::Config, context::Global, error::AlixtError, plan::TestPlan,
    },
    reporting::render::{generate_json, generate_table, generate_text}, utils::env,
};

pub async fn run<W: std::io::Write>(
    writer: &mut W,
    args: models::cli::Args,
) -> Result<(), AlixtError> {
    let Some(config_file) = args.file else {
        return Err(AlixtError::InternalError(
            "Somehow an arg.file containing None got into run()".to_string(),
        ));
    };
    let content = std::fs::read_to_string(&config_file)?;
    let config: Config = toml::from_str(&content)?;
    let config_dir = std::fs::canonicalize(&config_file)?
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_owned();

    let plan = TestPlan::from_config(config, &config_dir)?;

    #[allow(unused)]
    let mut global = Global::new();
    let client = Client::new();

    if let Some(capture_plan) = &plan.capture {
        if let Some(env_map) = &capture_plan.environment_variables {
            let dot_env_vars = if let Some(path) = &capture_plan.env_file {
                Some(env::load_env_file(path)?)
            } else {
                None
            };

            let captured = env::capture_system_environment(env_map, dot_env_vars)?;

            global.env_variables.extend(captured);
        }

        if let Some(requests) = &capture_plan.requests {
            for request in requests {
                execute::http::execute_capture_request(&client, request, &mut global).await?;
            }
        }
    }

    let global = Arc::new(global);
    let outcome = execute::http::execute_test(&client, plan, global).await?;

    match args.mode {
        OutputFormat::Text => {
            generate_text(writer, outcome)?;
        }
        OutputFormat::Table => {
            generate_table(writer, outcome)?;
        }
        OutputFormat::Json => {
            generate_json(writer, outcome)?;
        }
    }
    Ok(())
}
