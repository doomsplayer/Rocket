#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::route;

#[route(FIX, "hello")]  //~ ERROR is not a valid HTTP method
//~^ ERROR valid HTTP method
fn get() -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
