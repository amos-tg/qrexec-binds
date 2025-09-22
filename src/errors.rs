use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        self,
    },
    io,
};
use anyhow;

pub type QRXRes<T> = Result<T, QRXErr>;

// ERROR Messages :  
pub(crate) const STDOUT_ERR: &str = 
    "Error: child proc failed to produce stdout";
pub(crate) const STDIN_ERR: &str = 
    "Error: child proc failed to produce stdin";
pub(crate) const STDERR_ERR: &str =
    "Error: child proc failed to produce stderr";


#[derive(Debug)]
pub enum QRXErr {
    Io(io::Error), 
    Anyhow(anyhow::Error), 
}

impl Error for QRXErr {}

impl Display for QRXErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Io(io_err) => write!(f, "{}", io_err),
            Self::Anyhow(ah_err) => write!(f, "{}", ah_err), 
        }  
    }
}

impl From<anyhow::Error> for QRXErr {
    fn from(err: anyhow::Error) -> Self {
        return Self::Anyhow(err);
    }
}

impl From<io::Error> for QRXErr {
    fn from(err: io::Error) -> Self {
        return Self::Io(err);
    }
}
