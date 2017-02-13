#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/")]
fn get(_: &str) -> &'static str {
    "hi"
} //~ ERROR argument

fn main() {}
