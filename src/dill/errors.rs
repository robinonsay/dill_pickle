use std::fmt;

const ERROR_MSG_LEN: usize = 256;

#[derive(Debug)]
pub enum DillError {
    UnexpectedErr([u8; ERROR_MSG_LEN]),
    NullErr([u8; ERROR_MSG_LEN]),
    LibOpenErr([u8; ERROR_MSG_LEN]),
    Utf8Err([u8; ERROR_MSG_LEN]),
}

impl DillError {
    // Helper function to convert &str to [u8; ERROR_MSG_LEN]
    pub fn create_msg(msg: &str) -> [u8; ERROR_MSG_LEN] {
        let mut array = [0u8; ERROR_MSG_LEN];
        let bytes = msg.as_bytes();
        let len = bytes.len().min(ERROR_MSG_LEN);
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }
    // Helper function to convert &[u8; ERROR_MSG_LEN] to &str
    fn msg_as_str(msg: &[u8; ERROR_MSG_LEN]) -> &str {
        let end = msg.iter().position(|&b| b == 0).unwrap_or(ERROR_MSG_LEN);
        std::str::from_utf8(&msg[..end]).unwrap_or("Invalid UTF-8 in error message")
    }
}

impl fmt::Display for DillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DillError::UnexpectedErr(ref msg) => write!(f, "Unexpected error: {}", DillError::msg_as_str(msg)),
            DillError::NullErr(ref msg) => write!(f, "Null error: {}", DillError::msg_as_str(msg)),
            DillError::LibOpenErr(ref msg) => write!(f, "Library open error: {}", DillError::msg_as_str(msg)),
            DillError::Utf8Err(ref msg) => write!(f, "UTF-8 error: {}", DillError::msg_as_str(msg)),
        }
    }
}
