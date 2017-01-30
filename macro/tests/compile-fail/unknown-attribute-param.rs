#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get(path = "hello", unknown = 123)]  //~ ERROR 'unknown' is not a known param
fn get() -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
