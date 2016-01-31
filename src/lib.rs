#![feature(custom_derive, plugin)]
#![plugin(serde_macros, clippy)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications,
        warnings, clippy)]

//! Serializes and psudo-deserializes structs implementing the Error trait. This should only be done if you know you won't ever need an actual error value, only the description and causes.
//!
//! This crate assumes that the error descriptions are complete, and are indicative of the error going on.
//! There is also no way to get any values accoated with the error, since that would require real serialization of arbratrary structs, implementing a trait you didn't make.
//!
//! #Examples
//!
//! ```
//! use std::convert::From;
//! use std::fmt::Error;
//! use errorser::serialize_error_string;
//!
//! let error = Error;
//! let errser = serialize_error_string(&error);
//! assert_eq!(errser, "???");
//! ```

extern crate bincode;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

mod real_error_impls;

use std::convert::{From, AsRef};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use bincode::SizeLimit;
use bincode::serde::serialize as bin_serialize;
use serde_json::{to_string as json_serialize, from_string as json_deserialize, error as json};

pub use real_error_impls::RealError;

/// An error that can be serialized by serde or rustc_seralize.
#[derive(Clone, Serialize, Deserialize, Debug, RustcEncodable, RustcDecodable)]
pub enum SeralizableError {
    /// If the error is commonly used, such as something from std, some of it's fields may be preserved. This variant represents these kinds of errors.
    RealError(RealError),
    /// If the error isn't in std or isn't common, it is automatically made a PseudoError, loosing it's fields.
    PseudoError(PseudoError),
}

impl Display for SeralizableError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use SeralizableError::*;
        match *self {
            RealError(ref _rerr) => unimplemented!(),
            PseudoError(ref perr) => write!(fmt, "{}", perr),
        }
    }
}

impl<'a> From<&'a Error> for SeralizableError {
    fn from(err: &Error) -> SeralizableError {
        // TODO: Check if can be seralized as RealError.
        SeralizableError::PseudoError(PseudoError::from(err))
    }
}

impl Error for SeralizableError {
    fn description(&self) -> &str {
        use SeralizableError::*;
        match *self {
            RealError(ref _rerr) => unimplemented!(),
            PseudoError(ref perr) => perr.description(),
        }
    }

    #[allow(trivial_casts)]
    fn cause(&self) -> Option<&Error> {
        use SeralizableError::*;
        match *self {
            RealError(ref _rerr) => unimplemented!(),
            PseudoError(ref perr) => perr.cause(),
        }
    }
}

/// An error that has been made seralization-capable, but has lost it's fields, due to incompatibility with the library or preserving fields being unnesacary.
///
/// See the crate root for more info.
#[derive(Clone, Serialize, Deserialize, Debug, RustcEncodable, RustcDecodable)]
pub struct PseudoError {
    cause: Option<Box<PseudoError>>,
    desc: String,
}

impl Display for PseudoError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.desc)
    }
}

impl<'a> From<&'a Error> for PseudoError {
    fn from(err: &Error) -> PseudoError {
        PseudoError {
            cause: err.cause().map(|cause| Box::new(PseudoError::from(cause))), /* In prevous versions of this line, it was almost lisp with the number of close perens here. */
            desc: err.description().to_owned(),
        }
    }
}

impl Error for PseudoError {
    fn description(&self) -> &str {
        &self.desc
    }

    #[allow(trivial_casts)]
    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|cause| &**cause as &Error)    // Good luck. Also &**cause looks like it's cencoring profanity. Kinda feels like it would too, with how complacated it is.
        // Actually, I'm not that mean, I'll try to explain.

        // So cause.as_ref is Option::as_ref, turning Option<Box<PseudoError>> into Option<&Box<PseudoError>>.
        // Mapping lets me take an &Box<_>.
        // &**cause is best read right to left. The transforms work like this:
        // &Box<_> -> Box<_> -> _ -> &PseudoError
        // From there it's just &PseudoError as &Error.
    }
}

/// Serializes any type implementing Error to a string that can be deseralized with deserialize_bytes.
///
/// #Examples
///
/// ```
/// use std::convert::From;
/// use std::fmt::Error;
/// use errorser::serialize_error_bytes;
///
/// let error = Error;
/// let errser = serialize_error_bytes(&error);
/// assert_eq!(errser, [0, 1]);
/// ```
pub fn serialize_error_bytes(to_ser: &Error) -> Vec<u8> {
    bin_serialize(&SeralizableError::from(to_ser), SizeLimit::Infinite).unwrap()
}

/// Serializes any type implementing Error to a string that can be deseralized with deserialize_string.
///
/// #Examples
///
/// ```
/// use std::convert::From;
/// use std::fmt::Error;
/// use errorser::serialize_error_string;
///
/// let error = Error;
/// let errser = serialize_error_string(&error);
/// assert_eq!(errser, "???");
/// ```
pub fn serialize_error_string(to_ser: &Error) -> String {
    json_serialize(&SeralizableError::from(to_ser)).unwrap()
}

pub fn deserialize_error_string(to_de: &str) -> Result<SeralizableError, json::Error> {
    json_deserialize(to_de)
}

#[cfg(test)]
mod test {
    #[test]
    fn serialize_error_bytes() {}
}
