use crate::{Server, Connection};
use std::sync::{Arc, Mutex, mpsc};
use std::net::TcpStream;
use std::io::{self, Read, Write};

fn ip_connections(stream: &TcpStream) -> String {
   
    if let Ok(peer_addr) = stream.peer_addr() {
        return peer_addr.to_string();
    }
    
    "Unknown".to_string()
}

pub fn start_listener(server: &Server, address: &str) {

    let listener: std::net::TcpListener = std::net::TcpListener::bind(address).expect("[\x1b[31;1mX\x1b[0m] Server cannot be started!");

    let connections: Arc<Mutex<std::collections::HashMap<String, Connection>>> = Arc::clone(&server.connections);
    let stdout_mutex: Arc<Mutex<()>> = Arc::clone(&server.stdout_mutex);

    std::thread::spawn(move || {

        for stream in listener.incoming() {
            match stream {
        
                Ok(stream) => {
                
                    let id: String = Server::generate_random_id();
                    let ip: String = ip_connections(&stream);
                    let cloned_stream: TcpStream = stream.try_clone().unwrap();
                    let (tx, rx) = mpsc::channel();

                    let handle: std::thread::JoinHandle<()> = start_connection_thread(cloned_stream, rx, Arc::clone(&stdout_mutex));

                    let connection: Connection = Connection {
                        stream,
                        active: true,
                        suspended: false,
                        handle: Some(handle),
                        sender: Some(tx),
                        peer_addr: ip.clone(),
                    };

                    {
                        let mut connections = connections.lock().unwrap();
                        connections.insert(id.clone(), connection);
                    }

                    {
                        let _lock = stdout_mutex.lock().unwrap();
                        println!("\n\n\x1b[32;1m[+]\x1b[0m [Session] {} \x1b[32mEstablished!\x1b[0m\n", id);
                        io::stdout().flush().unwrap();
                    }

                    {
                        let _lock = stdout_mutex.lock().unwrap();
                        print!("\x1b[38;5;154mKoopa> \x1b[0m");
                        io::stdout().flush().unwrap();
                    }
                }
                Err(e) => {
                    let _lock = stdout_mutex.lock().unwrap();
                    println!("\x1b[31;3mError accepting connection\x1b[0m {}", e);
                    
                    io::stdout().flush().unwrap();
                }
            }
        }
    });
}

pub fn start_connection_thread(mut stream: TcpStream, rx: mpsc::Receiver<String>, stdout_mutex: Arc<Mutex<()>>) -> std::thread::JoinHandle<()> {

    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
    
        loop {
            if let Ok(message) = rx.try_recv() {
                if message == "suspend" {
                    return;
                }
            }

            match stream.read(&mut buffer) {
                
                Ok(0) => break,

                Ok(bytes) => {
                    let output: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes]);
                    {
                        let _lock: std::sync::MutexGuard<'_, ()> = stdout_mutex.lock().unwrap();
                        print!("{}", output);
                        io::stdout().flush().unwrap();
                    }
                }

                Err(_) => break,
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    })
}
