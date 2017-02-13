#![feature(proc_macro,unboxed_closures,fn_traits)]
extern crate rocket;
extern crate rocket_macro;

use std::path::PathBuf;
use rocket::http::uri::SegmentError;
use rocket_macro::post;

#[post("/<a>/<b..>")]
fn get(a: String, b: PathBuf) -> String {
    format!("{}/{}", a, b.to_string_lossy())
}

#[post("/<a>/<b..>")]
fn get2(a: String, b: Result<PathBuf, SegmentError>) -> String {
    format!("{}/{}", a, b.unwrap().to_string_lossy())
}

fn main() {}


