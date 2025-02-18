use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::thread;
use std::sync::Arc;

/// Handles incoming messages from the peer
fn handle_receive(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("\nPeer disconnected.");
                break;
            }
            Ok(size) => {
                let received_msg = String::from_utf8_lossy(&buffer[..size]);
                println!("\nReceived: {}", received_msg);
                print!("> ");
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("Receive error: {}", e);
                break;
            }
        }
    }
}

/// Handles sending messages to the peer
fn handle_send(mut stream: TcpStream) {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        if input.trim() == "exit" {
            println!("Closing connection...");
            break;
        }

        if let Err(e) = stream.write_all(input.as_bytes()) {
            eprintln!("Send error: {}", e);
            break;
        }
    }
}

/// Starts a listening server for incoming peer connections
pub fn start_listener(addr: &str) {
    let listener = TcpListener::bind(addr).expect("Failed to bind port");

    println!("Listening on {} for incoming peers...", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New peer connected!");
                let stream_clone = stream.try_clone().expect("Failed to clone stream");

                // Spawn separate threads for reading and writing
                thread::spawn(move || handle_receive(stream));
                thread::spawn(move || handle_send(stream_clone));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

/// Connects to another peer and enables two-way communication
pub fn connect_to_peer(addr: &str) {
    let stream = TcpStream::connect(addr).expect("Failed to connect to peer");
    println!("Connected to peer: {}", addr);

    let stream_clone = stream.try_clone().expect("Failed to clone stream");

    // Spawn separate threads for reading and writing
    thread::spawn(move || handle_receive(stream));
    thread::spawn(move || handle_send(stream_clone));
}
