use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a file over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
