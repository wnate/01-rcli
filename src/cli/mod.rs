pub mod base64;
pub mod csv;
pub mod genpass;
mod http;

pub use self::base64::Base64SubCommand;
use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
pub use self::http::HttpSubCommand;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(name = "base64", subcommand)]
    Base64(Base64SubCommand),
    #[command(name = "http", subcommand)]
    Http(HttpSubCommand),
}
