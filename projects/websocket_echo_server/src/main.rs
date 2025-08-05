use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt; 
use futures_util::SinkExt;   

#[tokio::main]
async fn main() {
    println!("🌐 WebSocket Echo Server running on ws://127.0.0.1:9001");

    let listener = TcpListener::bind("127.0.0.1:9001").await.expect("Failed to bind to port");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("🔌 New connection from: {}", addr);

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    let (mut write, mut read) = ws_stream.split();
                    
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("📨 Received text: {}", text);
                                if let Err(e) = write.send(Message::Text(text)).await {
                                    eprintln!("❌ Send error: {}", e);
                                    break;
                                }
                            }
                            Ok(Message::Binary(bin)) => {
                                println!("📨 Received binary ({} bytes)", bin.len());
                                if let Err(e) = write.send(Message::Binary(bin)).await {
                                    eprintln!("❌ Send error: {}", e);
                                    break;
                                }
                            }
                            Ok(Message::Ping(ping)) => {
                                if let Err(e) = write.send(Message::Pong(ping)).await {
                                    eprintln!("❌ Send error: {}", e);
                                    break;
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("🚪 Client disconnected");
                                break;
                            }
                            Ok(_) => {}  // Ignore other message types
                            Err(e) => {
                                eprintln!("❌ Error reading message: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => eprintln!("❌ WebSocket handshake failed: {}", e),
            }
        });
    }
}