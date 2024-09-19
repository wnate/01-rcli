use clap::Parser;
use rcli::{generate_password, process_csv, Opts, SubCommand};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            generate_password(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.symbol,
                opts.number,
            )?;
        }
    }
    Ok(())
}
