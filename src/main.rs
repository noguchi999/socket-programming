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
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];
}