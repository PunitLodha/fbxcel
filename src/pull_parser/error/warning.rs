//! Invalid operation.

use std::error;
use std::fmt;

/// Warning.
#[derive(Debug)]
pub enum Warning {
    /// Node name is empty.
    EmptyNodeName,
    /// Incorrect boolean representation.
    ///
    /// Boolean value in node attributes should be some prescribed value
    /// (for example `b'T'` and `b'Y'` for FBX 7.4).
    /// Official SDK and tools may emit those values correctly, but some
    /// third-party exporters emits them wrongly with `0x00` and `0x01`, and
    /// those will be ignored by official SDK and tools.
    IncorrectBooleanRepresentation,
}

impl error::Error for Warning {}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::EmptyNodeName => write!(f, "Node name is empty"),
            Warning::IncorrectBooleanRepresentation => {
                write!(f, "Incorrect boolean representation")
            }
        }
    }
}
