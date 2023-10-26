use std::fs::{create_dir_all, File};
use std::path::PathBuf;

use bio::io::fasta;
use clap::Args;
use log::info;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::model::error::{CytosError, CytosResult};
use crate::model::fasta::fasta_file::FastaFile;

#[derive(Args)]
pub struct BootstrapCommand {
    /// Read the FASTA file from this path.
    pub input: PathBuf,

    /// Write the bootstrapped FASTA files to this directory.
    pub output: PathBuf,

    /// Create this many bootstrap replicates.
    #[clap(long, default_value = "1")]
    pub replicates: usize,

    /// Random number generation seed (e.g. 42).
    #[clap(long)]
    pub seed: Option<u64>,
}


fn subsample_sequence(seq: &[u8], rng: &mut StdRng) -> Vec<u8> {
    let n_chars = seq.len();
    let between = Uniform::from(0..n_chars);
    let mut out: Vec<u8> = Vec::with_capacity(n_chars);
    for _ in 0..n_chars {
        let i = between.sample(rng);
        out.push(seq[i]);
    }
    out
}


pub fn run_bootstrap_msa(args: &BootstrapCommand) -> CytosResult<()> {

    // Load the FASTA file
    let fasta = FastaFile::load(&args.input)?;
    let prefix = args.input.file_stem().unwrap().to_str().unwrap();
    let ext = args.input.extension().unwrap().to_str().unwrap();

    // Create the output directory
    create_dir_all(&args.output).map_err(CytosError::IoError)?;

    // Create the RNG
    let mut rng = match args.seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    // Create the bootstrap replicates
    for i in 0..args.replicates {
        let output_path = args.output.join(format!("{}_{}.{}", prefix, i, ext));
        let mut output_file = File::create(&output_path).map_err(CytosError::IoError)?;
        let mut writer = fasta::Writer::new(&mut output_file);
        for id in &fasta.gids {
            let seq = fasta.gid_to_seq.get(id).unwrap();
            let subsampled_seq = subsample_sequence(seq.seq(), &mut rng);
            let new_seq = fasta::Record::with_attrs(id, seq.desc(), &subsampled_seq);
            writer.write_record(&new_seq).map_err(CytosError::IoError)?;
        }
    }

    info!("Created {} bootstrap replicate(s) at: {}", args.replicates, args.output.display());

    Ok(())
}