mod opts;
mod process;

pub use opts::{GenPassOpts, Opts, SubCommand};
pub use process::{generate_password, process_csv};
