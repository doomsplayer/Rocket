#![feature(proc_macro)]
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
