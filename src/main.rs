use alixt::{models::cli::Args, utils::template};
use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.generate_template {
        if let Err(e) = template::generate() {
            eprintln!("Failed to generate template file: {e:#?}");
            exit(1);
        }
        exit(0);
    }
    
    match alixt::run(args).await {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}
