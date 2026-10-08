//! Smoke tests for `define_app_error!`, `define_app_error_static!`, and
//! `impl_from_error!`. Each test verifies that the macro expands to a usable
//! type with the expected trait surface.

use std::error::Error;

use minimer::{define_app_error, define_app_error_static};

define_app_error!(pub MyAppError);
define_app_error_static!(pub MyAppErrorStatic);

#[test]
fn newtype_constructors_work() {
    let err: MyAppError = MyAppError::new("test message");
    assert!(format!("{}", err).contains("test message"));
}

#[test]
fn newtype_from_string_works() {
    let err: MyAppError = MyAppError::from("string-form".to_string());
    assert!(format!("{}", err).contains("string-form"));
}

#[test]
fn newtype_from_str_works() {
    let err: MyAppError = MyAppError::from("str-form");
    assert!(format!("{}", err).contains("str-form"));
}

#[test]
fn newtype_implements_error_trait() {
    fn assert_error<E: Error>(_: &E) {}
    let err: MyAppError = MyAppError::new("x");
    assert_error(&err);
}

#[test]
fn registered_from_impl_enables_question_mark() {
    fn try_get_var() -> Result<String, MyAppError> {
        Ok(std::env::var("DEFINITELY_NOT_SET_BIRZL")?)
    }
    let result: Result<String, MyAppError> = try_get_var();
    assert!(result.is_err());
}

#[test]
fn static_newtype_constructors_work() {
    let err: MyAppErrorStatic = MyAppErrorStatic::new("static test");
    assert!(format!("{}", err).contains("static test"));
}

#[test]
fn static_newtype_implements_error_trait() {
    fn assert_error<E: Error>(_: &E) {}
    let err: MyAppErrorStatic = MyAppErrorStatic::new("x");
    assert_error(&err);
}

#[test]
fn deref_to_inner_app_error_works() {
    let err: MyAppError = MyAppError::new("deref test");
    let _message: &String = &err.message;
}

#[test]
fn parse_int_error_question_mark_works() {
    fn try_parse() -> Result<u64, MyAppError> {
        Ok("not a number".parse::<u64>()?)
    }
    assert!(try_parse().is_err());
}

#[test]
fn io_error_question_mark_works() {
    fn try_read() -> Result<String, MyAppError> {
        Ok(std::fs::read_to_string("/nonexistent/path/birzl")?)
    }
    assert!(try_read().is_err());
}

#[test]
fn utf8_error_question_mark_works() {
    fn try_decode() -> Result<String, MyAppError> {
        let bytes: &[u8] = &[0xff, 0xfe, 0xfd];
        Ok(std::str::from_utf8(bytes)?.to_string())
    }
    assert!(try_decode().is_err());
}

#[test]
fn static_newtype_from_app_error_keeps_the_sub_error_chain_as_text() {
    #[derive(Debug)]
    struct Outer(std::io::Error);

    impl std::fmt::Display for Outer {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("reading the shard failed")
        }
    }

    impl Error for Outer {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.0)
        }
    }

    let err: MyAppError = MyAppError::from_error("loading the course failed", Box::new(Outer(std::io::Error::other("disk unplugged"))));
    let static_err: MyAppErrorStatic = MyAppErrorStatic::from(err.0);

    assert!(static_err.message.ends_with("loading the course failed -> reading the shard failed -> disk unplugged"));
}

#[test]
fn static_newtype_from_app_error_without_sub_error_keeps_the_message() {
    let err: MyAppError = MyAppError::new("plain failure");
    let static_err: MyAppErrorStatic = MyAppErrorStatic::from(err.0);

    assert!(static_err.message.ends_with("plain failure"));
}
