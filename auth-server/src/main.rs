use std::{
    io::{BufReader, Read},
    net::{TcpListener},
};

use auth_server::net::Connection;

const LISTEN_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let listener = TcpListener::bind(LISTEN_IP_ADDR).expect("Cannot create the TCPListener");

    println!("Server online!");

    for (i, stream) in listener.incoming().enumerate() {
        println!("--------------------");
        println!("Incoming Connection:");

        let connection = Connection::new(i as u64, stream.unwrap());
        let peer_addr = connection.peer_addr();

        // Print Informations
        println!(
            " - IP: {}",
            match peer_addr {
                Ok(addr) => addr.ip().to_string(),
                Err(_) => String::from("UNKNOW"),
            }
        );
        println!(" - ID: {}", connection.id);
        println!("--------------------");

        // Handle the connection
        if let Err(_) = peer_addr {
            println!("UNKNOW IP ADDRESS. Possible disconnection. Ignoring...");
            drop(connection);
            continue;
        }

        handle_connection(connection);
    }
}

fn handle_connection(connection: Connection) {
    loop {
        let mut read_buffer = [0; 1024];
        let mut reader = BufReader::with_capacity(read_buffer.len(), &*connection);

        match reader.read(&mut read_buffer) {
            Ok(size) => {
                println!("Data received from {}:", connection.to_string());
                println!(" - Size: {size}");
                auth_lib::printing::print_buffer(&read_buffer[0..size]);
                println!();
            }
            Err(e) => {
                println!("Error from {}: ", connection.to_string());
                println!(" - Code: {}", e.kind().to_string());
                println!(" - Message: {}", e.to_string());
                println!("Disconnecting...");
                drop(connection);
                break;
            }
        }
    }
}
