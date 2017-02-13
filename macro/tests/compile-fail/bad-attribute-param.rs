#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

#[get(path = "hello", 123)]  //~ ERROR expected
fn get() -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
