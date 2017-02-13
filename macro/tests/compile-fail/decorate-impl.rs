#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("")]        //~ ERROR can only be used on functions
impl C for A {} //~ ERROR but was applied

fn main() {
    let _ = routes![get];
}
