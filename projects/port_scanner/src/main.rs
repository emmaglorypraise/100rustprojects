use std::io::{self, Write};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;
 
fn main() {
    println!("🔍 Rust Port Scanner");
 
    let host = input("Enter host (e.g., 127.0.0.1 or google.com): ");
    let start = input("Enter start port: ").parse::<u16>().unwrap_or(1);
    let end = input("Enter end port: ").parse::<u16>().unwrap_or(1024);
 
    println!("🧪 Scanning {} from port {} to {}", host, start, end);
 
    for port in start..=end {
        let address = format!("{}:{}", host, port);
        let timeout = Duration::from_millis(300);
 
        if let Ok(addrs) = address.to_socket_addrs() {
            for addr in addrs {
                if is_port_open(addr, timeout) {
                    println!("✅ Port {} is open", port);
                }
            }
        }
    }
 
    println!("✅ Scan complete.");
}
 
fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}