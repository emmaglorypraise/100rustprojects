use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, error, warn};
use chrono::Utc;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    info!("💬 Multi-Client Chat Server listening on 127.0.0.1:7878");

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr()?.to_string();
                info!("📥 New connection: {}", addr);

                let clients = Arc::clone(&clients);
                clients.lock().unwrap().insert(addr.clone(), stream.try_clone()?);

                // Spawn a new thread for handling each client
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, addr, clients) {
                        error!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                warn!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream, addr: String, clients: Clients) -> std::io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    let timestamp = Utc::now().to_rfc3339(); // Timestamp for messages

    for line in reader.lines() {
        let msg = match line {
            Ok(msg) => msg,
            Err(_) => break, // Client disconnected
        };

        let full_msg = format!("[{}] [{}]: {}", timestamp, addr, msg);
        info!("{}", full_msg); // Log the message

        let mut clients_lock = clients.lock().unwrap();
        let mut disconnected = vec![];

        // Broadcast the message to all other connected clients
        for (peer, mut client_stream) in clients_lock.iter_mut() {
            if peer != &addr {
                if let Err(_) = writeln!(client_stream, "{}", full_msg) {
                    disconnected.push(peer.clone());
                }
            }
        }

        // Remove disconnected clients
        for peer in disconnected {
            clients_lock.remove(&peer);
            warn!("Client {} disconnected.", peer);
        }
    }

    info!("❌ {} disconnected.", addr);
    clients.lock().unwrap().remove(&addr);
    Ok(())
}
