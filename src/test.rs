use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Helper function for testing seralization generically.
///
/// Passes if err pseudo-equals errdeser, and both lack a cause.
fn help_test_ser_no_cause(err: &Error, errdeser: &Error) {
    assert!(err.cause().is_none(),
            "Err has cause when cause should be none. Are you sure you are using this helper \
             function correctly?");
    assert!(errdeser.cause().is_none(),
            "Errdeser gained cause during serialization.");
    assert!(err.description() == errdeser.description(),
            "Error descriptions did not match! err = {}, errdeser = {}",
            err.description(),
            errdeser.description());
    let errdis = format!("{}", err);
    let errdeserdis = format!("{}", errdeser);
    assert!(errdis == errdeserdis,
            "Error displays did not match! err = {}, errdeser = {}",
            errdis,
            errdeserdis);
}

/// Helper function for testing seralization generically.
///
/// Passes if err pseudo-equals errdeser, and have a cause that also pseudo-equal eachother.
/// The cause of the causes should be also be None.
fn help_test_ser_some_cause(err: &Error, errdeser: &Error) {
    assert!(err.cause().is_some(),
            "Err lacked a cause. Are you sure you are using this helper function correctly?");
    assert!(errdeser.cause().is_some(),
            "Errdeser.cause lost during serialization.");
    assert!(err.description() == errdeser.description(),
            "Error descriptions did not match! err = {}, errdeser = {}",
            err.description(),
            errdeser.description());
    let errdis = format!("{}", err);
    let errdeserdis = format!("{}", errdeser);
    assert!(errdis == errdeserdis,
            "Error displays did not match! err = {}, errdeser = {}",
            errdis,
            errdeserdis);
    let cause = err.cause().unwrap();
    let desercause = errdeser.cause().unwrap();
    help_test_ser_no_cause(cause, desercause);
}

#[derive(Debug)]
struct TestError;

impl Display for TestError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "Lesser display")
    }
}

impl Error for TestError {
    fn description(&self) -> &str {
        "Lesser description"
    }
}

#[test]
fn serialize_error_bytes_no_cause() {
    let err = TestError;
    let bytes = super::serialize_error_bytes(&err);
    let errdeser = super::deserialize_error_bytes(&bytes).expect("Error while deserializing.");
    help_test_ser_no_cause(&err, &errdeser);
}

#[test]
fn serialize_error_string_no_cause() {
    let err = TestError;
    let bytes = super::serialize_error_string(&err);
    let errdeser = super::deserialize_error_string(&bytes).expect("Error while deserializing.");
    help_test_ser_no_cause(&err, &errdeser);
}

#[derive(Debug)]
struct TestCauseError(TestError);

impl Display for TestCauseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "Greater display")
    }
}

impl Error for TestCauseError {
    fn description(&self) -> &str {
        "Greater description"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.0)
    }
}

#[test]
fn serialize_error_bytes_some_cause() {
    let err = TestCauseError(TestError);
    let bytes = super::serialize_error_bytes(&err);
    let errdeser = super::deserialize_error_bytes(&bytes).expect("Error while deserializing.");
    help_test_ser_some_cause(&err, &errdeser);
}

#[test]
fn serialize_error_string_some_cause() {
    let err = TestCauseError(TestError);
    let bytes = super::serialize_error_string(&err);
    let errdeser = super::deserialize_error_string(&bytes).expect("Error while deserializing.");
    help_test_ser_some_cause(&err, &errdeser);
}
