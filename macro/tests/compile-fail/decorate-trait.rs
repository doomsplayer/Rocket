#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("")]   //~ ERROR can only be used on functions
trait C {} //~ ERROR but was applied

fn main() {
    let _ = routes![get];
}
