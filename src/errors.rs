/// 自定义错误

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IOError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

// #[cfg(test)]
// mod tests {
//     use crate::errors::IOError;
//     use std::error::Error;
//
//     #[test]
//     fn test_io_error() {
//         let error = IOError::GetFileInfo(std::io::Error::from(std::io::ErrorKind::NotFound));
//         assert_eq!(error.to_string(), "Failed to get file info: Not found (os error 2)");
//     }
// }