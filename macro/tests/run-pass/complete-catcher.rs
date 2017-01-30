#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket::{Error, Request};
use rocket_macro::error;

fn main() {
        let _ = errors![err2b];
}

#[error(404)]
fn err2b<'a>(_err: Error, _req: &'a Request<'a>) -> &'a str {
    "hi"
}
