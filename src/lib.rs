mod cli;
mod process;

pub use cli::{csv::CsvOpts, csv::OutputFormat, genpass::GenPassOpts, Opts, SubCommand};
pub use process::{generate_password, process_csv};
