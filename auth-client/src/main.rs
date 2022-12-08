use std::{io::Write, net::TcpStream};

const SERVER_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let mut stream =
        TcpStream::connect(SERVER_IP_ADDR).expect("Impossible to connect to the server");

        
    _ = stream.write(&[2 as u8; 16]).unwrap();
    _ = stream.write(&[3 as u8; 16]).unwrap();

    loop {
        stream.peer_addr().unwrap();
    }
}
