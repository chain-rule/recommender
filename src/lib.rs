//! [Recommender system][recommender].
//!
//! [recommender]: https://en.wikipedia.org/wiki/Recommender_system

pub mod baseline;
pub mod dataset;
pub mod parser;

/// An error.
pub struct Error(Box<std::error::Error>);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        self.0.description()
    }

    #[inline]
    fn cause(&self) -> Option<&std::error::Error> {
        self.0.cause()
    }
}

macro_rules! implement {
    ($($kind:ty,)*) => {
        $(
            impl From<$kind> for Error {
                #[inline]
                fn from(error: $kind) -> Self {
                    Error(Box::new(error))
                }
            }
        )*
    }
}

implement! {
    std::io::Error,
    std::num::ParseFloatError,
    std::num::ParseIntError,
}
