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


use std::process::exit;
use std::io::Write;

use clap::Parser;

use alixt::Error as ApiError;
use alixt::{Args, parse_file, request, template};

fn main() {
    let mut args = Args::parse();

    if args.generate_template {
        if let Err(e) = template::generate() {
            eprintln!("Failed to generate template file: {e:#?}");
            exit(1);
        }
        exit(0);
    }

    let mut writer: Box<dyn Write> = if args.verbose {
        Box::new(std::io::stdout())
    } else {
        Box::new(std::io::sink())
    };

    if let Some(file) = args.file.take() {
        match parse_file(&mut writer, file) {
            Ok(run) => {
                if !args.verbose {
                    run.pretty_print();
                }
                if run.all_passed() {
                    if args.verbose {
                        let _ = writeln!(&mut writer, "\n[SUCCESS]: All tests passed.");
                    }
                    exit(0);
                } else {
                    if args.verbose {
                        let _ = writeln!(&mut writer, "\n[FAILURE]: One or more tests failed.");
                    }
                    exit(1);
                }
            }
            Err(ApiError::Toml(e)) => {
                println!("[ERROR] toml parse error: \n----\n{e}");
                exit(1);
            }
            Err(e) => {
                println!("[ERROR]: {e:#?}");
                exit(1);
            }
        }
    } else if let Err(e) = request(
        &mut std::io::stdout(),
        args.method,
        &None,
        &args.url,
        &args.port,
        &args.target,
        &args.body,
    ) {
        println!("[ERROR]: {e:#?}");
        exit(1);
    }
}
