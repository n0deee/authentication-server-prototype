use std::net::TcpStream;

const SERVER_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let stream = TcpStream::connect(SERVER_IP_ADDR).expect("Impossible to connect to the server");

    loop {
        stream.peer_addr().unwrap();
    }
}
