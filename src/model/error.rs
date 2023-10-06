#[derive(Debug)]
pub enum CytosError {
    Exit(String),
    IoError(std::io::Error),
    DuplicateFastaId(String),
    EmptyFastaFile(String),
    MsaLengthMismatch(String),
    InvalidArgument(String),
}

impl std::fmt::Display for CytosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Exit(e) => write!(f, "{}", e),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::DuplicateFastaId(e) => write!(f, "Duplicate FASTA ID: {}", e),
            Self::EmptyFastaFile(e) => write!(f, "Empty FASTA file: {}", e),
            Self::MsaLengthMismatch(e) => write!(f, "MSA length mismatch: {}", e),
            Self::InvalidArgument(e) => write!(f, "Invalid argument: {}", e),
        }
    }
}

impl std::error::Error for CytosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Exit(_) => None,
            Self::IoError(e) => Some(e),
            Self::DuplicateFastaId(_) => None,
            Self::EmptyFastaFile(_) => None,
            Self::MsaLengthMismatch(_) => None,
            Self::InvalidArgument(_) => None,
        }
    }
}


pub type CytosResult<T> = Result<T, CytosError>;
