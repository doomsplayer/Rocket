#![feature(proc_macro,unboxed_closures,fn_traits)]
extern crate rocket;
#[macro_use]
extern crate rocket_macro;

use rocket_macro::post;

#[post("/", format = "application/x-custom")]
fn get() -> &'static str {
    "hi"
}

fn main() {}
