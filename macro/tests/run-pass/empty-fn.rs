#![feature(proc_macro,unboxed_closures,fn_traits)]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("")]
fn get() -> &'static str {
    "hi"
}

#[get("/")]
fn get_empty() {}

fn main() {}
