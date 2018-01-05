#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(conservative_impl_trait)]

#![recursion_limit="128"] #[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;
extern crate futures;
extern crate tokio_core;

pub mod database_connection;
pub mod models;
pub mod slack_command;
pub mod schema;
pub mod stream;
pub mod myerror;


