use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use bio::io::fasta;
use clap::Args;
use log::{info, warn};

use crate::model::error::{CytosError, CytosResult};
use crate::model::fasta::fasta_file::FastaFile;

#[derive(Args)]
pub struct ConcatCommand {
    /// Write the concatenated FASTA file to this path.
    pub output: PathBuf,

    /// Space-separated list of FASTA files to concatenate (this will form the order).
    pub input: Vec<PathBuf>,

    /// Missing character separator.
    #[clap(long, default_value = "-")]
    pub gap: String,
}

fn read_fasta_files(input: &Vec<PathBuf>) -> CytosResult<HashMap<&PathBuf, FastaFile>> {
    let mut fasta_files: HashMap<&PathBuf, FastaFile> = HashMap::new();
    for path in input {
        // Warn the user if a duplicate file was provided, and skip it
        if fasta_files.contains_key(path) {
            warn!("Skipping duplicate FASTA file: {}", path.display());
            continue;
        }
        let fasta = FastaFile::load(path)?;
        fasta_files.insert(path, fasta);
    }
    Ok(fasta_files)
}

fn get_ids_from_fasta_files<'a>(fasta_files: &'a HashMap<&PathBuf, FastaFile>) -> Vec<&'a str> {
    let mut ids: HashSet<&str> = HashSet::new();
    for fasta in fasta_files.values() {
        for id in fasta.gid_to_seq.keys() {
            ids.insert(id);
        }
    }
    let mut ids_sorted = ids.into_iter().collect::<Vec<&str>>();
    ids_sorted.sort();
    ids_sorted
}

fn get_total_size_from_fasta_files(fasta_files: &HashMap<&PathBuf, FastaFile>) -> usize {
    fasta_files.values().map(|fasta| fasta.size).sum()
}

pub fn concat_msa(args: &ConcatCommand) -> CytosResult<()> {

    // Parse the separation character
    let sep_char = {
        let str_vec = args.gap.as_bytes();
        if str_vec.len() != 1 {
            return Err(CytosError::InvalidArgument("The gap character must be a single character.".to_string()));
        }
        str_vec[0]
    };

    // Read the FASTA files
    let fasta_files = read_fasta_files(&args.input)?;
    info!("Loaded {} FASTA files.", fasta_files.len());

    // Collect the ids from each file
    let ids = get_ids_from_fasta_files(&fasta_files);
    let total_size = get_total_size_from_fasta_files(&fasta_files);
    info!("Found {} unique ids across all files.", ids.len());

    // Allocate memory for the output alignment
    let mut output: HashMap<&str, Vec<u8>> = HashMap::with_capacity(ids.len());
    for id in ids.iter() {
        let id = *id;
        output.insert(id, Vec::with_capacity(total_size));
    }

    // Iterate over the FASTA files and add the sequences to the output
    for fasta_path in &args.input {
        let fasta = fasta_files.get(fasta_path).expect("Internal error");

        // Iterate over the ids
        for id in &ids {
            let id = *id;
            warn!("{}", id);

            // Get the sequence
            let opt_record = fasta.gid_to_seq.get(id);

            let seq = if let Some(record) = opt_record {
                // Add the sequence
                let seq_slice = record.seq();
                seq_slice.to_vec()
            } else {
                // Add gaps
                vec![sep_char; fasta.size]
            };

            // Add the sequence to the output
            let output_seq = output.get_mut(id).expect("Internal error");
            output_seq.extend_from_slice(&seq);
        }
    }

    // Write the output file
    let out_file = File::create(&args.output).map_err(CytosError::IoError)?;
    let mut writer = fasta::Writer::new(out_file);

    for id in &ids {
        let id = *id;
        let seq = output.get(id).expect("Internal error");
        writer.write(id, None, seq).map_err(CytosError::IoError)?;
    }
    info!("Wrote alignment of length {} to: {}", total_size, args.output.display());

    Ok(())
}



