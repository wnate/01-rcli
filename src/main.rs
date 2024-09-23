use clap::Parser;
use rcli::{
    generate_password, process_csv, process_http_serve, Base64SubCommand, HttpSubCommand, Opts,
    SubCommand,
};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                println!("encode {:?}", opts);
            }
            Base64SubCommand::Decode(opts) => {
                println!("decode {:?}", opts);
            }
        },
        SubCommand::Http(opts) => match opts {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(&opts.dir, opts.port)?;
            }
        },
    }
    Ok(())
}
