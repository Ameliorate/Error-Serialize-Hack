// mod io_error;

use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Used if an error from a popular crate is better being fully seralized, or if it is in std and has fields that should be preserved.
///
/// This enum may be expanded in the future. For this reason, you should not exaustively match aganst it.
#[derive(Clone, Serialize, Deserialize, RustcEncodable, RustcDecodable, Debug, Copy)]
pub enum RealError {
    // IoError(io_error::SerializableIoError),
    /// This ensures that you cannot match the whole enum, and must always consiter for more feilds in the furure.
    ///
    /// This should also never exist in usage, and if it is, consuming it will return some error.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl RealError {
    /// Returns true if the error value can be represented as a RealError.
    #[allow(dead_code)]
    pub fn can_represent(_err: &Error) -> bool {
        unimplemented!()
    }
}

impl Display for RealError {
    fn fmt(&self, _fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::RealError::*;
        match *self {
            // IoError(ref err) => write!(fmt, "{}", err),
            __Nonexhaustive => Err(fmt::Error), // __Nonexhaustive should never exist in a RealError. If I could return a real error, I would.
        }
    }
}

impl<'a> From<&'a Error> for RealError {
    fn from(_err: &Error) -> RealError {
        unimplemented!()
    }
}

impl Error for RealError {
    fn description(&self) -> &str {
        use self::RealError::*;
        match *self {
            // IoError(ref err) => err.description(),
            __Nonexhaustive => {
                "__Nonexhaustive: This value should never exist. If you are seeing this, it may be \
                 a result of corrupt data, or an attempt to crash the program with bad data."
            }
        }
    }

    #[allow(trivial_casts)]
    fn cause(&self) -> Option<&Error> {
        use self::RealError::*;
        match *self {
            // IoError(ref err) => err.cause(),
            __Nonexhaustive => None,
        }
    }
}
