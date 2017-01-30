#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/test/<one>/<two>/<three>")]
fn get(one: &str, two: usize, three: isize) -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
