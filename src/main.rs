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


use alixt::{models::cli::Args, utils};
use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    let mut args = Args::parse();

    let mut writer: Box<dyn std::io::Write> = if args.output.is_some() {
        let file = match std::fs::File::create(args.output.take().unwrap()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: {e:#?}");
                exit(1);
            },
        };
        Box::new(file)
    } else {
        Box::new(std::io::stdout())
    };

    if args.generate_template {
        if let Err(e) = utils::template::generate_pretty(&mut writer) {
            eprintln!("Error generating template file: {e:#?}");
            exit(1)
        } else {
            exit(0)
        }
    }

    if args.generate_template_basic {
        if let Err(e) = utils::template::generate(&mut writer) {
            eprintln!("Error generating template file: {e:#?}");
            exit(1)
        } else {
            exit(0)
        }
    }

    match alixt::run(&mut writer, args).await {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}
