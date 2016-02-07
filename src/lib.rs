#![feature(custom_derive, plugin, deprecated)]
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

extern crate bincode;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
mod test;

use std::convert::{From, AsRef};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use bincode::{SizeLimit, serde as bin};
use bincode::serde::{serialize as bin_serialize, deserialize as bin_deserialize};
use serde_json::{to_string as json_serialize, from_str as json_deserialize, error as json};

/// An error that has been made seralization-capable, but has lost it's fields, due to incompatibility with the library or preserving fields being unnesacary.
///
/// See the crate root for more info.
#[derive(Clone, Serialize, Deserialize, Debug, RustcEncodable, RustcDecodable)]
pub struct PseudoError {
    cause: Option<Box<PseudoError>>,
    desc: String,
    disp: String,
}

impl Display for PseudoError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.disp)
    }
}

impl<'a> From<&'a Error> for PseudoError {
    fn from(err: &Error) -> PseudoError {
        PseudoError {
            cause: err.cause().map(|cause| Box::new(PseudoError::from(cause))), /* In prevous versions of this line, it was almost lisp with the number of close perens here. */
            desc: err.description().to_owned(),
            disp: format!("{}", err),
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
        // &Box<_> -> Box<_> -> _ -> &_
        // From there it's just &PseudoError as &Error.
    }
}

/// Serializes any type implementing Error to a string that can be deseralized with deserialize_bytes.
pub fn serialize_error_bytes(to_ser: &Error) -> Vec<u8> {
    bin_serialize(&PseudoError::from(to_ser), SizeLimit::Infinite).unwrap()
}

/// Serializes any type implementing Error to a string that can be deseralized with deserialize_string.
pub fn serialize_error_string(to_ser: &Error) -> String {
    json_serialize(&PseudoError::from(to_ser)).unwrap()
}

/// Deseralizes a slice of bytes into a SeralizableError.
pub fn deserialize_error_bytes(to_de: &[u8]) -> Result<PseudoError, bin::DeserializeError> {
    bin_deserialize(to_de)
}

/// Deseralizes a string to a SeralizableError.
pub fn deserialize_error_string(to_de: &str) -> Result<PseudoError, json::Error> {
    json_deserialize(to_de)
}
