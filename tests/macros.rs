//! Smoke tests for `define_app_error!`, `define_app_error_static!`, and
//! `impl_from_error!`. Each test verifies that the macro expands to a usable
//! type with the expected trait surface.

use std::error::Error;

use minimer::{define_app_error, define_app_error_static, impl_from_error};

define_app_error!(pub MyAppError);
define_app_error_static!(pub MyAppErrorStatic);

impl_from_error!(MyAppError, std::env::VarError);
impl_from_error!(MyAppErrorStatic, std::env::VarError);

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
