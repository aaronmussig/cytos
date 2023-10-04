use std::env;
use std::io::Write;

use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use env_logger::fmt::Color;
use log::{error, info, Level};
use log::LevelFilter;

use crate::cli::{Cli, Commands};
use crate::cli::msa::concat::concat_msa;
use crate::cli::msa::MsaCommands;
use crate::config::VERSION;

pub mod cli;
pub mod model;
mod config;

fn main() {
    // Initialise the logger at the default level of info
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    // env_logger::init();
    Builder::from_default_env()
        .format(|buf, record| {
            let color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Magenta,
                Level::Trace => Color::Cyan,
            };
            let mut style = buf.style();
            style.set_color(color).set_bold(true);

            writeln!(buf,
                     "[{}] {} - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     style.value(record.level()),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Output the program version
    info!("cytos v{}", VERSION.unwrap_or("unknown"));

    // Create the CLI and parse the arguments
    let cli = Cli::parse();
    let result = match cli.command {

        // msa
        Commands::Msa(args) => {
            match &args.command {

                // msa concat
                MsaCommands::Concat(cmd) => {
                    concat_msa(cmd)
                }
            }
        }
    };

    // Report the result
    match result {
        Ok(_) => info!("Done."),
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
