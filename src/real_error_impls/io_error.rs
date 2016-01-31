use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::ops::Deref;
use std::io;

use SeralizableError;

#[derive(Clone, Serialize, Deserialize, RustcEncodable, RustcDecodable, Debug)]
pub struct SerializableIoError {
    repr: Repr,
}

impl Display for SerializableIoError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Repr::*;
        match self.repr {
            Os(_num) => unimplemented!(),
            Custom(ref cs) => io::Error::new(cs.kind.clone().into(), (*cs.error).clone()).fmt(fmt),
        }
    }
}

impl Error for SerializableIoError {
    fn description(&self) -> &str {
        use self::Repr::*;
        match self.repr {
            Os(_num) => unimplemented!(),
            Custom(ref cs) => io::Error::new(cs.kind.clone().into(), (*cs.error).clone()).description(),
            // This doesn't yet borrow check, because I need a reference, but the value is dropped when I return.
            // Perhaps this could be solved by having a global set of io::Errors?
            // That would have to be a minor memory leak, though, since I have to return a &str, which can't be refernce counted or the like.
        }
    }

    fn cause(&self) -> Option<&Error> {
        use self::Repr::*;
        match self.repr {
            Os(_num) => unimplemented!(),
            Custom(ref cs) => io::Error::new(cs.kind.clone().into(), (*cs.error).clone()).cause(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, RustcEncodable, RustcDecodable, Debug)]
enum Repr {
    Os(i32),
    Custom(Custom),
}

#[derive(Clone, Serialize, Deserialize, RustcEncodable, RustcDecodable, Debug)]
struct Custom {
    kind: SerializableIoErrorKind,
    error: Box<SeralizableError>,
}

/// A 1:1 copy of std::io::ErrorKind, but implementing seralization methods.
#[derive(Serialize, Deserialize, RustcEncodable, RustcDecodable, Debug, Clone)]
pub enum SerializableIoErrorKind {
    /// An entity was not found, often a file.
    NotFound,

    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,

    /// The connection was refused by the remote server.
    ConnectionRefused,

    /// The connection was reset by the remote server.
    ConnectionReset,

    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,

    /// The network operation failed because it was not connected yet.
    NotConnected,

    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,

    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,

    /// The operation failed because a pipe was closed.
    BrokenPipe,

    /// An entity already exists, often a file.
    AlreadyExists,

    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,

    /// A parameter was incorrect.
    InvalidInput,

    /// Data not valid for the operation were encountered.
    ///
    /// Unlike `InvalidInput`, this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    InvalidData,

    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,

    /// An error returned when an operation could not be completed because a
    /// call to `write` returned `Ok(0)`.
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    WriteZero,

    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,

    /// Any I/O error not part of this list.
    Other,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,

    /// This ensures that you cannot match the whole enum, and must always consiter for more feilds in the furure.
    ///
    /// This should also never exist in usage, and if it is, consuming it will return some error.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<SerializableIoErrorKind> for io::ErrorKind {
    fn from(err: SerializableIoErrorKind) -> io::ErrorKind {
        // I really hated having to type this manually, even with autocomplete.
        use self::SerializableIoErrorKind::*;
        use std::io::ErrorKind as Ek;
        match err {
            NotFound => Ek::NotFound,
            PermissionDenied => Ek::PermissionDenied,
            ConnectionRefused => Ek::ConnectionRefused,
            ConnectionReset => Ek::ConnectionReset,
            ConnectionAborted => Ek::ConnectionAborted,
            NotConnected => Ek::NotConnected,
            AddrInUse => Ek::AddrInUse,
            AddrNotAvailable => Ek::AddrNotAvailable,
            BrokenPipe => Ek::BrokenPipe,
            AlreadyExists => Ek::AlreadyExists,
            WouldBlock => Ek::WouldBlock,
            InvalidInput => Ek::InvalidInput,
            InvalidData => Ek::InvalidData,
            TimedOut => Ek::TimedOut,
            WriteZero => Ek::WriteZero,
            Interrupted => Ek::Interrupted,
            Other => Ek::Other,
            UnexpectedEof => Ek::UnexpectedEof,
            __Nonexhaustive => Ek::__Nonexhaustive,
            // Yes, I know, internals. I'm just preserving what is already there.
            // It's not supposed to be there anyway, so what's the harm?
        }
    }
}

impl From<io::ErrorKind> for SerializableIoErrorKind {
    fn from(err: io::ErrorKind) -> SerializableIoErrorKind {
        // Luckally, with this one, I can just copy and paste, just changing some use values.
        use self::SerializableIoErrorKind as Ek;
        use std::io::ErrorKind::*;
        match err {
            NotFound => Ek::NotFound,
            PermissionDenied => Ek::PermissionDenied,
            ConnectionRefused => Ek::ConnectionRefused,
            ConnectionReset => Ek::ConnectionReset,
            ConnectionAborted => Ek::ConnectionAborted,
            NotConnected => Ek::NotConnected,
            AddrInUse => Ek::AddrInUse,
            AddrNotAvailable => Ek::AddrNotAvailable,
            BrokenPipe => Ek::BrokenPipe,
            AlreadyExists => Ek::AlreadyExists,
            WouldBlock => Ek::WouldBlock,
            InvalidInput => Ek::InvalidInput,
            InvalidData => Ek::InvalidData,
            TimedOut => Ek::TimedOut,
            WriteZero => Ek::WriteZero,
            Interrupted => Ek::Interrupted,
            Other => Ek::Other,
            UnexpectedEOF => Ek::UnexpectedEof, // I Implicitly transform this to the new version, since the old one is deprecated and has a new value.
            UnexpectedEof => Ek::UnexpectedEof,
            __Nonexhaustive => Ek::__Nonexhaustive,
        }
    }
}
