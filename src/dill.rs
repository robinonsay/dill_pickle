mod dill
{
    use std::{fmt, path::Path};

    #[derive(Debug)]
    enum DillError {
        Unexpected(String)
    }

    enum LoadingFlags
    {
        LAZY = 1,
        NOW = 2,

    }
    enum ScopeFlags
    {
        GLOBAL = 0,
        LOCAL = 1,
    }

    impl fmt::Display for DillError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DillError::Unexpected(ref msg) => write!(f, "Unexpected error: {}", msg),
            }
        }
    }
    
    trait Loader {
        fn dlopen(&self, path: Path, lflags: LoadingFlags, sflags: ScopeFlags) -> Result<(), DillError>;
        fn dlsym<F>(&self, symbol: String) -> Result<F, DillError>;
        fn dlclose(&self) -> Result<(), DillError>;
        fn dlerror() -> String;
    }
}