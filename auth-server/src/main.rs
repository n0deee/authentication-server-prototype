use std::{io::Read, io::Write, net::TcpListener};

use auth_lib::db::{Token, TokenPrupose};
use auth_lib::net::packets::Credentials;
use auth_server::{db::MemoryTokenManager, net::Connection};

const LISTEN_IP_ADDR: &str = "127.0.0.1:1667";

fn main() {
    let listener = TcpListener::bind(LISTEN_IP_ADDR).expect("Cannot create the TCPListener");
    let mut token_manager = MemoryTokenManager::new();

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

        handle_connection(connection, &mut token_manager);
        println!("Connection handled!");
    }
}

fn handle_connection(mut connection: Connection, token_manager: &mut MemoryTokenManager) {
    loop {
        let mut read_buffer = [0; 1024];

        match connection.read(&mut read_buffer) {
            Ok(size) => {
                println!("Data received from {}:", connection.to_string());
                println!(" - Size: {size}");
                auth_lib::printing::println_buffer_with_ascii(&read_buffer[..size]);
                println!();

                // Getting data
                let credentials: Credentials =
                    bincode::deserialize::<Credentials>(&read_buffer).unwrap();

                // Autentication
                // TODO: Refactoration. This thing is terrible to read

                println!("Authenticating...");
                let token = auth(credentials).unwrap();
                let token_insertion_result = token_manager.insert(token);

                if let Ok(token) = token_insertion_result {
                    if let Some(_invalidation_readon) = token.is_invalid() {
                        println!("ERROR: Not Authenticated! Token is invalid!");
                        connection.write(&[0; 1]).unwrap();
                        break;
                    }

                    let decoded_token = bincode::serialize::<Token>(&token).unwrap();
                    connection.write(&decoded_token).unwrap();
                    println!("Authenticated!");
                    break;
                } else {
                    println!("ERROR: Not Authenticated! Token insertion error!");
                    connection.write(&[0; 1]).unwrap();
                    break;
                }
            }
            Err(e) => {
                println!("Error from {}: ", connection.to_string());
                println!(" - Code: {}", e.kind().to_string());
                println!(" - Message: {}", e.to_string());
                println!("Disconnecting...");
                break;
            }
        }
    }
    drop(connection);
}

fn auth(credentials: Credentials) -> Result<Token, ()> {
    use sha2::{Digest, Sha512};

    let mut hasher = Sha512::new();

    hasher.update(credentials.username);
    hasher.update(credentials.password);

    let finalized_hash = hasher.finalize().to_vec();

    let key = base64::encode(finalized_hash);
    let token = Token::new(
        key,
        TokenPrupose::UserAccess(0),
        chrono::Utc::now().timestamp_millis(),
        60_000,
    );

    Ok(token)
}
