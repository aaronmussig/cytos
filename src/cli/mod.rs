use clap::{Parser, Subcommand};

use crate::cli::msa::MsaArgs;

pub mod msa;


#[derive(Parser)]
#[command(author, version)]
#[command(about = "cytos - a collection of bioinformatics scripts/tasks.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Actions to perform on multiple sequence alignments.
    Msa(MsaArgs),

}

