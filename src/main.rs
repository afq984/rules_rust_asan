use std::io;

use thiserror::Error;

// Example from
// https://docs.rs/thiserror/latest/thiserror/
#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}


fn main() {
    println!("{}", DataStoreError::Redaction("12345".to_string()));
}

#[test]
fn test_main() {
    main();
}
