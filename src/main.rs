use std::process::exit;

use clap::Parser;

use api_tester::{Args, request, parse_file, template};
use api_tester::Error as ApiError;


fn main() {
    let mut args = Args::parse();

    if args.generate_template {
        if let Err(e) = template::generate() {
            eprintln!("Failed to generate template file: {e:#?}")
        }
        return;
    }

    if let Some(file) = args.file.take() {
        match parse_file(file) {
            Ok(true) => {
                println!("\n[SUCCESS]: All tests passed.");
                exit(0);
            },
            Ok(false) => {
                println!("\n[FAILURE]: One or more tests failed.");
                exit(1);
            },
            Err(ApiError::Toml(e)) => {
                println!("[ERROR] toml parse error: \n----\n{e}");
                exit(1);
            },
            Err(e) => {
                println!("[ERROR]: {e:#?}");
                exit(1);
            }
        }
    } else {
        if let Err(e) = request(
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
}
