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


use thiserror::Error;


#[derive(Error, Debug)]
pub enum AlixtError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML content: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Failed to serialize to toml: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Failed to parse JSON body: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal Application error: {0}")]
    InternalError(String),

    #[error("Table Error: {0}")]
    TabletError(#[from] alixt_table::TableError),
}
