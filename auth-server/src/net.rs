use std::{
    net::TcpStream,
    ops::{Deref, DerefMut},
};

pub struct Connection {
    pub id: u64,
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(id: u64, stream: TcpStream) -> Connection {
        Connection { id, stream }
    }
}

impl Deref for Connection {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}

impl Into<TcpStream> for Connection {
    fn into(self) -> TcpStream {
        self.stream
    }
}

impl ToString for Connection {
    fn to_string(&self) -> String {
        let ip_string = match self.stream.peer_addr() {
            Ok(ip_addr) => ip_addr.ip().to_string(),
            Err(_) => String::from("UNKNOW"),
        };

        format!("{} ({})", ip_string, self.id)
    }
}