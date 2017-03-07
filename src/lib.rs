#![cfg_attr(feature="flame_it", feature(plugin,custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;

extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

extern crate threadpool;

extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;

extern crate rand;
extern crate random_choice;

extern crate libc;

extern crate chrono;

#[macro_use]
extern crate chan;
extern crate chan_signal;

mod server;
mod job_handler;
pub mod errors;
mod job;
mod utils;
mod worker;
mod middleware;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;


pub use server::SidekiqServer;
pub use job_handler::{JobHandler, JobHandlerResult, printer_handler, error_handler, panic_handler};
pub use middleware::{MiddleWare, MiddleWareResult, peek_middleware, retry_middleware,
                     time_elapse_middleware, NextFunc};
pub use job::{Job, RetryInfo};
pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Debug, Clone)]
pub enum JobSuccessType {
    Success,
    Ignore,
}