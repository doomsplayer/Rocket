#![feature(proc_macro,unboxed_closures,fn_traits)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_macro;

use rocket_macro::post;
use rocket::http::Cookies;
use rocket::request::Form;

#[derive(FromForm)]
struct User<'a> {
    name: &'a str,
    nickname: String,
}

#[post("/<name>?<query>", format = "application/json", data = "<user>", rank = 2)]
fn get<'r>(name: &str,
           query: User<'r>,
           user: Form<'r, User<'r>>,
           cookies: &Cookies)
           -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
