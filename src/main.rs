use std::env;

mod server;
mod client;

fn main() {

    let args: Vec<String> = env::args().collect();

    let which = match args[1].as_str() {
        "-s" => server::start_server,
        "-c" => client::start_client,
        _ => panic!("Please choose either -c or -s to start the client or server respectively.")
    };

    which();

}
