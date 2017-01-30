#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get(1)]  //~ ERROR expected `path = string`
fn get0() -> &'static str {
    "hi"
}

#[get(path = 1)]  //~ ERROR must be a string
fn get1() -> &'static str {
    "hi"
}

#[get(path = "h", rank = "2")]  //~ ERROR must be an int
fn get2() -> &'static str {
    "hi"
}

#[get(path = "h", format = 100)]  //~ ERROR must be a "content/type"
fn get3() -> &'static str {
    "hi"
}

fn main() {}
