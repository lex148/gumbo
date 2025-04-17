use thiserror::Error;
pub type Result<T> = std::result::Result<T, GumboError>;

#[derive(Debug, Error)]
pub enum GumboError {
    #[error("Could not find the root path of a gumbo project. Are you working in a gumbo project?")]
    InvalidRootPath,
    #[error("Error Adding Dependencies: {0}")]
    DependenciesFailed(String),
    #[error("Error Running Cargo: {0}")]
    CargoInitFailed(String),
    #[error("IO Error: {0}")]
    Io(std::io::Error),
    #[error("The following string was used as a path: {0}")]
    InvalidPathStr(String),
    #[error("Error: Unknown field type: {0}")]
    InvalidFieldType(String),
    #[error("Error: Unknown action type: {0}")]
    InvalidControllerAction(String),
}

impl From<std::io::Error> for GumboError {
    fn from(inner: std::io::Error) -> Self {
        GumboError::Io(inner)
    }
}
