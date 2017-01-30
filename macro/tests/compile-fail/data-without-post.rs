#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

use rocket::Data;

#[get("/", data = "<something>")]
//~^ ERROR payload supporting methods
fn get(something: Data) -> &'static str {
    "hi"
}

fn main() {}
