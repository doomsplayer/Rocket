#![feature(proc_macro,unboxed_closures,fn_traits)]
extern crate rocket;
extern crate rocket_macro;

use rocket::{Error, Request};
use rocket_macro::error;

#[error(404)]
pub fn err0() -> &'static str {
    "hi"
}

#[error(404)]
pub fn err1a(_err: Error) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err1b<'a>(_req: &Request<'a>) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err2a<'a>(_err: Error, _req: &Request<'a>) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err2b<'a>(_err: Error, _req: &'a Request<'a>) -> &'a str {
    "hi"
}

fn main() {}
