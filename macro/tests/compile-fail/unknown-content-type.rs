#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/", format = "application/x-custom")] //~ WARNING not a known content-type
fn one() -> &'static str {
    "hi"
}

#[get("/", format = "x-custom/plain")] //~ WARNING not a known content-type
fn two() -> &'static str {
    "hi"
}

#[get("/", format = "x-custom/x-custom")] //~ WARNING not a known content-type
fn three() -> &'static str {
    "hi"
}

// Make the test fail here so we can actually check for the warnings above.
assert!(false);

fn main() {}
