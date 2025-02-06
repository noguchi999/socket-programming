use std::env;
#[macro_use]
extern crate log;

mod tcp_server;
mod tcp_client;
mod udp_server;
mod udp_client;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
}