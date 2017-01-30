#![feature(proc_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::get;

#[get("/", format = "applicationx-custom")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn one() -> &'static str {
    "hi"
}

#[get("/", format = "")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn two() -> &'static str {
    "hi"
}

#[get("/", format = "//")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn three() -> &'static str {
    "hi"
}

#[get("/", format = "/")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn four() -> &'static str {
    "hi"
}

#[get("/", format = "a/")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn five() -> &'static str {
    "hi"
}

#[get("/", format = "/a")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn six() -> &'static str {
    "hi"
}

#[get("/", format = "/a/")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn seven() -> &'static str {
    "hi"
}

#[get("/", format = "a/b/")] //~ ERROR malformed
//~^ ERROR `format` must be a "content/type"
fn eight() -> &'static str {
    "hi"
}

fn main() {}
