mod start_listener;

use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
use crate::start_listener::{start_listener, start_connection_thread}; 
use std::io::{self, Write};

pub type ConnectionId = String;
pub type IdNumber = u32;

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
    pub index_id: Arc<Mutex<HashMap<IdNumber, ConnectionId>>>,
    pub stdout_mutex: Arc<Mutex<()>>,
}

impl Server {
    
    pub fn new() -> Self {
        Server {
            connections: Arc::new(Mutex::new(HashMap::new())),
            index_id: Arc::new(Mutex::new(HashMap::new())),
            stdout_mutex: Arc::new(Mutex::new(())),
        }
    }

    
    pub fn start(&self, address: &str) {
        start_listener(self, address);
    }


    pub fn generate_random_id() -> String {
        
        use rand::Rng;

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        
        let part1: u32 = rng.gen(); 
        let part2: u16 = rng.gen(); 
        let part3: u16 = rng.gen();
        
        format!("{:08x}-{:04x}-{:04x}", part1, part2, part3)
    }    


    pub fn list_connections(&self) {
        
        let connections: std::sync::MutexGuard<'_, HashMap<String, Connection>> = self.connections.lock().unwrap();
        let id_number: std::sync::MutexGuard<'_, HashMap<u32, String>> = self.index_id.lock().unwrap();
        
        for (id, conn) in connections.iter() {

            let index: Option<&u32> = id_number.iter().find(|(_, v)| *v == id).map(|(k, _)| k);

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

            if let Some(index) = index {
                println!("[{}] \x1b[38;5;2mID:\x1b[0m {} | \x1b[38;5;2mIP:\x1b[0m {} | \x1b[37;1mStatus:\x1b[0m {}{}\x1b[0m", 
                    index, id, conn.peer_addr, status_color, status);
            } else {
                println!("\x1b[38;5;2mID:\x1b[0m {} | \x1b[38;5;2mIP:\x1b[0m {} | \x1b[37;1mStatus:\x1b[0m {}{}\x1b[0m", 
                id, conn.peer_addr, status_color, status);
            }
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
