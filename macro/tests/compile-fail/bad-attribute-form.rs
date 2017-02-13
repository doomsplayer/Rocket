#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get] //~ ERROR incorrect use of attribute
//~^ ERROR malformed attribute
fn get() -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
