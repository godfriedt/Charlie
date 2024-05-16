use std::env;

mod listener;
mod connect;

fn main() {

    let args: Vec<String> = env::args().collect();

    let socket_function = match args[1].as_str() {
        "listen" => listener::socket_listen,
        "connect" => connect::socket_connect,
        _ => panic!("Please choose either -c or -s to start the client or server respectively.")
    };

    socket_function(&args[2], &args[3]);

}
