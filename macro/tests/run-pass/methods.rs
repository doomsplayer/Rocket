#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macro;

use rocket_macro::{route, patch, head, delete, post, get, put};

#[get("/")]
fn get() {}
#[route(GET, "/")]
fn get_r() {}

#[put("/")]
fn put() {}
#[route(PUT, "/")]
fn put_r() {}

#[post("/")]
fn post() {}
#[route(POST, "/")]
fn post_r() {}

#[delete("/")]
fn delete() {}
#[route(DELETE, "/")]
fn delete_r() {}

#[head("/")]
fn head() {}
#[route(HEAD, "/")]
fn head_r() {}

#[patch("/")]
fn patch() {}
#[route(PATCH, "/")]
fn patch_r() {}

// TODO: Allow this once Diesel incompatibility is fixed.
// #[options("/")] fn options() {  }
#[route(OPTIONS, "/")]
fn options_r() {}

fn main() {}
