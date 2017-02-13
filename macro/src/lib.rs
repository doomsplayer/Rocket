//! # Rocket - Code Generation
//!
//! This crate implements the code generation portions of Rocket. This includes
//! custom derives, custom attributes, and procedural macros. The documentation
//! here is purely technical. The code generation facilities are documented
//! thoroughly in the [Rocket programming guide](https://rocket.rs/guide).
//!
//! ## Custom Attributes
//!
//! This crate implements the following custom attributes:
//!
//!   * **route**
//!   * **get**
//!   * **put**
//!   * **post**
//!   * **delete**
//!   * **head**
//!   * **patch**
//!   * **error**
//!
//! The grammar for all _route_ attributes, including **route**, **get**,
//! **put**, **post**, **delete**, **head**, and **patch**, is defined as:
//!
//! <pre>
//! route := METHOD? '(' ('path' '=')? path (',' kv_param)* ')'
//!
//! path := URI_SEG
//!       | DYNAMIC_PARAM
//!       | '?' DYNAMIC_PARAM
//!       | path '/' path
//!       (string literal)
//!
//! kv_param := 'rank' '=' INTEGER
//!           | 'format' '=' STRING
//!           | 'data' '=' DYNAMIC_PARAM
//!
//! INTEGER := isize, as defined by Rust
//! STRING := UTF-8 string literal, as defined by Rust
//! IDENT := Valid identifier, as defined by Rust
//!
//! URI_SEG := Valid HTTP URI Segment
//! DYNAMIC_PARAM := '<' IDENT '..'? '>' (string literal)
//! </pre>
//!
//! Note that the **route** attribute takes a method as its first argument,
//! while the remaining do not. That is, **route** looks like:
//!
//!     #[route(GET, path = "/hello")]
//!
//! while the equivalent using **get** looks like:
//!
//!     #[get("/hello")]
//!
//! The syntax for the **error** attribute is:
//!
//! <pre>
//! error := INTEGER
//! </pre>
//!
//! ## Custom Derives
//!
//! This crate implements the following custom derives:
//!
//!   * **FromForm**
//!
//! ## Procedural Macros
//!
//! This crate implements the following procedural macros:
//!
//!   * **routes**
//!   * **errors**
//!
//! The syntax for both of these is defined as:
//!
//! <pre>
//! macro := PATH (',' macro)*
//!
//! PATH := a path, as defined by Rust
//! </pre>
//!
//! # Debugging Codegen
//!
//! When the `ROCKET_CODEGEN_DEBUG` environment variable is set, this crate logs
//! the items it has generated to the console at compile-time. For example, you
//! might run the following to build a Rocket application with codegen logging
//! enabled:
//!
//! ```
//! ROCKET_CODEGEN_DEBUG=1 cargo build
//! ```

#![crate_type = "dylib"]
#![feature(proc_macro, conservative_impl_trait)]
#![recursion_limit="1000"]

#[macro_use]
extern crate log;
extern crate rocket;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate error_chain;
extern crate result;

mod errors;
mod utils;
mod decorators;

use proc_macro::TokenStream;

const USER_FN_PREFIX: &'static str = "_rocket_user_function_";
const PARAM_PREFIX: &'static str = "_rocket_param_";

macro_rules! export_decorators {
    ($($name:ident => $func:ident),+) => (
        $(
            #[proc_macro_attribute]
            pub fn $name(args: TokenStream, item: TokenStream) -> TokenStream {
                let result = match decorators::$func(args, item) {
                    Ok(ts) => ts,
                    Err(e) => panic!("{}", e),
                };
                debug!("{} decorator output: {}", stringify!($name), result);
                result
            }
         )+
    )
}

export_decorators!(
    error => error_decorator,
    route => route_decorator,
    get => get_decorator,
    put => put_decorator,
    post => post_decorator,
    delete => delete_decorator,
    head => head_decorator,
    patch => patch_decorator
    // TODO: Allow this once Diesel incompatibility is fixed. Fix docs too.
    // "options" => options_decorator
);


#[proc_macro_derive(FromForm)]
pub fn from_form(input: TokenStream) -> TokenStream {
    let result = match decorators::from_form_derive(input) {
        Ok(ts) => ts,
        Err(e) => panic!("{}", e),
    };
    debug!("FromForm output: {}", result);
    result
}
