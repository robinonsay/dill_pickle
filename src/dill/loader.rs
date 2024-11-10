use super::errors::DillError;

// const MAX_PATH_LEN: usize = 256;

/// Represents the Symbol to look up
pub struct Symbol<'a> {
    pub name: &'a str
}

/// Dill Loader
pub trait Loader {
    /// Open
    fn open(&mut self, lib: &str) -> Result<(), DillError>;
    fn load<F>(&self, symbol: Symbol) -> Result<F, DillError>;
    fn close(&mut self) -> Result<(), DillError>;
}
