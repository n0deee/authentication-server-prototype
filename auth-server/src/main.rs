use std::net::{TcpListener, TcpStream};

const LISTEN_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let listener = TcpListener::bind(LISTEN_IP_ADDR).expect("Cannot create the TCPListener");

    println!("Server online!");

    for (_i, stream) in listener.incoming().enumerate() {
        println!("--------------------");
        println!("Incoming Connection:");

        let stream = stream.unwrap();
        let peer_addr = stream.peer_addr();

        // Print Informations
        println!(
            " - IP: {}",
            match peer_addr {
                Ok(addr) => addr.ip().to_string(),
                Err(_) => String::from("UNKNOW"),
            }
        );
        println!("--------------------");

        // Handle the connection
        if let Err(_) = peer_addr {
            println!("UNKNOW IP ADDRESS. Possible disconnection. Ignoring...");
            drop(stream);
            continue;
        }

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let peer_ipaddr = stream.peer_addr().unwrap().ip();
    let mut read_buffer = [0; 1024];

    loop {
        match stream.peek(&mut read_buffer) {
            Ok(size) => {
                println!("Data received from {peer_ipaddr}:");
                println!(" - Size: {size}");
                println!("{read_buffer:?}");
            }
            Err(e) => {
                println!("Error from {peer_ipaddr}: ");
                println!(" - Code: {}", e.kind().to_string());
                println!(" - Message: {}", e.to_string());
                println!("Disconnecting...");
                drop(stream);
                break;
            }
        }
    }
}
