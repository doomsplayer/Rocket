#![feature(specialization)]
#![feature(conservative_impl_trait)]
#![feature(drop_types_in_const)]
#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(type_ascription)]

//! # Rocket - Core API Documentation
//!
//! Hello, and welcome to the core Rocket API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//! There's an [overview](https://rocket.rs/overview) of Rocket on the main site
//! as well as a [full, detailed guide](https://rocket.rs/guide). If you'd like
//! pointers on getting started, see the
//! [quickstart](https://rocket.rs/guide/quickstart) or [getting
//! started](https://rocket.rs/guide/getting-started) chapters of the guide.
//!
//! You may also be interested in looking at the [contrib API
//! documentation](../rocket_contrib), which contains JSON and templating
//! support.
//!
//! ## Libraries
//!
//! Rocket's functionality is split into three crates:
//!
//!   1. [Core](/rocket) - The core library. Needed by every Rocket application.
//!   2. [Codegen](/rocket_codegen) - Core code generation plugin. Should always
//!      be used alongsize `rocket`, though it's not necessary.
//!   3. [Contrib](/rocket_contrib) - Provides useful functionality for many
//!      Rocket application. Completely optional.
//!
//! ## Usage
//!
//! The sanctioned way to use Rocket is via the code generation plugin. This
//! makes Rocket easier to use and allows a somewhat stable API as Rocket
//! matures. To use Rocket with the code generation plugin in your Cargo-based
//! project, add the following to `Cargo.toml`:
//!
//! ```rust,ignore
//! [dependencies]
//! rocket = "*"
//! rocket_codegen = "*"
//! ```
//!
//! If you'll be deploying your project to [crates.io](https://crates.io),
//! you'll need to change the "*" to the current version of Rocket.
//!
//! Then, add the following to the top of your `main.rs` file:
//!
//! ```rust
//! #![feature(plugin)]
//! #![plugin(rocket_codegen)]
//!
//! extern crate rocket;
//! ```
//!
//! See the [guide](https://rocket.rs/guide) for more information on how to
//! write Rocket applications. Here's a simple example to get you started:
//!
//! ```rust
//! #![feature(plugin)]
//! #![plugin(rocket_codegen)]
//!
//! extern crate rocket;
//!
//! #[get("/")]
//! fn hello() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! fn main() {
//! # if false { // We don't actually want to launch the server in an example.
//!     rocket::ignite().mount("/", routes![hello]).launch()
//! # }
//! }
//! ```
//!
//! ## Configuration
//!
//! Rocket and Rocket libraries are configured via the `Rocket.toml` file. For
//! more information on how to configure Rocket, see the [configuration
//! section](https://rocket.rs/guide/getting-started/#configuration) of the
//! guide as well as the [config](config) module documentation.
//!
//! ## Testing
//!
//! Rocket includes a small testing library that can be used to test your Rocket
//! application. For information on how to test your Rocket applications, the
//! [testing module](testing) documentation.
//!

#[macro_use] extern crate log;
extern crate term_painter;
extern crate hyper;
extern crate url;
extern crate toml;
extern crate num_cpus;
extern crate state;
extern crate cookie;
extern crate time;

#[cfg(test)] #[macro_use] extern crate lazy_static;

#[doc(hidden)] #[macro_use] pub mod logger;
#[cfg(any(test, feature = "testing"))] pub mod testing;
pub mod http;
pub mod request;
pub mod response;
pub mod outcome;
pub mod config;
pub mod data;
pub mod handler;

mod error;
mod router;
mod rocket;
mod codegen;
mod catcher;
mod ext;

#[doc(inline)] pub use response::Response;
#[doc(inline)] pub use handler::{Handler, ErrorHandler};
#[doc(inline)] pub use logger::LoggingLevel;
#[doc(hidden)] pub use codegen::{StaticRouteInfo, StaticCatchInfo};
#[doc(inline)] pub use outcome::Outcome;
#[doc(inline)] pub use data::Data;
pub use router::Route;
pub use request::{Request, State};
pub use error::Error;
pub use catcher::Catcher;
pub use rocket::Rocket;

/// Alias to [Rocket::ignite()](/rocket/struct.Rocket.html#method.ignite).
/// Creates a new instance of `Rocket`.
pub fn ignite() -> Rocket {
    Rocket::ignite()
}

/// Alias to [Rocket::custom()](/rocket/struct.Rocket.html#method.custom).
/// Creates a new instance of `Rocket` with a custom configuration.
pub fn custom(config: config::Config, log: bool) -> Rocket {
    Rocket::custom(config, log)
}


#[macro_export]
macro_rules! routes {
    ($($r:expr),*) => (
        vec![$(::rocket::Route::from(&$r)),*]
    )
}

#[macro_export]
macro_rules! errors {
    ($($c:expr),*) => (
        vec![$(::rocket::Catcher::from(&$c)),*]
    )
}

