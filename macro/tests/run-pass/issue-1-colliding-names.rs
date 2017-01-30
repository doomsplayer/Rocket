#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/<todo>")]
fn todo(todo: &str) -> &str {
    todo
}

fn main() {
    let _ = routes![todo];
}
