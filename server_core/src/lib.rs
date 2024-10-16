mod start_listener;

use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
use crate::start_listener::{start_listener, start_connection_thread}; 
use std::io::{self, Write};

pub type ConnectionId = String;

#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub active: bool,
    pub suspended: bool,
    pub handle: Option<std::thread::JoinHandle<()>>,
    pub sender: Option<mpsc::Sender<String>>,
    pub peer_addr: String,
}

pub struct Server {
    pub connections: Arc<Mutex<HashMap<ConnectionId, Connection>>>,
    pub stdout_mutex: Arc<Mutex<()>>,
}

impl Server {
    
    pub fn new() -> Self {
        Server {
            connections: Arc::new(Mutex::new(HashMap::new())),
            stdout_mutex: Arc::new(Mutex::new(())),
        }
    }

    
    pub fn start(&self, address: &str) {
        start_listener(self, address);
    }


    pub fn generate_random_id() -> String {
        
        use rand::{distributions::Alphanumeric, Rng};
    
        let rng = rand::thread_rng();
    
        let part1: String = rng.clone().sample_iter(&Alphanumeric).take(6).map(char::from).collect();
        let part2: String = rng.clone().sample_iter(&Alphanumeric).take(6).map(char::from).collect();
        let part3: String = rng.clone().sample_iter(&Alphanumeric).take(6).map(char::from).collect();
    
        format!("{}-{}-{}", part1, part2, part3)
    }    


    pub fn list_connections(&self) {
        
        let connections: std::sync::MutexGuard<'_, HashMap<String, Connection>> = self.connections.lock().unwrap();
        
        for (id, conn) in connections.iter() {

            let status = if conn.active {
                if conn.suspended {
                    "Suspended"
                } else {
                    "Active"
                }
            } else {
                "Terminated"
            };

            let status_color = match status {
                "Terminated" => "\x1b[37;41;1m",
                "Active" => "\x1b[37;42;1m",
                "Suspended" => "\x1b[33;1m",
                _ => "\x1b[37;1m",
            };

            println!("\x1b[38;5;2mID:\x1b[0m {} | \x1b[38;5;2mIP:\x1b[0m {} | \x1b[37;1mStatus:\x1b[0m {}{}\x1b[0m", id, conn.peer_addr, status_color, status);
        }
    }


    pub fn kill(&self, id: &ConnectionId) {
        
        let mut connections: std::sync::MutexGuard<'_, HashMap<String, Connection>> = self.connections.lock().unwrap();
    
        if let Some(mut connection) = connections.remove(id) {
            
            connection.active = false;
    
            connection.stream.shutdown(std::net::Shutdown::Both).unwrap_or_else(|e| {
                let _lock: std::sync::MutexGuard<'_, ()> = self.stdout_mutex.lock().unwrap();
                println!("\n\x1b[31;3m[!]\x1b[0m Error shutting down stream: {}\n", e);
            });
    
            if let Some(handle) = connection.handle.take() {
                
                handle.join().unwrap_or_else(|_| {
                    let _lock: std::sync::MutexGuard<'_, ()> = self.stdout_mutex.lock().unwrap();
                    println!("\n\x1b[31;3m[!]\x1b[0m Error joining thread for session {}\n", id);
                });
            }
    
            {
                let _lock: std::sync::MutexGuard<'_, ()> = self.stdout_mutex.lock().unwrap();
                println!("\n\x1b[32;1m[-]\x1b[0m Session {} killed\n", id);
                io::stdout().flush().unwrap();
            } 
        } 
        else {

            let _lock: std::sync::MutexGuard<'_, ()> = self.stdout_mutex.lock().unwrap();
            println!("\n\x1b[31;1m[X]\x1b[0m No session with ID={} found\n", id);
            io::stdout().flush().unwrap();
        }
    }
    
    
    pub fn resume_connection(&self, id: &ConnectionId) {
        
        let mut connections = self.connections.lock().unwrap();
        
        if let Some(connection) = connections.get_mut(id) {
            
            if connection.suspended {
        
                println!("\n[\x1b[32;1m+\x1b[0m] Resuming session ID={}.\n", id);
                connection.suspended = false;

                let stream_clone: TcpStream = connection.stream.try_clone().unwrap();
                let (tx, rx) = mpsc::channel();
                let handle = start_connection_thread(
                    stream_clone,
                    rx,
                    Arc::clone(&self.stdout_mutex),
                );
                connection.handle = Some(handle);
                connection.sender = Some(tx);
            } 
            else {
                println!("[X] Session ID={} is not suspended!", id);
            }
        } 
        else {
            println!("[X] No session ID={} was found!", id);
        }
    }
}
