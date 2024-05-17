#![feature(str_split_remainder)]

use std::env;

mod listener;
mod connect;

fn main() {

    // get args
    let args: Vec<String> = env::args().collect();

    // driver code
    let socket_function = match args[1].as_str() {
        "listen" => listener::socket_listen,
        "connect" => connect::socket_connect,
        _ => panic!("Please choose either listen or connect to start the listener or connector respectively.")
    };

    // call function
    socket_function(&args[2], &args[3]);
}
