#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::route;

#[route(CONNECT, "hello")]  //~ ERROR valid HTTP method
fn get() -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
