use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
pub struct Args {
    /// Generate a test_requests.toml template file to use
    #[arg(long)]
    pub generate_template: bool,

    /// Run requests from a .toml file.
    #[arg(short, long)]
    pub file: String,

    /// displays more detailed output for each request
    #[arg(short, long, value_enum, default_value_t = Output::Table)]
    pub output: Output,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
    Text,
    Table,
    Json,
}
