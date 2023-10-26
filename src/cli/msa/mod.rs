use clap::{Args, Subcommand};

use crate::cli::msa::bootstrap::BootstrapCommand;
use crate::cli::msa::concat::ConcatCommand;

pub mod concat;
pub mod bootstrap;


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

    /// Bootstrap a multiple sequence alignment.
    Bootstrap(BootstrapCommand),
}
