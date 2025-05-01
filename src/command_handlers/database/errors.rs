use thiserror::Error;
use welds::connections::errors::Error as ConnectionError;
use welds::errors::WeldsError;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Error, Debug)]
pub enum DbError {}

