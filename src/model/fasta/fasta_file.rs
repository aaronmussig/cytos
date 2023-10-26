use std::collections::HashMap;
use std::fs::File;
use std::path;

use bio::io::fasta;

use crate::model::error::{CytosError, CytosResult};

pub struct FastaFile<'a> {
    pub path: &'a path::PathBuf,
    pub gid_to_seq: HashMap<String, fasta::Record>,
    pub gids: Vec<String>,
    pub size: usize,
}

impl<'a> FastaFile<'a> {
    pub fn load(path: &'a path::PathBuf) -> CytosResult<FastaFile> {

        // Open the FASTA file
        let file = {
            let f = File::open(path);
            match f {
                Ok(f) => f,
                Err(e) => return Err(CytosError::Exit(
                    format!("Unable to open '{}' [{}]", path.display(), e)
                )),
            }
        };
        let reader = fasta::Reader::new(file);
        let mut size: Option<usize> = None;

        // Iterate over the contents
        let mut gid_to_seq = HashMap::new();
        let mut gids: Vec<String> = Vec::new();
        for result in reader.records() {

            // Obtain the record
            let record = result.map_err(CytosError::IoError)?;
            let id = record.id().to_string();
            let seq = record.seq();

            // Check if this is a duplicate
            if gid_to_seq.contains_key(&id) {
                return Err(CytosError::DuplicateFastaId(id));
            }

            // Update the size
            if let Some(cur_size) = size {
                // Exit if the size is not consistent
                if cur_size != seq.len() {
                    return Err(CytosError::MsaLengthMismatch(id));
                }
            } else {
                size = Some(seq.len());
            }

            // Add the record to the map
            gid_to_seq.insert(id.clone(), record);
            gids.push(id.clone());
        }

        // Exit if size was never updated
        if size.is_none() {
            return Err(CytosError::EmptyFastaFile(path.display().to_string()));
        }

        Ok(FastaFile {
            path,
            gid_to_seq,
            gids,
            size: size.unwrap(),
        })
    }
}
