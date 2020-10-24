use crate::*;

use std::fmt::Display;



/// Utility methods for [std::result::Result]
pub trait ResultExt {
    type Output;

    /// Shorthand for `result.unwrap_or_else(|err| fatal!("{}", err))`
    fn or_die(self) -> Self::Output;
}

impl<T, E: Display> ResultExt for Result<T, E> {
    type Output = T;

    fn or_die(self) -> Self::Output {
        match self {
            Ok(r) => r,
            Err(err) => fatal!("{}", err),
        }
    }
}
