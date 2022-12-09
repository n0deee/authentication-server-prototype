use auth_lib::net::packets;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const SERVER_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let mut stream: TcpStream =
        TcpStream::connect(SERVER_IP_ADDR).expect("Impossible to connect to the server");

    let credentials = packets::Credentials {
        username: String::from("N0de"),
        password: String::from("p4ssw0rd"),
    };

    let serialized_credentials: Vec<u8> = bincode::serialize(&credentials).unwrap();

    stream.write(&serialized_credentials).unwrap();

    loop {
        let mut read_buffer = [0; 1024];
        match stream.read(&mut read_buffer) {
            Ok(size) => {
                println!("Data received from Server:");
                println!(" - Size: {size}");
                auth_lib::printing::print_buffer(&read_buffer[0..size]);
                println!();
            }
            Err(e) => {
                println!("Error from Server: ");
                println!(" - Code: {}", e.kind().to_string());
                println!(" - Message: {}", e.to_string());
                println!("Disconnecting...");
                drop(stream);
                break;
            }
        }
    }
}
