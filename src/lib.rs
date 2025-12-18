pub mod execute;
pub mod models;
pub mod preprocess;
pub mod reporting;
pub mod utils;


use crate::models::{cli::Output, error::AlixtError, test_data::TestData};

pub async fn run_test(args: models::cli::Args) -> Result<TestData, AlixtError> {
    let content = std::fs::read_to_string(args.file)?;
    let plan = preprocess::parse(content)?;

    let results = execute::test_endpoints::run_test(plan).await?;
    
    match args.output {
        Output::Text => todo!(),
        Output::Table => todo!(),
        Output::Json => todo!(),
    }
}
