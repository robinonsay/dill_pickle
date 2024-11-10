use super::errors::DillError;

/// Dill Loader
pub trait Loader {
    /// Open
    fn load<F>(&self, sym_name: &str) -> Result<F, DillError>;
}
