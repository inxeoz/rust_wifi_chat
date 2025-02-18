use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

/// Handles receiving messages from the peer
fn handle_receive(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 1024];

    loop {
        let mut stream = stream.lock().unwrap(); // Lock the stream for reading
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
fn handle_send(stream: Arc<Mutex<TcpStream>>) {
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

        let mut stream = stream.lock().unwrap(); // Lock the stream for writing
        if let Err(e) = stream.write_all(input.as_bytes()) {
            eprintln!("Send error: {}", e);
            break;
        }
    }
}

/// Starts a listener for incoming peer connections
pub fn start_listener(addr: &str) {
    let listener = TcpListener::bind(addr).expect("Failed to bind port");
    println!("Listening on {} for incoming peers...", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New peer connected!");
                let stream = Arc::new(Mutex::new(stream));

                // Spawn separate threads for reading and writing
                let receive_stream = Arc::clone(&stream);
                let send_stream = Arc::clone(&stream);

                thread::spawn(move || handle_receive(receive_stream));
                thread::spawn(move || handle_send(send_stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

/// Connects to another peer and enables two-way communication
pub fn connect_to_peer(addr: &str) {
    let stream = TcpStream::connect(addr).expect("Failed to connect to peer");
    println!("Connected to peer: {}", addr);

    let stream = Arc::new(Mutex::new(stream));

    // Spawn separate threads for reading and writing
    let receive_stream = Arc::clone(&stream);
    let send_stream = Arc::clone(&stream);

    thread::spawn(move || handle_receive(receive_stream));
    thread::spawn(move || handle_send(send_stream));
}
