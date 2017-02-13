#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/<name>")] //~ ERROR 'name' is declared
fn get(other: &str) -> &'static str {
    "hi"
} //~ ERROR isn't in the function

#[get("/a?<r>")] //~ ERROR 'r' is declared
fn get1() -> &'static str {
    "hi"
} //~ ERROR isn't in the function

#[post("/a", data = "<test>")] //~ ERROR 'test' is declared
fn post() -> &'static str {
    "hi"
} //~ ERROR isn't in the function

fn main() {}
