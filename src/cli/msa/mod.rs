use clap::{Args, Subcommand};

use crate::cli::msa::concat::ConcatCommand;

pub mod concat;


#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MsaArgs {
    #[command(subcommand)]
    pub command: MsaCommands,
}

#[derive(Subcommand)]
pub enum MsaCommands {
    /// Concatenate multiple alignments into one alignment.
    Concat(ConcatCommand),
}
