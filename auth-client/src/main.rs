use auth_lib::{net::packets, printing::println_buffer};
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
        password: String::from("p4ss w0rd"),
    };

    let serialized_credentials: Vec<u8> = bincode::serialize(&credentials).unwrap();
    send(&mut stream, &serialized_credentials);
    send(&mut stream, &serialized_credentials);

    loop {
        let mut read_buffer = [0; 1024];
        match stream.read(&mut read_buffer) {
            Ok(size) => {
                println!("Data received from Server:");
                println!(" - Size: {size}");
                auth_lib::printing::println_buffer_with_ascii(&read_buffer[..size]);
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

fn send(stream: &mut TcpStream, data: &[u8]) {
    print!("Sending data: ");
    println_buffer(data);
    stream.write(&data).unwrap();
    println!("Sent!");
}

#[cfg(test)]
mod test {
}
