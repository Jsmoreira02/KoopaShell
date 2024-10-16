use server_core::Server;
use std::sync::Arc;
use std::io::{self, Write};
use std::net::Shutdown;
use rustyline::Editor;

pub fn connect_to_shell(server: &Server, id: &str) {
    
    let mut connections = server.connections.lock().unwrap();

    if let Some(connection) = connections.get_mut(id) {
        if connection.active && !connection.suspended {
        
            let mut stream_clone_for_commands: std::net::TcpStream = connection.stream.try_clone().unwrap();
            let id_cloned: String = id.to_string();
            let stdout_mutex: Arc<std::sync::Mutex<()>> = Arc::clone(&server.stdout_mutex);
            
            let mut rl: Editor<(), rustyline::history::FileHistory> = Editor::new().unwrap();

            {
                let _lock = stdout_mutex.lock().unwrap();
                println!("[\x1b[32;1m✓\x1b[0m] Connected to {}\n", id_cloned);
                io::stdout().flush().unwrap();
            }
            
            {   
                stream_clone_for_commands.write_all(format!("\n").as_bytes()).unwrap();
                stream_clone_for_commands.flush().unwrap();
            }

            loop {
                
                let readline: Result<String, rustyline::error::ReadlineError> = rl.readline("");

                match readline {
                    Ok(command) => {

                        let _ = rl.add_history_entry(command.as_str());
                        let command = command.trim();
                        
                        if command == "exit" {
                    
                            stream_clone_for_commands.shutdown(Shutdown::Both).unwrap();
                            connection.active = false;

                            {
                                let _lock: std::sync::MutexGuard<'_, ()> = stdout_mutex.lock().unwrap();
                                println!("\x1b[31;3mSession Ended!\x1b[0m");
                                io::stdout().flush().unwrap();
                            }

                            break;
                        } 
                        else if command == "suspend" {    
                            
                            connection.suspended = true;

                            {
                                let _lock: std::sync::MutexGuard<'_, ()> = stdout_mutex.lock().unwrap();
                                println!("\x1b[33;1m[!] Connection Suspended!\x1b[0m");
                                io::stdout().flush().unwrap();
                            }

                            break;
                        }
                        else {
                            
                            if command.is_empty() {
                                stream_clone_for_commands.flush().unwrap();
                            } 

                            if !connection.suspended {
                                stream_clone_for_commands.write_all(format!("{}\n", command).as_bytes()).unwrap();
                                stream_clone_for_commands.flush().unwrap();
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        } 
        else if connection.suspended {
            
            let _lock: std::sync::MutexGuard<'_, ()> = server.stdout_mutex.lock().unwrap();
            println!("\x1b[34;1m[!]\x1b[0m Session {} is Suspended", id);
            io::stdout().flush().unwrap();
        } 
        else {
            
            let _lock: std::sync::MutexGuard<'_, ()> = server.stdout_mutex.lock().unwrap();
            println!("\x1b[31;1m[X]\x1b[0m Session {} is not Active", id);
            io::stdout().flush().unwrap();
        }
    } 
    else {
        
        let _lock: std::sync::MutexGuard<'_, ()> = server.stdout_mutex.lock().unwrap();
        println!("\x1b[31;1m[X]\x1b[0m No Session with ID={} was found!", id);
        io::stdout().flush().unwrap();
    }
}
