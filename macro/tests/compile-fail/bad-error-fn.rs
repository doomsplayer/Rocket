#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket::{Error, Request};
use rocket_macro::error;

#[error(404)]
fn err_a(_a: Error, _b: Request, _c: Error) -> &'static str {
    "hi"
}
//~^ ERROR: can have at most 2

#[error(404)]
fn err_b(_a: (isize, usize)) -> &'static str {
    "hi"
}
//~^ ERROR: unexpected error handler argument

fn main() {}
