use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
 
fn main() -> std::io::Result<()> {
    println!("🌐 TCP Echo Server listening on 127.0.0.1:7878");
 
    let listener = TcpListener::bind("127.0.0.1:7878")?;
 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("📥 Connection from: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
 
    Ok(())
}
 
fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());
 
    for line in reader.lines() {
        match line {
            Ok(msg) => {
                println!("📨 [{}] {}", peer, msg);
                let response = format!("Echo: {}\n", msg);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("❌ Error with {}: {}", peer, e);
                break;
            }
        }
    }
 
    println!("🔌 Connection with {} closed.", peer);
}
