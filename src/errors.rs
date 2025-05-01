use thiserror::Error;
pub type Result<T> = std::result::Result<T, GumboError>;
use welds::connections::errors::Error as ConnectionError;
use welds::errors::WeldsError;

#[derive(Debug, Error)]
pub enum GumboError {
    #[error("Could not find the root path of a gumbo project. Are you working in a gumbo project?")]
    InvalidRootPath,

    #[error("Error Adding Dependencies: {0}")]
    DependenciesFailed(String),

    #[error("Error Running Cargo: {0}")]
    CargoInitFailed(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("The following string was used as a path: {0}")]
    InvalidPathStr(String),

    #[error("Error: Unknown field type: {0}")]
    InvalidFieldType(String),

    #[error("Error: Unknown action type: {0}")]
    InvalidControllerAction(String),

    #[error("Database Error")]
    SqlError(#[from] WeldsError),

    #[error("Database Connection Error")]
    ConnectionError(#[from] ConnectionError),

    #[error("The table was not found: {0}")]
    TableNotFound(String),
}
