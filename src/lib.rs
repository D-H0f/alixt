pub mod execute;
pub mod models;
pub mod preprocess;
pub mod reporting;
pub mod utils;


use std::sync::Arc;

use reqwest::Client;

use crate::{models::{cli::Output, context::Global, error::AlixtError}, reporting::logging::standard_out};

pub async fn run(args: models::cli::Args) -> Result<(), AlixtError> {
    let content = std::fs::read_to_string(args.file)?;
    let plan = preprocess::parse(content)?;

    #[allow(unused)]
    let mut global = Global::new();
    // add logic to grab env and global variables after it is created, but before it is frozen in
    // Arc

    let global = Arc::new(global);
    let client = Client::new();

    let outcome = execute::http::execute_test(&client, plan, global).await?;
    
    match args.output {
        Output::Text => {
            standard_out(outcome);
        },
        Output::Table => {},
        Output::Json => {},
    }
    Ok(())
}
