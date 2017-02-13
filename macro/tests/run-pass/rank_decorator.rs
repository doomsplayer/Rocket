#![feature(proc_macro,unboxed_closures,fn_traits)]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/", rank = 1)]
fn get1() -> &'static str {
    "hi"
}

#[get("/", rank = 2)]
fn get2() -> &'static str {
    "hi"
}

#[get("/", rank = 3)]
fn get3() -> &'static str {
    "hi"
}

fn main() {}
